#![allow(deprecated)]
use crate::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements ExplosiveMinecart. Needed for returning it from Java.
pub struct ExplosiveMinecart<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ExplosiveMinecart<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ExplosiveMinecart from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("ExplosiveMinecart") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ExplosiveMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_fuse_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuseTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn fuse_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuseTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn ignite(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ignite", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_ignited(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isIgnited", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn explode(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "explode",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxSpeed", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSlowWhenEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_display_block(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlock",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::bukkit::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::block::data::BlockData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlockOffset", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_velocity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn velocity(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVelocity",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn get_location(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/entity/EntityType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::EntityType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::EntityType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn world(&mut self) -> Result<crate::bukkit::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::World(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPortalCooldown", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::bukkit::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            "()Lorg/bukkit/block/PistonMoveReaction;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::PistonMoveReaction(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::PistonMoveReaction::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let val_1 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            "(FF)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::bukkit::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoundingBox",
            "()Lorg/bukkit/util/BoundingBox;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::BoundingBox(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_location(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::entity::Entity<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_entity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
        arg1: std::option::Option<
            impl Into<&'mc crate::bukkit::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/Location;Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisualFire", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn server(&mut self) -> Result<crate::bukkit::Server<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getServer",
            "()Lorg/bukkit/Server;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Server(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPersistent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    #[deprecated]
    pub fn passenger(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPassenger",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFallDistance", "()F", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.f().unwrap())
    }
    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            "(Lorg/bukkit/event/entity/EntityDamageEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::bukkit::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            "()Lorg/bukkit/event/entity/EntityDamageEvent;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::event::entity::EntityDamageEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTicksLived", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn play_effect(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            "(Lorg/bukkit/EntityEffect;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn swim_sound(&mut self) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInsideVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leaveVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn vehicle(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVehicle",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCustomNameVisible", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisibleByDefault", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInvulnerable", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn pose(&mut self) -> Result<crate::bukkit::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPose",
            "()Lorg/bukkit/entity/Pose;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::Pose(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::Pose::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::bukkit::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            "()Lorg/bukkit/entity/SpawnCategory;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::SpawnCategory(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::SpawnCategory::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spigot(
        &mut self,
    ) -> Result<crate::bukkit::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spigot",
            "()Lorg/bukkit/command/CommandSender$Spigot;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::command::CommandSenderSpigot(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/lang/String;)V",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &mut self,
        arg0: u128,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let upper = (arg0 >> 64) as u64 as i64;
        let lower = arg0 as u64 as i64;
        let val_0 = jni::objects::JValueGen::Object(
            self.jni_ref()
                .new_object("java/util/UUID", "(JJ)V", &[upper.into(), lower.into()])
                .unwrap(),
        );
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/util/UUID;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPermissionSet",
            "(Lorg/bukkit/permissions/Permission;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasPermission",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::bukkit::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        // 3
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(&self.jni_object(),"addAttachment","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;ZI)Lorg/bukkit/permissions/PermissionAttachment;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::permissions::PermissionAttachment(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            "(Lorg/bukkit/permissions/PermissionAttachment;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "recalculatePermissions", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for ExplosiveMinecart<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::entity::Minecart<'mc>> for ExplosiveMinecart<'mc> {
    fn into(self) -> crate::bukkit::entity::Minecart<'mc> {
        crate::bukkit::entity::Minecart::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements HopperMinecart. Needed for returning it from Java.
pub struct HopperMinecart<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> HopperMinecart<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate HopperMinecart from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("HopperMinecart") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HopperMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_enabled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_enabled(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEnabled",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxSpeed", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSlowWhenEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_display_block(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlock",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::bukkit::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::block::data::BlockData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlockOffset", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_velocity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn velocity(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVelocity",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn get_location(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/entity/EntityType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::EntityType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::EntityType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn world(&mut self) -> Result<crate::bukkit::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::World(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPortalCooldown", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::bukkit::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            "()Lorg/bukkit/block/PistonMoveReaction;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::PistonMoveReaction(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::PistonMoveReaction::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let val_1 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            "(FF)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::bukkit::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoundingBox",
            "()Lorg/bukkit/util/BoundingBox;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::BoundingBox(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_location(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::entity::Entity<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_entity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
        arg1: std::option::Option<
            impl Into<&'mc crate::bukkit::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/Location;Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisualFire", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn server(&mut self) -> Result<crate::bukkit::Server<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getServer",
            "()Lorg/bukkit/Server;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Server(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPersistent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    #[deprecated]
    pub fn passenger(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPassenger",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFallDistance", "()F", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.f().unwrap())
    }
    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            "(Lorg/bukkit/event/entity/EntityDamageEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::bukkit::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            "()Lorg/bukkit/event/entity/EntityDamageEvent;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::event::entity::EntityDamageEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTicksLived", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn play_effect(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            "(Lorg/bukkit/EntityEffect;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn swim_sound(&mut self) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInsideVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leaveVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn vehicle(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVehicle",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCustomNameVisible", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisibleByDefault", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInvulnerable", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn pose(&mut self) -> Result<crate::bukkit::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPose",
            "()Lorg/bukkit/entity/Pose;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::Pose(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::Pose::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::bukkit::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            "()Lorg/bukkit/entity/SpawnCategory;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::SpawnCategory(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::SpawnCategory::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spigot(
        &mut self,
    ) -> Result<crate::bukkit::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spigot",
            "()Lorg/bukkit/command/CommandSender$Spigot;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::command::CommandSenderSpigot(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/lang/String;)V",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &mut self,
        arg0: u128,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let upper = (arg0 >> 64) as u64 as i64;
        let lower = arg0 as u64 as i64;
        let val_0 = jni::objects::JValueGen::Object(
            self.jni_ref()
                .new_object("java/util/UUID", "(JJ)V", &[upper.into(), lower.into()])
                .unwrap(),
        );
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/util/UUID;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPermissionSet",
            "(Lorg/bukkit/permissions/Permission;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasPermission",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::bukkit::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        // 3
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(&self.jni_object(),"addAttachment","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;ZI)Lorg/bukkit/permissions/PermissionAttachment;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::permissions::PermissionAttachment(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            "(Lorg/bukkit/permissions/PermissionAttachment;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "recalculatePermissions", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn inventory(
        &mut self,
    ) -> Result<crate::bukkit::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::Inventory(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.j().unwrap())
    }
    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn loot_table(
        &mut self,
    ) -> Result<crate::bukkit::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::loot::LootTable(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for HopperMinecart<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::entity::Minecart<'mc>> for HopperMinecart<'mc> {
    fn into(self) -> crate::bukkit::entity::Minecart<'mc> {
        crate::bukkit::entity::Minecart::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::inventory::InventoryHolder<'mc>> for HopperMinecart<'mc> {
    fn into(self) -> crate::bukkit::inventory::InventoryHolder<'mc> {
        crate::bukkit::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::loot::Lootable<'mc>> for HopperMinecart<'mc> {
    fn into(self) -> crate::bukkit::loot::Lootable<'mc> {
        crate::bukkit::loot::Lootable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements StorageMinecart. Needed for returning it from Java.
pub struct StorageMinecart<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> StorageMinecart<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StorageMinecart from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("StorageMinecart") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StorageMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxSpeed", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSlowWhenEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_display_block(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlock",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::bukkit::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::block::data::BlockData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlockOffset", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_velocity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn velocity(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVelocity",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn get_location(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/entity/EntityType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::EntityType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::EntityType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn world(&mut self) -> Result<crate::bukkit::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::World(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPortalCooldown", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::bukkit::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            "()Lorg/bukkit/block/PistonMoveReaction;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::PistonMoveReaction(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::PistonMoveReaction::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let val_1 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            "(FF)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::bukkit::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoundingBox",
            "()Lorg/bukkit/util/BoundingBox;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::BoundingBox(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_location(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::entity::Entity<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_entity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
        arg1: std::option::Option<
            impl Into<&'mc crate::bukkit::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/Location;Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisualFire", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn server(&mut self) -> Result<crate::bukkit::Server<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getServer",
            "()Lorg/bukkit/Server;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Server(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPersistent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    #[deprecated]
    pub fn passenger(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPassenger",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFallDistance", "()F", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.f().unwrap())
    }
    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            "(Lorg/bukkit/event/entity/EntityDamageEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::bukkit::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            "()Lorg/bukkit/event/entity/EntityDamageEvent;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::event::entity::EntityDamageEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTicksLived", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn play_effect(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            "(Lorg/bukkit/EntityEffect;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn swim_sound(&mut self) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInsideVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leaveVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn vehicle(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVehicle",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCustomNameVisible", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisibleByDefault", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInvulnerable", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn pose(&mut self) -> Result<crate::bukkit::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPose",
            "()Lorg/bukkit/entity/Pose;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::Pose(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::Pose::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::bukkit::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            "()Lorg/bukkit/entity/SpawnCategory;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::SpawnCategory(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::SpawnCategory::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spigot(
        &mut self,
    ) -> Result<crate::bukkit::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spigot",
            "()Lorg/bukkit/command/CommandSender$Spigot;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::command::CommandSenderSpigot(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/lang/String;)V",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &mut self,
        arg0: u128,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let upper = (arg0 >> 64) as u64 as i64;
        let lower = arg0 as u64 as i64;
        let val_0 = jni::objects::JValueGen::Object(
            self.jni_ref()
                .new_object("java/util/UUID", "(JJ)V", &[upper.into(), lower.into()])
                .unwrap(),
        );
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/util/UUID;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPermissionSet",
            "(Lorg/bukkit/permissions/Permission;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasPermission",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::bukkit::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        // 3
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(&self.jni_object(),"addAttachment","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;ZI)Lorg/bukkit/permissions/PermissionAttachment;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::permissions::PermissionAttachment(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            "(Lorg/bukkit/permissions/PermissionAttachment;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "recalculatePermissions", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn inventory(
        &mut self,
    ) -> Result<crate::bukkit::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getInventory",
            "()Lorg/bukkit/inventory/Inventory;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::inventory::Inventory(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.j().unwrap())
    }
    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn loot_table(
        &mut self,
    ) -> Result<crate::bukkit::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::loot::LootTable(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for StorageMinecart<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::entity::Minecart<'mc>> for StorageMinecart<'mc> {
    fn into(self) -> crate::bukkit::entity::Minecart<'mc> {
        crate::bukkit::entity::Minecart::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::inventory::InventoryHolder<'mc>> for StorageMinecart<'mc> {
    fn into(self) -> crate::bukkit::inventory::InventoryHolder<'mc> {
        crate::bukkit::inventory::InventoryHolder::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::bukkit::loot::Lootable<'mc>> for StorageMinecart<'mc> {
    fn into(self) -> crate::bukkit::loot::Lootable<'mc> {
        crate::bukkit::loot::Lootable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements SpawnerMinecart. Needed for returning it from Java.
pub struct SpawnerMinecart<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> SpawnerMinecart<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SpawnerMinecart from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SpawnerMinecart") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnerMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxSpeed", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSlowWhenEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_display_block(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlock",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::bukkit::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::block::data::BlockData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlockOffset", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_velocity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn velocity(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVelocity",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn get_location(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/entity/EntityType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::EntityType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::EntityType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn world(&mut self) -> Result<crate::bukkit::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::World(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPortalCooldown", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::bukkit::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            "()Lorg/bukkit/block/PistonMoveReaction;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::PistonMoveReaction(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::PistonMoveReaction::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let val_1 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            "(FF)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::bukkit::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoundingBox",
            "()Lorg/bukkit/util/BoundingBox;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::BoundingBox(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_location(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::entity::Entity<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_entity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
        arg1: std::option::Option<
            impl Into<&'mc crate::bukkit::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/Location;Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisualFire", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn server(&mut self) -> Result<crate::bukkit::Server<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getServer",
            "()Lorg/bukkit/Server;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Server(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPersistent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    #[deprecated]
    pub fn passenger(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPassenger",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFallDistance", "()F", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.f().unwrap())
    }
    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            "(Lorg/bukkit/event/entity/EntityDamageEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::bukkit::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            "()Lorg/bukkit/event/entity/EntityDamageEvent;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::event::entity::EntityDamageEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTicksLived", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn play_effect(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            "(Lorg/bukkit/EntityEffect;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn swim_sound(&mut self) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInsideVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leaveVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn vehicle(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVehicle",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCustomNameVisible", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisibleByDefault", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInvulnerable", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn pose(&mut self) -> Result<crate::bukkit::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPose",
            "()Lorg/bukkit/entity/Pose;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::Pose(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::Pose::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::bukkit::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            "()Lorg/bukkit/entity/SpawnCategory;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::SpawnCategory(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::SpawnCategory::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spigot(
        &mut self,
    ) -> Result<crate::bukkit::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spigot",
            "()Lorg/bukkit/command/CommandSender$Spigot;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::command::CommandSenderSpigot(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/lang/String;)V",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &mut self,
        arg0: u128,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let upper = (arg0 >> 64) as u64 as i64;
        let lower = arg0 as u64 as i64;
        let val_0 = jni::objects::JValueGen::Object(
            self.jni_ref()
                .new_object("java/util/UUID", "(JJ)V", &[upper.into(), lower.into()])
                .unwrap(),
        );
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/util/UUID;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPermissionSet",
            "(Lorg/bukkit/permissions/Permission;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasPermission",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::bukkit::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        // 3
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(&self.jni_object(),"addAttachment","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;ZI)Lorg/bukkit/permissions/PermissionAttachment;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::permissions::PermissionAttachment(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            "(Lorg/bukkit/permissions/PermissionAttachment;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "recalculatePermissions", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for SpawnerMinecart<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::entity::Minecart<'mc>> for SpawnerMinecart<'mc> {
    fn into(self) -> crate::bukkit::entity::Minecart<'mc> {
        crate::bukkit::entity::Minecart::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements PoweredMinecart. Needed for returning it from Java.
pub struct PoweredMinecart<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PoweredMinecart<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PoweredMinecart from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PoweredMinecart") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PoweredMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_fuel(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuel",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn fuel(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuel", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxSpeed", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSlowWhenEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_display_block(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlock",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::bukkit::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::block::data::BlockData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlockOffset", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_velocity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn velocity(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVelocity",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn get_location(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/entity/EntityType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::EntityType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::EntityType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn world(&mut self) -> Result<crate::bukkit::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::World(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPortalCooldown", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::bukkit::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            "()Lorg/bukkit/block/PistonMoveReaction;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::PistonMoveReaction(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::PistonMoveReaction::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let val_1 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            "(FF)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::bukkit::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoundingBox",
            "()Lorg/bukkit/util/BoundingBox;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::BoundingBox(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_location(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::entity::Entity<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_entity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
        arg1: std::option::Option<
            impl Into<&'mc crate::bukkit::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/Location;Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisualFire", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn server(&mut self) -> Result<crate::bukkit::Server<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getServer",
            "()Lorg/bukkit/Server;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Server(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPersistent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    #[deprecated]
    pub fn passenger(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPassenger",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFallDistance", "()F", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.f().unwrap())
    }
    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            "(Lorg/bukkit/event/entity/EntityDamageEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::bukkit::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            "()Lorg/bukkit/event/entity/EntityDamageEvent;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::event::entity::EntityDamageEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTicksLived", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn play_effect(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            "(Lorg/bukkit/EntityEffect;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn swim_sound(&mut self) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInsideVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leaveVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn vehicle(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVehicle",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCustomNameVisible", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisibleByDefault", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInvulnerable", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn pose(&mut self) -> Result<crate::bukkit::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPose",
            "()Lorg/bukkit/entity/Pose;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::Pose(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::Pose::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::bukkit::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            "()Lorg/bukkit/entity/SpawnCategory;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::SpawnCategory(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::SpawnCategory::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spigot(
        &mut self,
    ) -> Result<crate::bukkit::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spigot",
            "()Lorg/bukkit/command/CommandSender$Spigot;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::command::CommandSenderSpigot(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/lang/String;)V",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &mut self,
        arg0: u128,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let upper = (arg0 >> 64) as u64 as i64;
        let lower = arg0 as u64 as i64;
        let val_0 = jni::objects::JValueGen::Object(
            self.jni_ref()
                .new_object("java/util/UUID", "(JJ)V", &[upper.into(), lower.into()])
                .unwrap(),
        );
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/util/UUID;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPermissionSet",
            "(Lorg/bukkit/permissions/Permission;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasPermission",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::bukkit::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        // 3
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(&self.jni_object(),"addAttachment","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;ZI)Lorg/bukkit/permissions/PermissionAttachment;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::permissions::PermissionAttachment(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            "(Lorg/bukkit/permissions/PermissionAttachment;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "recalculatePermissions", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for PoweredMinecart<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::entity::Minecart<'mc>> for PoweredMinecart<'mc> {
    fn into(self) -> crate::bukkit::entity::Minecart<'mc> {
        crate::bukkit::entity::Minecart::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements RideableMinecart. Needed for returning it from Java.
pub struct RideableMinecart<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> RideableMinecart<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate RideableMinecart from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("RideableMinecart") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RideableMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxSpeed", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSlowWhenEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_display_block(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlock",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::bukkit::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::block::data::BlockData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlockOffset", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_velocity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn velocity(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVelocity",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn get_location(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/entity/EntityType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::EntityType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::EntityType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn world(&mut self) -> Result<crate::bukkit::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::World(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPortalCooldown", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::bukkit::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            "()Lorg/bukkit/block/PistonMoveReaction;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::PistonMoveReaction(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::PistonMoveReaction::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let val_1 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            "(FF)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::bukkit::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoundingBox",
            "()Lorg/bukkit/util/BoundingBox;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::BoundingBox(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_location(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::entity::Entity<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_entity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
        arg1: std::option::Option<
            impl Into<&'mc crate::bukkit::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/Location;Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisualFire", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn server(&mut self) -> Result<crate::bukkit::Server<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getServer",
            "()Lorg/bukkit/Server;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Server(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPersistent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    #[deprecated]
    pub fn passenger(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPassenger",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFallDistance", "()F", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.f().unwrap())
    }
    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            "(Lorg/bukkit/event/entity/EntityDamageEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::bukkit::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            "()Lorg/bukkit/event/entity/EntityDamageEvent;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::event::entity::EntityDamageEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTicksLived", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn play_effect(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            "(Lorg/bukkit/EntityEffect;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn swim_sound(&mut self) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInsideVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leaveVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn vehicle(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVehicle",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCustomNameVisible", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisibleByDefault", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInvulnerable", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn pose(&mut self) -> Result<crate::bukkit::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPose",
            "()Lorg/bukkit/entity/Pose;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::Pose(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::Pose::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::bukkit::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            "()Lorg/bukkit/entity/SpawnCategory;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::SpawnCategory(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::SpawnCategory::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spigot(
        &mut self,
    ) -> Result<crate::bukkit::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spigot",
            "()Lorg/bukkit/command/CommandSender$Spigot;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::command::CommandSenderSpigot(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/lang/String;)V",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &mut self,
        arg0: u128,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let upper = (arg0 >> 64) as u64 as i64;
        let lower = arg0 as u64 as i64;
        let val_0 = jni::objects::JValueGen::Object(
            self.jni_ref()
                .new_object("java/util/UUID", "(JJ)V", &[upper.into(), lower.into()])
                .unwrap(),
        );
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/util/UUID;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPermissionSet",
            "(Lorg/bukkit/permissions/Permission;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasPermission",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::bukkit::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        // 3
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(&self.jni_object(),"addAttachment","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;ZI)Lorg/bukkit/permissions/PermissionAttachment;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::permissions::PermissionAttachment(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            "(Lorg/bukkit/permissions/PermissionAttachment;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "recalculatePermissions", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for RideableMinecart<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::entity::Minecart<'mc>> for RideableMinecart<'mc> {
    fn into(self) -> crate::bukkit::entity::Minecart<'mc> {
        crate::bukkit::entity::Minecart::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// An instantiatable struct that implements CommandMinecart. Needed for returning it from Java.
pub struct CommandMinecart<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> CommandMinecart<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CommandMinecart from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("CommandMinecart") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CommandMinecart object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn command(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCommand",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_command(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCommand",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_damage(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn damage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn max_speed(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxSpeed", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn set_max_speed(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_slow_when_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSlowWhenEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_slow_when_empty(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn flying_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFlyingVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_flying_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlyingVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn derailed_velocity_mod(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDerailedVelocityMod",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_derailed_velocity_mod(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDerailedVelocityMod",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_display_block(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlock",
            "(Lorg/bukkit/material/MaterialData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block(
        &mut self,
    ) -> Result<crate::bukkit::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlock",
            "()Lorg/bukkit/material/MaterialData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::material::MaterialData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_data(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockData",
            "(Lorg/bukkit/block/data/BlockData;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_data(
        &mut self,
    ) -> Result<crate::bukkit::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDisplayBlockData",
            "()Lorg/bukkit/block/data/BlockData;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::block::data::BlockData(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_display_block_offset(
        &mut self,
        arg0: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn display_block_offset(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDisplayBlockOffset", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_velocity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVelocity",
            "(Lorg/bukkit/util/Vector;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn velocity(
        &mut self,
    ) -> Result<crate::bukkit::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVelocity",
            "()Lorg/bukkit/util/Vector;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::Vector(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_valid(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_empty(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn get_location(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
    ) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "(Lorg/bukkit/Location;)Lorg/bukkit/Location;",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/entity/EntityType;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::EntityType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::EntityType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_frozen(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn facing(
        &mut self,
    ) -> Result<crate::bukkit::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFacing",
            "()Lorg/bukkit/block/BlockFace;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::BlockFace(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::BlockFace::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn world(&mut self) -> Result<crate::bukkit::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::World(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_silent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn portal_cooldown(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPortalCooldown", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn piston_move_reaction(
        &mut self,
    ) -> Result<crate::bukkit::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            "()Lorg/bukkit/block/PistonMoveReaction;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::PistonMoveReaction(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::PistonMoveReaction::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn set_rotation(&mut self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let val_1 = jni::objects::JValueGen::Float(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            "(FF)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn height(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn width(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", "()D", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.d().unwrap())
    }
    pub fn bounding_box(
        &mut self,
    ) -> Result<crate::bukkit::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBoundingBox",
            "()Lorg/bukkit/util/BoundingBox;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::util::BoundingBox(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_on_ground(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn is_in_water(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_location(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::entity::Entity<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn teleport_with_entity(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::Location<'mc>>,
        arg1: std::option::Option<
            impl Into<&'mc crate::bukkit::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "teleport",
            "(Lorg/bukkit/Location;Lorg/bukkit/event/player/PlayerTeleportEvent$TeleportCause;)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn entity_id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_fire_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFireTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_fire_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_visual_fire(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisualFire",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visual_fire(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisualFire", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn max_freeze_ticks(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxFreezeTicks", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_freeze_ticks(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_dead(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn server(&mut self) -> Result<crate::bukkit::Server<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getServer",
            "()Lorg/bukkit/Server;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::Server(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn is_persistent(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPersistent", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_persistent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    #[deprecated]
    pub fn passenger(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPassenger",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    #[deprecated]
    pub fn set_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_passenger(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePassenger",
            "(Lorg/bukkit/entity/Entity;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn eject(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn fall_distance(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFallDistance", "()F", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.f().unwrap())
    }
    pub fn set_fall_distance(&mut self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFallDistance",
            "(F)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_last_damage_cause(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            "(Lorg/bukkit/event/entity/EntityDamageEvent;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn last_damage_cause(
        &mut self,
    ) -> Result<crate::bukkit::event::entity::EntityDamageEvent<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            "()Lorg/bukkit/event/entity/EntityDamageEvent;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::event::entity::EntityDamageEvent(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn ticks_lived(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTicksLived", "()I", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.i().unwrap())
    }
    pub fn set_ticks_lived(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTicksLived",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn play_effect(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "playEffect",
            "(Lorg/bukkit/EntityEffect;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn swim_sound(&mut self) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn swim_high_speed_splash_sound(
        &mut self,
    ) -> Result<crate::bukkit::Sound<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            "()Lorg/bukkit/Sound;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::Sound(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::Sound::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn is_inside_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInsideVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn leave_vehicle(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leaveVehicle", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn vehicle(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVehicle",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_custom_name_visible(
        &mut self,
        arg0: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_custom_name_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCustomNameVisible", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_visible_by_default(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_visible_by_default(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisibleByDefault", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_glowing(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGlowing",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_glowing(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_invulnerable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInvulnerable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_invulnerable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInvulnerable", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_silent(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSilent",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_gravity(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_gravity(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setGravity",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn set_portal_cooldown(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPortalCooldown",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn add_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_scoreboard_tag(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn pose(&mut self) -> Result<crate::bukkit::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPose",
            "()Lorg/bukkit/entity/Pose;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::Pose(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::Pose::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spawn_category(
        &mut self,
    ) -> Result<crate::bukkit::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnCategory",
            "()Lorg/bukkit/entity/SpawnCategory;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant =
                self.jni_ref()
                    .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::entity::SpawnCategory(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::entity::SpawnCategory::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn spigot(
        &mut self,
    ) -> Result<crate::bukkit::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spigot",
            "()Lorg/bukkit/command/CommandSender$Spigot;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::command::CommandSenderSpigot(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMetadata",
            "(Ljava/lang/String;Lorg/bukkit/metadata/MetadataValue;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn has_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasMetadata",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn remove_metadata(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeMetadata",
            "(Ljava/lang/String;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn send_message_with_string(
        &mut self,
        arg0: std::option::Option<Vec<impl Into<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/lang/String;)V",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &mut self,
        arg0: u128,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let upper = (arg0 >> 64) as u64 as i64;
        let lower = arg0 as u64 as i64;
        let val_0 = jni::objects::JValueGen::Object(
            self.jni_ref()
                .new_object("java/util/UUID", "(JJ)V", &[upper.into(), lower.into()])
                .unwrap(),
        );
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg1.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "sendMessage",
            "(Ljava/util/UUID;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::bukkit::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPermissionSet",
            "(Lorg/bukkit/permissions/Permission;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasPermission",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::bukkit::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        // 3
        let val_2 = jni::objects::JValueGen::Bool(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(&self.jni_object(),"addAttachment","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;ZI)Lorg/bukkit/permissions/PermissionAttachment;",&[jni::objects::JValueGen::from(&val_0),jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::permissions::PermissionAttachment(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<&'mc crate::bukkit::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            "(Lorg/bukkit/permissions/PermissionAttachment;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "recalculatePermissions", "()V", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", "()Z", &[]);
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(res.z().unwrap())
    }
    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn custom_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCustomName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_custom_name(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomName",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        Ok(())
    }
    pub fn persistent_data_container(
        &mut self,
    ) -> Result<crate::bukkit::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            "()Lorg/bukkit/persistence/PersistentDataContainer;",
            &[],
        );
        let res = crate::java_error_throw(self.jni_ref(), res)?;
        let ret = {
            crate::bukkit::persistence::PersistentDataContainer(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for CommandMinecart<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::bukkit::entity::Minecart<'mc>> for CommandMinecart<'mc> {
    fn into(self) -> crate::bukkit::entity::Minecart<'mc> {
        crate::bukkit::entity::Minecart::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
