#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents a Minecart with TNT inside it that can explode when triggered.
///
/// This is a representation of an abstract class.
pub struct ExplosiveMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ExplosiveMinecart<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ExplosiveMinecart from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/entity/minecart/ExplosiveMinecart")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ExplosiveMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    /// Set the fuse ticks of this minecart. If the fuse ticks are set to a non-zero value, this will ignite the explosive.
    pub fn set_fuse_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuseTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn fuse_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFuseTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn ignite(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ignite", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_ignited(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isIgnited", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// Immediately explode this minecart with the given power.
    pub fn explode(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "explode", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_display_block(
        &mut self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/MaterialData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockOffset",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_velocity(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn velocity(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPortalCooldown",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //@NotNull

    pub fn get_nearby_entities(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNearbyEntities",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::PistonMoveReaction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::PistonMoveReaction::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(FF)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOnGround", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn teleport_with_location(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "teleport", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntityId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxFreezeTicks",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn server(&mut self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Server;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn passenger(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn passengers(
        &mut self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn add_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityDamageEvent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn unique_id(
        &mut self,
    ) -> Result<blackboxmc_java::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn play_effect(
        &mut self,
        arg0: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/EntityEffect;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn swim_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_splash_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn vehicle(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //@Experimental

    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasGravity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn scoreboard_tags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboardTags",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn pose(&mut self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Pose;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::Pose::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::Pose::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/SpawnCategory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::SpawnCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::SpawnCategory::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spigot(
        &mut self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/command/CommandSender$Spigot;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "spigot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSenderSpigot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::EntityType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_uuid(
        &mut self,
        arg0: impl Into<blackboxmc_java::JavaUUID<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<crate::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/permissions/Permission;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "Z";
        // 3
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a.into());
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn effective_permissions(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/PersistentDataContainer;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for ExplosiveMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for ExplosiveMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ExplosiveMinecart into crate::entity::Minecart")
    }
}
/// Represents a Minecart with a Hopper inside it
///
/// This is a representation of an abstract class.
pub struct HopperMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> HopperMinecart<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate HopperMinecart from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/entity/minecart/HopperMinecart")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HopperMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn is_enabled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// Sets whether this Minecart will pick up items.
    pub fn set_enabled(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEnabled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_display_block(
        &mut self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/MaterialData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockOffset",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_velocity(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn velocity(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPortalCooldown",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //@NotNull

    pub fn get_nearby_entities(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNearbyEntities",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::PistonMoveReaction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::PistonMoveReaction::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(FF)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOnGround", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn teleport_with_location(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "teleport", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntityId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxFreezeTicks",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn server(&mut self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Server;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn passenger(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn passengers(
        &mut self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn add_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityDamageEvent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn unique_id(
        &mut self,
    ) -> Result<blackboxmc_java::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn play_effect(
        &mut self,
        arg0: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/EntityEffect;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn swim_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_splash_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn vehicle(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //@Experimental

    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasGravity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn scoreboard_tags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboardTags",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn pose(&mut self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Pose;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::Pose::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::Pose::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/SpawnCategory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::SpawnCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::SpawnCategory::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spigot(
        &mut self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/command/CommandSender$Spigot;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "spigot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSenderSpigot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::EntityType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_uuid(
        &mut self,
        arg0: impl Into<blackboxmc_java::JavaUUID<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<crate::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/permissions/Permission;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "Z";
        // 3
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a.into());
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn effective_permissions(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/PersistentDataContainer;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/loot/LootTable;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootTable;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootTable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for HopperMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for HopperMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HopperMinecart into crate::entity::Minecart")
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for HopperMinecart<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HopperMinecart into crate::inventory::InventoryHolder")
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for HopperMinecart<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HopperMinecart into crate::loot::Lootable")
    }
}
/// Represents a minecart with a chest. These types of <a href="../Minecart.html" title="interface in org.bukkit.entity"><code>minecarts</code></a> have their own inventory that can be accessed using methods from the <a title="interface in org.bukkit.inventory" href="../../inventory/InventoryHolder.html"><code>InventoryHolder</code></a> interface.
///
/// This is a representation of an abstract class.
pub struct StorageMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> StorageMinecart<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StorageMinecart from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/entity/minecart/StorageMinecart")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StorageMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_display_block(
        &mut self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/MaterialData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockOffset",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_velocity(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn velocity(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPortalCooldown",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //@NotNull

    pub fn get_nearby_entities(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNearbyEntities",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::PistonMoveReaction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::PistonMoveReaction::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(FF)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOnGround", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn teleport_with_location(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "teleport", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntityId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxFreezeTicks",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn server(&mut self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Server;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn passenger(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn passengers(
        &mut self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn add_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityDamageEvent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn unique_id(
        &mut self,
    ) -> Result<blackboxmc_java::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn play_effect(
        &mut self,
        arg0: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/EntityEffect;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn swim_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_splash_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn vehicle(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //@Experimental

    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasGravity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn scoreboard_tags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboardTags",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn pose(&mut self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Pose;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::Pose::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::Pose::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/SpawnCategory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::SpawnCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::SpawnCategory::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spigot(
        &mut self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/command/CommandSender$Spigot;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "spigot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSenderSpigot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::EntityType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_uuid(
        &mut self,
        arg0: impl Into<blackboxmc_java::JavaUUID<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<crate::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/permissions/Permission;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "Z";
        // 3
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a.into());
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn effective_permissions(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/PersistentDataContainer;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn inventory(
        &mut self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/inventory/Inventory;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInventory", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::inventory::Inventory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/loot/LootTable;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootTable;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootTable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for StorageMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for StorageMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StorageMinecart into crate::entity::Minecart")
    }
}
impl<'mc> Into<crate::inventory::InventoryHolder<'mc>> for StorageMinecart<'mc> {
    fn into(self) -> crate::inventory::InventoryHolder<'mc> {
        crate::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StorageMinecart into crate::inventory::InventoryHolder")
    }
}
impl<'mc> Into<crate::loot::Lootable<'mc>> for StorageMinecart<'mc> {
    fn into(self) -> crate::loot::Lootable<'mc> {
        crate::loot::Lootable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StorageMinecart into crate::loot::Lootable")
    }
}
/// Represents a Minecart with an <a href="../../block/CreatureSpawner.html" title="interface in org.bukkit.block"><code>entity spawner</code></a> inside it.
///
/// This is a representation of an abstract class.
pub struct SpawnerMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SpawnerMinecart<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SpawnerMinecart from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/entity/minecart/SpawnerMinecart")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnerMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_display_block(
        &mut self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/MaterialData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockOffset",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_velocity(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn velocity(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPortalCooldown",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //@NotNull

    pub fn get_nearby_entities(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNearbyEntities",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::PistonMoveReaction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::PistonMoveReaction::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(FF)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOnGround", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn teleport_with_location(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "teleport", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntityId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxFreezeTicks",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn server(&mut self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Server;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn passenger(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn passengers(
        &mut self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn add_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityDamageEvent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn unique_id(
        &mut self,
    ) -> Result<blackboxmc_java::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn play_effect(
        &mut self,
        arg0: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/EntityEffect;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn swim_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_splash_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn vehicle(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //@Experimental

    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasGravity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn scoreboard_tags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboardTags",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn pose(&mut self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Pose;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::Pose::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::Pose::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/SpawnCategory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::SpawnCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::SpawnCategory::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spigot(
        &mut self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/command/CommandSender$Spigot;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "spigot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSenderSpigot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::EntityType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_uuid(
        &mut self,
        arg0: impl Into<blackboxmc_java::JavaUUID<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<crate::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/permissions/Permission;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "Z";
        // 3
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a.into());
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn effective_permissions(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/PersistentDataContainer;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for SpawnerMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for SpawnerMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SpawnerMinecart into crate::entity::Minecart")
    }
}
/// Represents a powered minecart. A powered minecart moves on its own when a player deposits <a href="../../Material.html#COAL"><code>fuel</code></a>.
///
/// This is a representation of an abstract class.
pub struct PoweredMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PoweredMinecart<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PoweredMinecart from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/entity/minecart/PoweredMinecart")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PoweredMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    /// Set the number of ticks until the minecart runs out of fuel.
    pub fn set_fuel(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn fuel(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_display_block(
        &mut self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/MaterialData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockOffset",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_velocity(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn velocity(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPortalCooldown",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //@NotNull

    pub fn get_nearby_entities(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNearbyEntities",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::PistonMoveReaction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::PistonMoveReaction::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(FF)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOnGround", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn teleport_with_location(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "teleport", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntityId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxFreezeTicks",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn server(&mut self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Server;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn passenger(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn passengers(
        &mut self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn add_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityDamageEvent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn unique_id(
        &mut self,
    ) -> Result<blackboxmc_java::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn play_effect(
        &mut self,
        arg0: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/EntityEffect;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn swim_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_splash_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn vehicle(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //@Experimental

    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasGravity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn scoreboard_tags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboardTags",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn pose(&mut self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Pose;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::Pose::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::Pose::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/SpawnCategory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::SpawnCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::SpawnCategory::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spigot(
        &mut self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/command/CommandSender$Spigot;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "spigot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSenderSpigot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::EntityType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_uuid(
        &mut self,
        arg0: impl Into<blackboxmc_java::JavaUUID<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<crate::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/permissions/Permission;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "Z";
        // 3
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a.into());
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn effective_permissions(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/PersistentDataContainer;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for PoweredMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for PoweredMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PoweredMinecart into crate::entity::Minecart")
    }
}
/// Represents a minecart that can have certain <a href="../Entity.html" title="interface in org.bukkit.entity"><code>entities</code></a> as passengers. Normal passengers include all <a title="interface in org.bukkit.entity" href="../LivingEntity.html"><code>living entities</code></a> with the exception of <a href="../IronGolem.html" title="interface in org.bukkit.entity"><code>iron golems</code></a>. Non-player entities that meet normal passenger criteria automatically mount these minecarts when close enough.
///
/// This is a representation of an abstract class.
pub struct RideableMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> RideableMinecart<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate RideableMinecart from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/entity/minecart/RideableMinecart")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RideableMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_display_block(
        &mut self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/MaterialData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockOffset",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_velocity(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn velocity(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPortalCooldown",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //@NotNull

    pub fn get_nearby_entities(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNearbyEntities",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::PistonMoveReaction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::PistonMoveReaction::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(FF)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOnGround", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn teleport_with_location(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "teleport", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntityId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxFreezeTicks",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn server(&mut self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Server;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn passenger(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn passengers(
        &mut self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn add_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityDamageEvent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn unique_id(
        &mut self,
    ) -> Result<blackboxmc_java::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn play_effect(
        &mut self,
        arg0: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/EntityEffect;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn swim_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_splash_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn vehicle(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //@Experimental

    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasGravity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn scoreboard_tags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboardTags",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn pose(&mut self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Pose;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::Pose::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::Pose::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/SpawnCategory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::SpawnCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::SpawnCategory::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spigot(
        &mut self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/command/CommandSender$Spigot;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "spigot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSenderSpigot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::EntityType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_uuid(
        &mut self,
        arg0: impl Into<blackboxmc_java::JavaUUID<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<crate::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/permissions/Permission;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "Z";
        // 3
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a.into());
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn effective_permissions(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/PersistentDataContainer;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for RideableMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for RideableMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RideableMinecart into crate::entity::Minecart")
    }
}

///
/// This is a representation of an abstract class.
pub struct CommandMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CommandMinecart<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CommandMinecart from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/entity/minecart/CommandMinecart")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CommandMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn command(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCommand", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_command(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCommand",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_name(&mut self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_display_block(
        &mut self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block(
        &mut self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/material/MaterialData;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlock", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockOffset",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_velocity(
        &mut self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn velocity(&mut self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn facing(&mut self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::BlockFace::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::BlockFace::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPortalCooldown",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //@NotNull

    pub fn get_nearby_entities(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getNearbyEntities",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::PistonMoveReaction::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::PistonMoveReaction::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(FF)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let val_2 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/BoundingBox;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOnGround", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn teleport_with_location(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "teleport", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntityId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxFreezeTicks",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn server(&mut self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Server;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn passenger(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn passengers(
        &mut self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn add_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/event/entity/EntityDamageEvent;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/event/entity/EntityDamageEvent;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::entity::EntityDamageEvent::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn unique_id(
        &mut self,
    ) -> Result<blackboxmc_java::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn play_effect(
        &mut self,
        arg0: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/EntityEffect;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn swim_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_splash_sound(&mut self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Sound;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Sound::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Sound::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn vehicle(&mut self) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVehicle", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //@Experimental

    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasGravity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn scoreboard_tags(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboardTags",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn pose(&mut self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Pose;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::Pose::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::Pose::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/SpawnCategory;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::SpawnCategory::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::SpawnCategory::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn spigot(
        &mut self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/command/CommandSender$Spigot;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "spigot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::command::CommandSenderSpigot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::entity::EntityType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    //

    pub fn has_metadata(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn send_message_with_uuid(
        &mut self,
        arg0: impl Into<blackboxmc_java::JavaUUID<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<crate::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Lorg/bukkit/permissions/Permission;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_1);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: impl Into<String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        args.push(val_2);
        sig += "Z";
        // 3
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a.into());
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn effective_permissions(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/persistence/PersistentDataContainer;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for CommandMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for CommandMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CommandMinecart into crate::entity::Minecart")
    }
}
