#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct CommandMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CommandMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CommandMinecart<'mc> {
    fn from_raw(
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
}

impl<'mc> CommandMinecart<'mc> {
    /// Gets the command that this CommandMinecart will run when activated.
    /// This will never return null.If the CommandMinecart does not have a
    /// command, an empty String will be returned instead.
    pub fn command(&self) -> Result<String, Box<dyn std::error::Error>> {
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
    /// Sets the command that this CommandMinecart will run when activated.
    /// Setting the command to null is the same as setting it to an empty
    /// String.
    pub fn set_command(
        &self,
        command: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(command.into())?,
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
    /// Sets the name of this CommandMinecart.The name is used with commands
    /// that this CommandMinecart executes.Setting the name to null is the
    /// same as setting it to "@".
    pub fn set_name(&self, name: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(name.into())?,
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
    /// Sets a minecart's damage.
    pub fn set_damage(&self, damage: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(damage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a minecart's damage.
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum speed of a minecart. The speed is unrelated to the
    /// velocity.
    pub fn max_speed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Sets the maximum speed of a minecart. Must be nonnegative. Default is
    /// 0.4D.
    pub fn set_max_speed(&self, speed: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(speed);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn is_slow_when_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn set_slow_when_empty(&self, slow: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(slow.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn flying_velocity_mod(
        &self,
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
    /// Sets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn set_flying_velocity_mod(
        &self,
        flying: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(flying.into().jni_object().clone())
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
    /// Gets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails.
    ///
    /// A derailed minecart's velocity is multiplied by this factor each tick.
    pub fn derailed_velocity_mod(
        &self,
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
    /// Sets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails. A derailed minecart's velocity is multiplied
    /// by this factor each tick.
    pub fn set_derailed_velocity_mod(
        &self,
        derailed: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(derailed.into().jni_object().clone())
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block(
        &self,
        material: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(material.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block(
        &self,
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block_data(
        &self,
        block_data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_data.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block_data(
        &self,
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
    /// Sets the offset of the display block.
    pub fn set_display_block_offset(&self, offset: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(offset);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the offset of the display block.
    pub fn display_block_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
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
    /// Gets the vehicle's velocity.
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the vehicle's velocity in meters per tick.
    pub fn set_velocity(
        &self,
        vel: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(vel.into().jni_object().clone())
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
    /// Stores the entity's current position in the provided Location object.
    ///
    /// If the provided Location is null this method does nothing and returns
    /// null.
    pub fn get_location(
        &self,
        loc: impl Into<crate::Location<'mc>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(loc.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lorg/bukkit/Location;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLocation", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Gets the entity's height
    pub fn height(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's width
    pub fn width(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's current bounding box.
    ///
    /// The returned bounding box reflects the entity's current location and
    /// size.
    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/util/BoundingBox;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity is supported by a block. This value is a
    /// state updated by the server and is not recalculated unless the entity
    /// moves.
    pub fn is_on_ground(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns true if the entity is in water.
    pub fn is_in_water(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the current world this entity resides in
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/World;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the entity's rotation.
    ///
    /// Note that if the entity is affected by AI, it may override this rotation.
    pub fn set_rotation(&self, yaw: f32, pitch: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(yaw);
        args.push(val_1);
        sig += "F";
        let val_2 = jni::objects::JValueGen::Float(pitch);
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setRotation", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Teleports this entity to the given location. If this entity is riding a
    /// vehicle, it will be dismounted prior to teleportation.
    pub fn teleport(
        &self,
        location: impl Into<crate::Location<'mc>>,
        cause: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = cause {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;";
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
    /// Returns a list of entities within a bounding box centered around this
    /// entity
    pub fn get_nearby_entities(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(x);
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(y);
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(z);
        args.push(val_3);
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNearbyEntities", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Returns a unique id for this entity
    pub fn entity_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum fire ticks.
    pub fn max_fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn set_fire_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFireTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets if the entity has visual fire (it will always appear to be on fire).
    pub fn set_visual_fire(&self, fire: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(fire.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setVisualFire", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity has visual fire (it will always appear to be on fire).
    pub fn is_visual_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum freeze ticks (amount of ticks before it will
    /// be fully frozen)
    pub fn max_freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn set_freeze_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFreezeTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity is fully frozen (it has been in powdered snow for max
    /// freeze ticks).
    pub fn is_frozen(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Mark the entity's removal.
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if this entity has been marked for removal.
    pub fn is_dead(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns false if the entity has died, been despawned for some other
    /// reason, or has not been added to the world.
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the {@link Server} that contains this Entity
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Server;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity gets persisted.
    ///
    /// By default all entities are persistent. An entity will also not get
    /// persisted, if it is riding an entity that is not persistent.
    ///
    /// The persistent flag on players controls whether or not to save their
    /// playerdata file when they quit. If a player is directly or indirectly
    /// riding a non-persistent entity, the vehicle at the root and all its
    /// passengers won't get persisted.
    ///
    /// <b>This should not be confused with
    /// {@link LivingEntity#setRemoveWhenFarAway(boolean)} which controls
    /// despawning of living entities. </b>
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not the entity gets persisted.
    pub fn set_persistent(&self, persistent: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(persistent.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPersistent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Gets the primary passenger of a vehicle. For vehicles that could have multiple passengers, this will only return the primary passenger.
    pub fn passenger(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    #[deprecated]
    /// Set the passenger of a vehicle.
    pub fn set_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets a list of passengers of this vehicle.
    ///
    /// The returned list will not be directly linked to the entity's current
    /// passengers, and no guarantees are made as to its mutability.
    pub fn passengers(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Add a passenger to the vehicle.
    pub fn add_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Remove a passenger from the vehicle.
    pub fn remove_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removePassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check if a vehicle has passengers.
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Eject any passenger.
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the distance this entity has fallen
    pub fn fall_distance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")F";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the fall distance for this entity
    pub fn set_fall_distance(&self, distance: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(distance);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFallDistance", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Record the last {@link EntityDamageEvent} inflicted on this entity
    pub fn set_last_damage_cause(
        &self,
        event: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(event.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Retrieve the last {@link EntityDamageEvent} inflicted on this entity.
    /// This event may have been cancelled.
    pub fn last_damage_cause(
        &self,
    ) -> Result<Option<crate::event::entity::EntityDamageEvent<'mc>>, Box<dyn std::error::Error>>
    {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/event/entity/EntityDamageEvent;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::event::entity::EntityDamageEvent::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Returns a unique and persistent id for this entity
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/UUID;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities.
    pub fn ticks_lived(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities. May not be less than one
    /// tick.
    pub fn set_ticks_lived(&self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(value);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setTicksLived", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Performs the specified {@link EntityEffect} for this entity.
    ///
    /// This will be viewable to all players near the entity.
    ///
    /// If the effect is not applicable to this class of entity, it will not play.
    pub fn play_effect(
        &self,
        val_type: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/EntityEffect;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "playEffect", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the type of the entity.
    pub fn get_type(&self) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntityType;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntityType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes while swimming.
    pub fn swim_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water. For most
    /// entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_splash_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water at high
    /// speeds. For most entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_high_speed_splash_sound(
        &self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns whether this entity is inside a vehicle.
    pub fn is_inside_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Leave the current vehicle. If the entity is currently in a vehicle (and
    /// is removed from it), true will be returned, otherwise false will be
    /// returned.
    pub fn leave_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the vehicle that this entity is inside. If there is no vehicle,
    /// null will be returned.
    pub fn vehicle(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets whether or not to display the mob's custom name client side. The
    /// name will be displayed above the mob similarly to a player.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn set_custom_name_visible(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not the mob's custom name is displayed client side.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn is_custom_name_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn set_visible_by_default(&self, visible: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(visible.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn is_visible_by_default(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get all players that are currently tracking this entity.
    ///
    /// 'Tracking' means that this entity has been sent to the player and that
    /// they are receiving updates on its state. Note that the client's {@code
    /// 'Entity Distance'} setting does not affect the range at which entities
    /// are tracked.
    pub fn tracked_by(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTrackedBy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets whether the entity has a team colored (default: white) glow.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn set_glowing(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGlowing", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is glowing or not.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn is_glowing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is invulnerable or not.
    ///
    /// When an entity is invulnerable it can only be damaged by players in
    /// creative mode.
    pub fn set_invulnerable(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setInvulnerable", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is invulnerable or not.
    pub fn is_invulnerable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether the entity is silent or not.
    pub fn is_silent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is silent or not.
    ///
    /// When an entity is silent it will not produce any sound.
    pub fn set_silent(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setSilent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether gravity applies to this entity.
    pub fn has_gravity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether gravity applies to this entity.
    pub fn set_gravity(&self, gravity: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(gravity.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGravity", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the period of time (in ticks) before this entity can use a portal.
    pub fn portal_cooldown(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPortalCooldown", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the period of time (in ticks) before this entity can use a portal.
    pub fn set_portal_cooldown(&self, cooldown: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(cooldown);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPortalCooldown", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a set of tags for this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn scoreboard_tags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getScoreboardTags", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Add a tag to this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn add_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addScoreboardTag", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes a given tag from this entity.
    pub fn remove_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the entity when moved by a piston.
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/PistonMoveReaction;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the closest cardinal {@link BlockFace} direction an entity is
    /// currently facing.
    ///
    /// This will not return any non-cardinal directions such as
    /// {@link BlockFace#UP} or {@link BlockFace#DOWN}.
    ///
    /// {@link Hanging} entities will override this call and thus their behavior
    /// may be different.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/BlockFace;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the entity's current pose.
    /// <b>Note that the pose is only updated at the end of a tick, so may be
    /// inconsistent with other methods. eg {@link Player#isSneaking()} being
    /// true does not imply the current pose will be {@link Pose#SNEAKING}</b>
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Pose;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Pose::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the category of spawn to which this entity belongs.
    pub fn spawn_category(
        &self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/SpawnCategory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnCategory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::SpawnCategory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this entity has been spawned in a world.
    ///
    /// Entities not spawned in a world will not tick, be sent to players, or be
    /// saved to the server files.
    pub fn is_in_world(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get this entity as an NBT string.
    ///
    /// This string should not be relied upon as a serializable value.
    pub fn as_string(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsString", sig.as_str(), args);
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
    /// Crates an {@link EntitySnapshot} representing the current state of this entity.
    pub fn create_snapshot(
        &self,
    ) -> Result<Option<crate::entity::EntitySnapshot<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntitySnapshot;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "createSnapshot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntitySnapshot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Creates a copy of this entity and all its data. Spawns the copy at the given location.
    ///
    /// <b>Note:</b> Players cannot be copied.
    pub fn copy(
        &self,
        to: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = to {
            sig += "Lorg/bukkit/Location;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets a metadata value in the implementing object's metadata store.
    pub fn set_metadata(
        &self,
        metadata_key: impl Into<String>,
        new_metadata_value: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/metadata/MetadataValue;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_metadata_value.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a list of previously set metadata values from the implementing
    /// object's metadata store.
    pub fn get_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Tests to see whether the implementing object contains the given
    /// metadata value in its metadata store.
    pub fn has_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes the given metadata value from the implementing object's
    /// metadata store.
    pub fn remove_metadata(
        &self,
        metadata_key: impl Into<String>,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sends this sender multiple messages
    pub fn send_message(
        &self,
        sender: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        messages: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = messages {
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
    /// Gets the name of this command sender
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the custom name on a mob or block. If there is no name this method
    /// will return null.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), args);
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
    /// Sets a custom name on a mob or block. This name will be used in death
    /// messages and can be sent to the client as a nameplate over the mob.
    ///
    /// Setting the name to null or an empty string will clear it.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn set_custom_name(
        &self,
        name: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(name.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setCustomName", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a custom tag container capable of storing tags on the object.
    /// Note that the tags stored on this container are all stored under their
    /// own custom namespace therefore modifying default tags using this
    /// {@link PersistentDataHolder} is impossible.
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/persistence/PersistentDataContainer;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for CommandMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CommandMinecart into crate::entity::Minecart")
    }
}
#[repr(C)]
pub struct SpawnerMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SpawnerMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpawnerMinecart<'mc> {
    fn from_raw(
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
}

impl<'mc> SpawnerMinecart<'mc> {
    /// Sets a minecart's damage.
    pub fn set_damage(&self, damage: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(damage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a minecart's damage.
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum speed of a minecart. The speed is unrelated to the
    /// velocity.
    pub fn max_speed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Sets the maximum speed of a minecart. Must be nonnegative. Default is
    /// 0.4D.
    pub fn set_max_speed(&self, speed: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(speed);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn is_slow_when_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn set_slow_when_empty(&self, slow: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(slow.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn flying_velocity_mod(
        &self,
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
    /// Sets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn set_flying_velocity_mod(
        &self,
        flying: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(flying.into().jni_object().clone())
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
    /// Gets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails.
    ///
    /// A derailed minecart's velocity is multiplied by this factor each tick.
    pub fn derailed_velocity_mod(
        &self,
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
    /// Sets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails. A derailed minecart's velocity is multiplied
    /// by this factor each tick.
    pub fn set_derailed_velocity_mod(
        &self,
        derailed: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(derailed.into().jni_object().clone())
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block(
        &self,
        material: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(material.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block(
        &self,
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block_data(
        &self,
        block_data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_data.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block_data(
        &self,
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
    /// Sets the offset of the display block.
    pub fn set_display_block_offset(&self, offset: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(offset);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the offset of the display block.
    pub fn display_block_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
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
    /// Gets the vehicle's velocity.
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the vehicle's velocity in meters per tick.
    pub fn set_velocity(
        &self,
        vel: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(vel.into().jni_object().clone())
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
    /// Stores the entity's current position in the provided Location object.
    ///
    /// If the provided Location is null this method does nothing and returns
    /// null.
    pub fn get_location(
        &self,
        loc: impl Into<crate::Location<'mc>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(loc.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lorg/bukkit/Location;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLocation", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Gets the entity's height
    pub fn height(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's width
    pub fn width(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's current bounding box.
    ///
    /// The returned bounding box reflects the entity's current location and
    /// size.
    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/util/BoundingBox;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity is supported by a block. This value is a
    /// state updated by the server and is not recalculated unless the entity
    /// moves.
    pub fn is_on_ground(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns true if the entity is in water.
    pub fn is_in_water(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the current world this entity resides in
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/World;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the entity's rotation.
    ///
    /// Note that if the entity is affected by AI, it may override this rotation.
    pub fn set_rotation(&self, yaw: f32, pitch: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(yaw);
        args.push(val_1);
        sig += "F";
        let val_2 = jni::objects::JValueGen::Float(pitch);
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setRotation", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Teleports this entity to the given location. If this entity is riding a
    /// vehicle, it will be dismounted prior to teleportation.
    pub fn teleport(
        &self,
        location: impl Into<crate::Location<'mc>>,
        cause: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = cause {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;";
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
    /// Returns a list of entities within a bounding box centered around this
    /// entity
    pub fn get_nearby_entities(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(x);
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(y);
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(z);
        args.push(val_3);
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNearbyEntities", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Returns a unique id for this entity
    pub fn entity_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum fire ticks.
    pub fn max_fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn set_fire_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFireTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets if the entity has visual fire (it will always appear to be on fire).
    pub fn set_visual_fire(&self, fire: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(fire.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setVisualFire", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity has visual fire (it will always appear to be on fire).
    pub fn is_visual_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum freeze ticks (amount of ticks before it will
    /// be fully frozen)
    pub fn max_freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn set_freeze_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFreezeTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity is fully frozen (it has been in powdered snow for max
    /// freeze ticks).
    pub fn is_frozen(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Mark the entity's removal.
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if this entity has been marked for removal.
    pub fn is_dead(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns false if the entity has died, been despawned for some other
    /// reason, or has not been added to the world.
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the {@link Server} that contains this Entity
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Server;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity gets persisted.
    ///
    /// By default all entities are persistent. An entity will also not get
    /// persisted, if it is riding an entity that is not persistent.
    ///
    /// The persistent flag on players controls whether or not to save their
    /// playerdata file when they quit. If a player is directly or indirectly
    /// riding a non-persistent entity, the vehicle at the root and all its
    /// passengers won't get persisted.
    ///
    /// <b>This should not be confused with
    /// {@link LivingEntity#setRemoveWhenFarAway(boolean)} which controls
    /// despawning of living entities. </b>
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not the entity gets persisted.
    pub fn set_persistent(&self, persistent: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(persistent.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPersistent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Gets the primary passenger of a vehicle. For vehicles that could have multiple passengers, this will only return the primary passenger.
    pub fn passenger(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    #[deprecated]
    /// Set the passenger of a vehicle.
    pub fn set_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets a list of passengers of this vehicle.
    ///
    /// The returned list will not be directly linked to the entity's current
    /// passengers, and no guarantees are made as to its mutability.
    pub fn passengers(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Add a passenger to the vehicle.
    pub fn add_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Remove a passenger from the vehicle.
    pub fn remove_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removePassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check if a vehicle has passengers.
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Eject any passenger.
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the distance this entity has fallen
    pub fn fall_distance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")F";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the fall distance for this entity
    pub fn set_fall_distance(&self, distance: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(distance);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFallDistance", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Record the last {@link EntityDamageEvent} inflicted on this entity
    pub fn set_last_damage_cause(
        &self,
        event: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(event.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Retrieve the last {@link EntityDamageEvent} inflicted on this entity.
    /// This event may have been cancelled.
    pub fn last_damage_cause(
        &self,
    ) -> Result<Option<crate::event::entity::EntityDamageEvent<'mc>>, Box<dyn std::error::Error>>
    {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/event/entity/EntityDamageEvent;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::event::entity::EntityDamageEvent::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Returns a unique and persistent id for this entity
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/UUID;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities.
    pub fn ticks_lived(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities. May not be less than one
    /// tick.
    pub fn set_ticks_lived(&self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(value);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setTicksLived", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Performs the specified {@link EntityEffect} for this entity.
    ///
    /// This will be viewable to all players near the entity.
    ///
    /// If the effect is not applicable to this class of entity, it will not play.
    pub fn play_effect(
        &self,
        val_type: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/EntityEffect;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "playEffect", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the type of the entity.
    pub fn get_type(&self) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntityType;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntityType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes while swimming.
    pub fn swim_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water. For most
    /// entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_splash_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water at high
    /// speeds. For most entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_high_speed_splash_sound(
        &self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns whether this entity is inside a vehicle.
    pub fn is_inside_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Leave the current vehicle. If the entity is currently in a vehicle (and
    /// is removed from it), true will be returned, otherwise false will be
    /// returned.
    pub fn leave_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the vehicle that this entity is inside. If there is no vehicle,
    /// null will be returned.
    pub fn vehicle(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets whether or not to display the mob's custom name client side. The
    /// name will be displayed above the mob similarly to a player.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn set_custom_name_visible(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not the mob's custom name is displayed client side.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn is_custom_name_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn set_visible_by_default(&self, visible: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(visible.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn is_visible_by_default(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get all players that are currently tracking this entity.
    ///
    /// 'Tracking' means that this entity has been sent to the player and that
    /// they are receiving updates on its state. Note that the client's {@code
    /// 'Entity Distance'} setting does not affect the range at which entities
    /// are tracked.
    pub fn tracked_by(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTrackedBy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets whether the entity has a team colored (default: white) glow.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn set_glowing(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGlowing", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is glowing or not.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn is_glowing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is invulnerable or not.
    ///
    /// When an entity is invulnerable it can only be damaged by players in
    /// creative mode.
    pub fn set_invulnerable(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setInvulnerable", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is invulnerable or not.
    pub fn is_invulnerable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether the entity is silent or not.
    pub fn is_silent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is silent or not.
    ///
    /// When an entity is silent it will not produce any sound.
    pub fn set_silent(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setSilent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether gravity applies to this entity.
    pub fn has_gravity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether gravity applies to this entity.
    pub fn set_gravity(&self, gravity: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(gravity.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGravity", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the period of time (in ticks) before this entity can use a portal.
    pub fn portal_cooldown(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPortalCooldown", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the period of time (in ticks) before this entity can use a portal.
    pub fn set_portal_cooldown(&self, cooldown: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(cooldown);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPortalCooldown", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a set of tags for this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn scoreboard_tags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getScoreboardTags", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Add a tag to this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn add_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addScoreboardTag", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes a given tag from this entity.
    pub fn remove_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the entity when moved by a piston.
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/PistonMoveReaction;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the closest cardinal {@link BlockFace} direction an entity is
    /// currently facing.
    ///
    /// This will not return any non-cardinal directions such as
    /// {@link BlockFace#UP} or {@link BlockFace#DOWN}.
    ///
    /// {@link Hanging} entities will override this call and thus their behavior
    /// may be different.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/BlockFace;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the entity's current pose.
    /// <b>Note that the pose is only updated at the end of a tick, so may be
    /// inconsistent with other methods. eg {@link Player#isSneaking()} being
    /// true does not imply the current pose will be {@link Pose#SNEAKING}</b>
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Pose;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Pose::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the category of spawn to which this entity belongs.
    pub fn spawn_category(
        &self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/SpawnCategory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnCategory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::SpawnCategory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this entity has been spawned in a world.
    ///
    /// Entities not spawned in a world will not tick, be sent to players, or be
    /// saved to the server files.
    pub fn is_in_world(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get this entity as an NBT string.
    ///
    /// This string should not be relied upon as a serializable value.
    pub fn as_string(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsString", sig.as_str(), args);
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
    /// Crates an {@link EntitySnapshot} representing the current state of this entity.
    pub fn create_snapshot(
        &self,
    ) -> Result<Option<crate::entity::EntitySnapshot<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntitySnapshot;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "createSnapshot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntitySnapshot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Creates a copy of this entity and all its data. Spawns the copy at the given location.
    ///
    /// <b>Note:</b> Players cannot be copied.
    pub fn copy(
        &self,
        to: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = to {
            sig += "Lorg/bukkit/Location;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets a metadata value in the implementing object's metadata store.
    pub fn set_metadata(
        &self,
        metadata_key: impl Into<String>,
        new_metadata_value: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/metadata/MetadataValue;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_metadata_value.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a list of previously set metadata values from the implementing
    /// object's metadata store.
    pub fn get_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Tests to see whether the implementing object contains the given
    /// metadata value in its metadata store.
    pub fn has_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes the given metadata value from the implementing object's
    /// metadata store.
    pub fn remove_metadata(
        &self,
        metadata_key: impl Into<String>,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sends this sender multiple messages
    pub fn send_message(
        &self,
        sender: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        messages: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = messages {
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
    /// Gets the name of this command sender
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the custom name on a mob or block. If there is no name this method
    /// will return null.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), args);
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
    /// Sets a custom name on a mob or block. This name will be used in death
    /// messages and can be sent to the client as a nameplate over the mob.
    ///
    /// Setting the name to null or an empty string will clear it.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn set_custom_name(
        &self,
        name: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(name.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setCustomName", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a custom tag container capable of storing tags on the object.
    /// Note that the tags stored on this container are all stored under their
    /// own custom namespace therefore modifying default tags using this
    /// {@link PersistentDataHolder} is impossible.
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/persistence/PersistentDataContainer;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for SpawnerMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SpawnerMinecart into crate::entity::Minecart")
    }
}
#[repr(C)]
pub struct StorageMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StorageMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StorageMinecart<'mc> {
    fn from_raw(
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
}

impl<'mc> StorageMinecart<'mc> {
    /// Sets a minecart's damage.
    pub fn set_damage(&self, damage: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(damage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a minecart's damage.
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum speed of a minecart. The speed is unrelated to the
    /// velocity.
    pub fn max_speed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Sets the maximum speed of a minecart. Must be nonnegative. Default is
    /// 0.4D.
    pub fn set_max_speed(&self, speed: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(speed);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn is_slow_when_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn set_slow_when_empty(&self, slow: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(slow.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn flying_velocity_mod(
        &self,
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
    /// Sets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn set_flying_velocity_mod(
        &self,
        flying: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(flying.into().jni_object().clone())
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
    /// Gets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails.
    ///
    /// A derailed minecart's velocity is multiplied by this factor each tick.
    pub fn derailed_velocity_mod(
        &self,
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
    /// Sets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails. A derailed minecart's velocity is multiplied
    /// by this factor each tick.
    pub fn set_derailed_velocity_mod(
        &self,
        derailed: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(derailed.into().jni_object().clone())
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block(
        &self,
        material: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(material.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block(
        &self,
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block_data(
        &self,
        block_data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_data.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block_data(
        &self,
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
    /// Sets the offset of the display block.
    pub fn set_display_block_offset(&self, offset: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(offset);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the offset of the display block.
    pub fn display_block_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
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
    /// Gets the vehicle's velocity.
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the vehicle's velocity in meters per tick.
    pub fn set_velocity(
        &self,
        vel: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(vel.into().jni_object().clone())
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
    /// Stores the entity's current position in the provided Location object.
    ///
    /// If the provided Location is null this method does nothing and returns
    /// null.
    pub fn get_location(
        &self,
        loc: impl Into<crate::Location<'mc>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(loc.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lorg/bukkit/Location;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLocation", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Gets the entity's height
    pub fn height(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's width
    pub fn width(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's current bounding box.
    ///
    /// The returned bounding box reflects the entity's current location and
    /// size.
    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/util/BoundingBox;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity is supported by a block. This value is a
    /// state updated by the server and is not recalculated unless the entity
    /// moves.
    pub fn is_on_ground(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns true if the entity is in water.
    pub fn is_in_water(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the current world this entity resides in
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/World;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the entity's rotation.
    ///
    /// Note that if the entity is affected by AI, it may override this rotation.
    pub fn set_rotation(&self, yaw: f32, pitch: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(yaw);
        args.push(val_1);
        sig += "F";
        let val_2 = jni::objects::JValueGen::Float(pitch);
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setRotation", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Teleports this entity to the given location. If this entity is riding a
    /// vehicle, it will be dismounted prior to teleportation.
    pub fn teleport(
        &self,
        location: impl Into<crate::Location<'mc>>,
        cause: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = cause {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;";
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
    /// Returns a list of entities within a bounding box centered around this
    /// entity
    pub fn get_nearby_entities(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(x);
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(y);
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(z);
        args.push(val_3);
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNearbyEntities", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Returns a unique id for this entity
    pub fn entity_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum fire ticks.
    pub fn max_fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn set_fire_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFireTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets if the entity has visual fire (it will always appear to be on fire).
    pub fn set_visual_fire(&self, fire: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(fire.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setVisualFire", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity has visual fire (it will always appear to be on fire).
    pub fn is_visual_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum freeze ticks (amount of ticks before it will
    /// be fully frozen)
    pub fn max_freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn set_freeze_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFreezeTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity is fully frozen (it has been in powdered snow for max
    /// freeze ticks).
    pub fn is_frozen(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Mark the entity's removal.
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if this entity has been marked for removal.
    pub fn is_dead(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns false if the entity has died, been despawned for some other
    /// reason, or has not been added to the world.
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the {@link Server} that contains this Entity
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Server;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity gets persisted.
    ///
    /// By default all entities are persistent. An entity will also not get
    /// persisted, if it is riding an entity that is not persistent.
    ///
    /// The persistent flag on players controls whether or not to save their
    /// playerdata file when they quit. If a player is directly or indirectly
    /// riding a non-persistent entity, the vehicle at the root and all its
    /// passengers won't get persisted.
    ///
    /// <b>This should not be confused with
    /// {@link LivingEntity#setRemoveWhenFarAway(boolean)} which controls
    /// despawning of living entities. </b>
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not the entity gets persisted.
    pub fn set_persistent(&self, persistent: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(persistent.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPersistent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Gets the primary passenger of a vehicle. For vehicles that could have multiple passengers, this will only return the primary passenger.
    pub fn passenger(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    #[deprecated]
    /// Set the passenger of a vehicle.
    pub fn set_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets a list of passengers of this vehicle.
    ///
    /// The returned list will not be directly linked to the entity's current
    /// passengers, and no guarantees are made as to its mutability.
    pub fn passengers(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Add a passenger to the vehicle.
    pub fn add_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Remove a passenger from the vehicle.
    pub fn remove_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removePassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check if a vehicle has passengers.
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Eject any passenger.
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the distance this entity has fallen
    pub fn fall_distance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")F";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the fall distance for this entity
    pub fn set_fall_distance(&self, distance: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(distance);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFallDistance", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Record the last {@link EntityDamageEvent} inflicted on this entity
    pub fn set_last_damage_cause(
        &self,
        event: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(event.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Retrieve the last {@link EntityDamageEvent} inflicted on this entity.
    /// This event may have been cancelled.
    pub fn last_damage_cause(
        &self,
    ) -> Result<Option<crate::event::entity::EntityDamageEvent<'mc>>, Box<dyn std::error::Error>>
    {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/event/entity/EntityDamageEvent;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::event::entity::EntityDamageEvent::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Returns a unique and persistent id for this entity
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/UUID;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities.
    pub fn ticks_lived(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities. May not be less than one
    /// tick.
    pub fn set_ticks_lived(&self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(value);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setTicksLived", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Performs the specified {@link EntityEffect} for this entity.
    ///
    /// This will be viewable to all players near the entity.
    ///
    /// If the effect is not applicable to this class of entity, it will not play.
    pub fn play_effect(
        &self,
        val_type: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/EntityEffect;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "playEffect", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the type of the entity.
    pub fn get_type(&self) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntityType;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntityType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes while swimming.
    pub fn swim_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water. For most
    /// entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_splash_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water at high
    /// speeds. For most entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_high_speed_splash_sound(
        &self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns whether this entity is inside a vehicle.
    pub fn is_inside_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Leave the current vehicle. If the entity is currently in a vehicle (and
    /// is removed from it), true will be returned, otherwise false will be
    /// returned.
    pub fn leave_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the vehicle that this entity is inside. If there is no vehicle,
    /// null will be returned.
    pub fn vehicle(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets whether or not to display the mob's custom name client side. The
    /// name will be displayed above the mob similarly to a player.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn set_custom_name_visible(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not the mob's custom name is displayed client side.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn is_custom_name_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn set_visible_by_default(&self, visible: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(visible.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn is_visible_by_default(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get all players that are currently tracking this entity.
    ///
    /// 'Tracking' means that this entity has been sent to the player and that
    /// they are receiving updates on its state. Note that the client's {@code
    /// 'Entity Distance'} setting does not affect the range at which entities
    /// are tracked.
    pub fn tracked_by(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTrackedBy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets whether the entity has a team colored (default: white) glow.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn set_glowing(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGlowing", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is glowing or not.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn is_glowing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is invulnerable or not.
    ///
    /// When an entity is invulnerable it can only be damaged by players in
    /// creative mode.
    pub fn set_invulnerable(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setInvulnerable", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is invulnerable or not.
    pub fn is_invulnerable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether the entity is silent or not.
    pub fn is_silent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is silent or not.
    ///
    /// When an entity is silent it will not produce any sound.
    pub fn set_silent(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setSilent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether gravity applies to this entity.
    pub fn has_gravity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether gravity applies to this entity.
    pub fn set_gravity(&self, gravity: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(gravity.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGravity", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the period of time (in ticks) before this entity can use a portal.
    pub fn portal_cooldown(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPortalCooldown", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the period of time (in ticks) before this entity can use a portal.
    pub fn set_portal_cooldown(&self, cooldown: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(cooldown);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPortalCooldown", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a set of tags for this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn scoreboard_tags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getScoreboardTags", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Add a tag to this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn add_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addScoreboardTag", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes a given tag from this entity.
    pub fn remove_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the entity when moved by a piston.
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/PistonMoveReaction;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the closest cardinal {@link BlockFace} direction an entity is
    /// currently facing.
    ///
    /// This will not return any non-cardinal directions such as
    /// {@link BlockFace#UP} or {@link BlockFace#DOWN}.
    ///
    /// {@link Hanging} entities will override this call and thus their behavior
    /// may be different.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/BlockFace;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the entity's current pose.
    /// <b>Note that the pose is only updated at the end of a tick, so may be
    /// inconsistent with other methods. eg {@link Player#isSneaking()} being
    /// true does not imply the current pose will be {@link Pose#SNEAKING}</b>
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Pose;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Pose::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the category of spawn to which this entity belongs.
    pub fn spawn_category(
        &self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/SpawnCategory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnCategory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::SpawnCategory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this entity has been spawned in a world.
    ///
    /// Entities not spawned in a world will not tick, be sent to players, or be
    /// saved to the server files.
    pub fn is_in_world(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get this entity as an NBT string.
    ///
    /// This string should not be relied upon as a serializable value.
    pub fn as_string(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsString", sig.as_str(), args);
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
    /// Crates an {@link EntitySnapshot} representing the current state of this entity.
    pub fn create_snapshot(
        &self,
    ) -> Result<Option<crate::entity::EntitySnapshot<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntitySnapshot;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "createSnapshot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntitySnapshot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Creates a copy of this entity and all its data. Spawns the copy at the given location.
    ///
    /// <b>Note:</b> Players cannot be copied.
    pub fn copy(
        &self,
        to: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = to {
            sig += "Lorg/bukkit/Location;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets a metadata value in the implementing object's metadata store.
    pub fn set_metadata(
        &self,
        metadata_key: impl Into<String>,
        new_metadata_value: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/metadata/MetadataValue;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_metadata_value.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a list of previously set metadata values from the implementing
    /// object's metadata store.
    pub fn get_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Tests to see whether the implementing object contains the given
    /// metadata value in its metadata store.
    pub fn has_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes the given metadata value from the implementing object's
    /// metadata store.
    pub fn remove_metadata(
        &self,
        metadata_key: impl Into<String>,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sends this sender multiple messages
    pub fn send_message(
        &self,
        sender: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        messages: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = messages {
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
    /// Gets the name of this command sender
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the custom name on a mob or block. If there is no name this method
    /// will return null.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), args);
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
    /// Sets a custom name on a mob or block. This name will be used in death
    /// messages and can be sent to the client as a nameplate over the mob.
    ///
    /// Setting the name to null or an empty string will clear it.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn set_custom_name(
        &self,
        name: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(name.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setCustomName", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a custom tag container capable of storing tags on the object.
    /// Note that the tags stored on this container are all stored under their
    /// own custom namespace therefore modifying default tags using this
    /// {@link PersistentDataHolder} is impossible.
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/persistence/PersistentDataContainer;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the object's inventory.
    pub fn inventory(
        &self,
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
    /// Set the loot table for a container or entity.
    ///
    /// To remove a loot table use null. Do not use {@link LootTables#EMPTY} to
    /// clear a LootTable.
    pub fn set_loot_table(
        &self,
        table: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/loot/LootTable;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(table.into().jni_object().clone())
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
    /// Gets the Loot Table attached to this block or entity.
    ///
    /// If an block/entity does not have a loot table, this will return null, NOT
    /// an empty loot table.
    pub fn loot_table(
        &self,
    ) -> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootTable;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootTable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::loot::LootTable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Set the seed used when this Loot Table generates loot.
    pub fn set_seed(&self, seed: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(seed);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Loot Table's seed.
    ///
    /// The seed is used when generating loot.
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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
#[repr(C)]
pub struct HopperMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HopperMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HopperMinecart<'mc> {
    fn from_raw(
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
}

impl<'mc> HopperMinecart<'mc> {
    /// Checks whether or not this Minecart will pick up
    /// items into its inventory.
    pub fn is_enabled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this Minecart will pick up items.
    pub fn set_enabled(&self, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(enabled.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEnabled",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets a minecart's damage.
    pub fn set_damage(&self, damage: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(damage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a minecart's damage.
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum speed of a minecart. The speed is unrelated to the
    /// velocity.
    pub fn max_speed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Sets the maximum speed of a minecart. Must be nonnegative. Default is
    /// 0.4D.
    pub fn set_max_speed(&self, speed: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(speed);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn is_slow_when_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn set_slow_when_empty(&self, slow: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(slow.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn flying_velocity_mod(
        &self,
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
    /// Sets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn set_flying_velocity_mod(
        &self,
        flying: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(flying.into().jni_object().clone())
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
    /// Gets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails.
    ///
    /// A derailed minecart's velocity is multiplied by this factor each tick.
    pub fn derailed_velocity_mod(
        &self,
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
    /// Sets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails. A derailed minecart's velocity is multiplied
    /// by this factor each tick.
    pub fn set_derailed_velocity_mod(
        &self,
        derailed: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(derailed.into().jni_object().clone())
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block(
        &self,
        material: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(material.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block(
        &self,
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block_data(
        &self,
        block_data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_data.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block_data(
        &self,
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
    /// Sets the offset of the display block.
    pub fn set_display_block_offset(&self, offset: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(offset);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the offset of the display block.
    pub fn display_block_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
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
    /// Gets the vehicle's velocity.
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the vehicle's velocity in meters per tick.
    pub fn set_velocity(
        &self,
        vel: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(vel.into().jni_object().clone())
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
    /// Stores the entity's current position in the provided Location object.
    ///
    /// If the provided Location is null this method does nothing and returns
    /// null.
    pub fn get_location(
        &self,
        loc: impl Into<crate::Location<'mc>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(loc.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lorg/bukkit/Location;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLocation", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Gets the entity's height
    pub fn height(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's width
    pub fn width(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's current bounding box.
    ///
    /// The returned bounding box reflects the entity's current location and
    /// size.
    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/util/BoundingBox;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity is supported by a block. This value is a
    /// state updated by the server and is not recalculated unless the entity
    /// moves.
    pub fn is_on_ground(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns true if the entity is in water.
    pub fn is_in_water(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the current world this entity resides in
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/World;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the entity's rotation.
    ///
    /// Note that if the entity is affected by AI, it may override this rotation.
    pub fn set_rotation(&self, yaw: f32, pitch: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(yaw);
        args.push(val_1);
        sig += "F";
        let val_2 = jni::objects::JValueGen::Float(pitch);
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setRotation", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Teleports this entity to the given location. If this entity is riding a
    /// vehicle, it will be dismounted prior to teleportation.
    pub fn teleport(
        &self,
        location: impl Into<crate::Location<'mc>>,
        cause: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = cause {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;";
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
    /// Returns a list of entities within a bounding box centered around this
    /// entity
    pub fn get_nearby_entities(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(x);
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(y);
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(z);
        args.push(val_3);
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNearbyEntities", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Returns a unique id for this entity
    pub fn entity_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum fire ticks.
    pub fn max_fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn set_fire_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFireTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets if the entity has visual fire (it will always appear to be on fire).
    pub fn set_visual_fire(&self, fire: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(fire.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setVisualFire", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity has visual fire (it will always appear to be on fire).
    pub fn is_visual_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum freeze ticks (amount of ticks before it will
    /// be fully frozen)
    pub fn max_freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn set_freeze_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFreezeTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity is fully frozen (it has been in powdered snow for max
    /// freeze ticks).
    pub fn is_frozen(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Mark the entity's removal.
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if this entity has been marked for removal.
    pub fn is_dead(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns false if the entity has died, been despawned for some other
    /// reason, or has not been added to the world.
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the {@link Server} that contains this Entity
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Server;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity gets persisted.
    ///
    /// By default all entities are persistent. An entity will also not get
    /// persisted, if it is riding an entity that is not persistent.
    ///
    /// The persistent flag on players controls whether or not to save their
    /// playerdata file when they quit. If a player is directly or indirectly
    /// riding a non-persistent entity, the vehicle at the root and all its
    /// passengers won't get persisted.
    ///
    /// <b>This should not be confused with
    /// {@link LivingEntity#setRemoveWhenFarAway(boolean)} which controls
    /// despawning of living entities. </b>
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not the entity gets persisted.
    pub fn set_persistent(&self, persistent: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(persistent.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPersistent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Gets the primary passenger of a vehicle. For vehicles that could have multiple passengers, this will only return the primary passenger.
    pub fn passenger(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    #[deprecated]
    /// Set the passenger of a vehicle.
    pub fn set_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets a list of passengers of this vehicle.
    ///
    /// The returned list will not be directly linked to the entity's current
    /// passengers, and no guarantees are made as to its mutability.
    pub fn passengers(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Add a passenger to the vehicle.
    pub fn add_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Remove a passenger from the vehicle.
    pub fn remove_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removePassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check if a vehicle has passengers.
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Eject any passenger.
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the distance this entity has fallen
    pub fn fall_distance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")F";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the fall distance for this entity
    pub fn set_fall_distance(&self, distance: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(distance);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFallDistance", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Record the last {@link EntityDamageEvent} inflicted on this entity
    pub fn set_last_damage_cause(
        &self,
        event: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(event.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Retrieve the last {@link EntityDamageEvent} inflicted on this entity.
    /// This event may have been cancelled.
    pub fn last_damage_cause(
        &self,
    ) -> Result<Option<crate::event::entity::EntityDamageEvent<'mc>>, Box<dyn std::error::Error>>
    {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/event/entity/EntityDamageEvent;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::event::entity::EntityDamageEvent::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Returns a unique and persistent id for this entity
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/UUID;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities.
    pub fn ticks_lived(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities. May not be less than one
    /// tick.
    pub fn set_ticks_lived(&self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(value);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setTicksLived", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Performs the specified {@link EntityEffect} for this entity.
    ///
    /// This will be viewable to all players near the entity.
    ///
    /// If the effect is not applicable to this class of entity, it will not play.
    pub fn play_effect(
        &self,
        val_type: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/EntityEffect;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "playEffect", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the type of the entity.
    pub fn get_type(&self) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntityType;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntityType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes while swimming.
    pub fn swim_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water. For most
    /// entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_splash_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water at high
    /// speeds. For most entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_high_speed_splash_sound(
        &self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns whether this entity is inside a vehicle.
    pub fn is_inside_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Leave the current vehicle. If the entity is currently in a vehicle (and
    /// is removed from it), true will be returned, otherwise false will be
    /// returned.
    pub fn leave_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the vehicle that this entity is inside. If there is no vehicle,
    /// null will be returned.
    pub fn vehicle(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets whether or not to display the mob's custom name client side. The
    /// name will be displayed above the mob similarly to a player.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn set_custom_name_visible(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not the mob's custom name is displayed client side.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn is_custom_name_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn set_visible_by_default(&self, visible: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(visible.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn is_visible_by_default(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get all players that are currently tracking this entity.
    ///
    /// 'Tracking' means that this entity has been sent to the player and that
    /// they are receiving updates on its state. Note that the client's {@code
    /// 'Entity Distance'} setting does not affect the range at which entities
    /// are tracked.
    pub fn tracked_by(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTrackedBy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets whether the entity has a team colored (default: white) glow.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn set_glowing(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGlowing", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is glowing or not.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn is_glowing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is invulnerable or not.
    ///
    /// When an entity is invulnerable it can only be damaged by players in
    /// creative mode.
    pub fn set_invulnerable(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setInvulnerable", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is invulnerable or not.
    pub fn is_invulnerable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether the entity is silent or not.
    pub fn is_silent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is silent or not.
    ///
    /// When an entity is silent it will not produce any sound.
    pub fn set_silent(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setSilent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether gravity applies to this entity.
    pub fn has_gravity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether gravity applies to this entity.
    pub fn set_gravity(&self, gravity: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(gravity.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGravity", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the period of time (in ticks) before this entity can use a portal.
    pub fn portal_cooldown(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPortalCooldown", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the period of time (in ticks) before this entity can use a portal.
    pub fn set_portal_cooldown(&self, cooldown: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(cooldown);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPortalCooldown", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a set of tags for this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn scoreboard_tags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getScoreboardTags", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Add a tag to this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn add_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addScoreboardTag", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes a given tag from this entity.
    pub fn remove_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the entity when moved by a piston.
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/PistonMoveReaction;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the closest cardinal {@link BlockFace} direction an entity is
    /// currently facing.
    ///
    /// This will not return any non-cardinal directions such as
    /// {@link BlockFace#UP} or {@link BlockFace#DOWN}.
    ///
    /// {@link Hanging} entities will override this call and thus their behavior
    /// may be different.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/BlockFace;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the entity's current pose.
    /// <b>Note that the pose is only updated at the end of a tick, so may be
    /// inconsistent with other methods. eg {@link Player#isSneaking()} being
    /// true does not imply the current pose will be {@link Pose#SNEAKING}</b>
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Pose;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Pose::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the category of spawn to which this entity belongs.
    pub fn spawn_category(
        &self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/SpawnCategory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnCategory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::SpawnCategory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this entity has been spawned in a world.
    ///
    /// Entities not spawned in a world will not tick, be sent to players, or be
    /// saved to the server files.
    pub fn is_in_world(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get this entity as an NBT string.
    ///
    /// This string should not be relied upon as a serializable value.
    pub fn as_string(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsString", sig.as_str(), args);
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
    /// Crates an {@link EntitySnapshot} representing the current state of this entity.
    pub fn create_snapshot(
        &self,
    ) -> Result<Option<crate::entity::EntitySnapshot<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntitySnapshot;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "createSnapshot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntitySnapshot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Creates a copy of this entity and all its data. Spawns the copy at the given location.
    ///
    /// <b>Note:</b> Players cannot be copied.
    pub fn copy(
        &self,
        to: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = to {
            sig += "Lorg/bukkit/Location;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets a metadata value in the implementing object's metadata store.
    pub fn set_metadata(
        &self,
        metadata_key: impl Into<String>,
        new_metadata_value: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/metadata/MetadataValue;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_metadata_value.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a list of previously set metadata values from the implementing
    /// object's metadata store.
    pub fn get_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Tests to see whether the implementing object contains the given
    /// metadata value in its metadata store.
    pub fn has_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes the given metadata value from the implementing object's
    /// metadata store.
    pub fn remove_metadata(
        &self,
        metadata_key: impl Into<String>,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sends this sender multiple messages
    pub fn send_message(
        &self,
        sender: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        messages: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = messages {
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
    /// Gets the name of this command sender
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the custom name on a mob or block. If there is no name this method
    /// will return null.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), args);
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
    /// Sets a custom name on a mob or block. This name will be used in death
    /// messages and can be sent to the client as a nameplate over the mob.
    ///
    /// Setting the name to null or an empty string will clear it.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn set_custom_name(
        &self,
        name: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(name.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setCustomName", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a custom tag container capable of storing tags on the object.
    /// Note that the tags stored on this container are all stored under their
    /// own custom namespace therefore modifying default tags using this
    /// {@link PersistentDataHolder} is impossible.
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/persistence/PersistentDataContainer;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the object's inventory.
    pub fn inventory(
        &self,
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
    /// Set the loot table for a container or entity.
    ///
    /// To remove a loot table use null. Do not use {@link LootTables#EMPTY} to
    /// clear a LootTable.
    pub fn set_loot_table(
        &self,
        table: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/loot/LootTable;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(table.into().jni_object().clone())
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
    /// Gets the Loot Table attached to this block or entity.
    ///
    /// If an block/entity does not have a loot table, this will return null, NOT
    /// an empty loot table.
    pub fn loot_table(
        &self,
    ) -> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootTable;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootTable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::loot::LootTable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Set the seed used when this Loot Table generates loot.
    pub fn set_seed(&self, seed: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(seed);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Loot Table's seed.
    ///
    /// The seed is used when generating loot.
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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
#[repr(C)]
pub struct PoweredMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PoweredMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PoweredMinecart<'mc> {
    fn from_raw(
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
}

impl<'mc> PoweredMinecart<'mc> {
    /// Get the number of ticks until the minecart runs out of fuel.
    pub fn fuel(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the number of ticks until the minecart runs out of fuel.
    pub fn set_fuel(&self, fuel: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(fuel);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets a minecart's damage.
    pub fn set_damage(&self, damage: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(damage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a minecart's damage.
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum speed of a minecart. The speed is unrelated to the
    /// velocity.
    pub fn max_speed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Sets the maximum speed of a minecart. Must be nonnegative. Default is
    /// 0.4D.
    pub fn set_max_speed(&self, speed: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(speed);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn is_slow_when_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn set_slow_when_empty(&self, slow: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(slow.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn flying_velocity_mod(
        &self,
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
    /// Sets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn set_flying_velocity_mod(
        &self,
        flying: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(flying.into().jni_object().clone())
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
    /// Gets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails.
    ///
    /// A derailed minecart's velocity is multiplied by this factor each tick.
    pub fn derailed_velocity_mod(
        &self,
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
    /// Sets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails. A derailed minecart's velocity is multiplied
    /// by this factor each tick.
    pub fn set_derailed_velocity_mod(
        &self,
        derailed: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(derailed.into().jni_object().clone())
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block(
        &self,
        material: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(material.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block(
        &self,
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block_data(
        &self,
        block_data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_data.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block_data(
        &self,
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
    /// Sets the offset of the display block.
    pub fn set_display_block_offset(&self, offset: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(offset);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the offset of the display block.
    pub fn display_block_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
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
    /// Gets the vehicle's velocity.
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the vehicle's velocity in meters per tick.
    pub fn set_velocity(
        &self,
        vel: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(vel.into().jni_object().clone())
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
    /// Stores the entity's current position in the provided Location object.
    ///
    /// If the provided Location is null this method does nothing and returns
    /// null.
    pub fn get_location(
        &self,
        loc: impl Into<crate::Location<'mc>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(loc.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lorg/bukkit/Location;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLocation", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Gets the entity's height
    pub fn height(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's width
    pub fn width(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's current bounding box.
    ///
    /// The returned bounding box reflects the entity's current location and
    /// size.
    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/util/BoundingBox;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity is supported by a block. This value is a
    /// state updated by the server and is not recalculated unless the entity
    /// moves.
    pub fn is_on_ground(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns true if the entity is in water.
    pub fn is_in_water(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the current world this entity resides in
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/World;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the entity's rotation.
    ///
    /// Note that if the entity is affected by AI, it may override this rotation.
    pub fn set_rotation(&self, yaw: f32, pitch: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(yaw);
        args.push(val_1);
        sig += "F";
        let val_2 = jni::objects::JValueGen::Float(pitch);
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setRotation", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Teleports this entity to the given location. If this entity is riding a
    /// vehicle, it will be dismounted prior to teleportation.
    pub fn teleport(
        &self,
        location: impl Into<crate::Location<'mc>>,
        cause: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = cause {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;";
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
    /// Returns a list of entities within a bounding box centered around this
    /// entity
    pub fn get_nearby_entities(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(x);
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(y);
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(z);
        args.push(val_3);
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNearbyEntities", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Returns a unique id for this entity
    pub fn entity_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum fire ticks.
    pub fn max_fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn set_fire_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFireTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets if the entity has visual fire (it will always appear to be on fire).
    pub fn set_visual_fire(&self, fire: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(fire.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setVisualFire", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity has visual fire (it will always appear to be on fire).
    pub fn is_visual_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum freeze ticks (amount of ticks before it will
    /// be fully frozen)
    pub fn max_freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn set_freeze_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFreezeTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity is fully frozen (it has been in powdered snow for max
    /// freeze ticks).
    pub fn is_frozen(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Mark the entity's removal.
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if this entity has been marked for removal.
    pub fn is_dead(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns false if the entity has died, been despawned for some other
    /// reason, or has not been added to the world.
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the {@link Server} that contains this Entity
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Server;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity gets persisted.
    ///
    /// By default all entities are persistent. An entity will also not get
    /// persisted, if it is riding an entity that is not persistent.
    ///
    /// The persistent flag on players controls whether or not to save their
    /// playerdata file when they quit. If a player is directly or indirectly
    /// riding a non-persistent entity, the vehicle at the root and all its
    /// passengers won't get persisted.
    ///
    /// <b>This should not be confused with
    /// {@link LivingEntity#setRemoveWhenFarAway(boolean)} which controls
    /// despawning of living entities. </b>
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not the entity gets persisted.
    pub fn set_persistent(&self, persistent: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(persistent.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPersistent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Gets the primary passenger of a vehicle. For vehicles that could have multiple passengers, this will only return the primary passenger.
    pub fn passenger(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    #[deprecated]
    /// Set the passenger of a vehicle.
    pub fn set_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets a list of passengers of this vehicle.
    ///
    /// The returned list will not be directly linked to the entity's current
    /// passengers, and no guarantees are made as to its mutability.
    pub fn passengers(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Add a passenger to the vehicle.
    pub fn add_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Remove a passenger from the vehicle.
    pub fn remove_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removePassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check if a vehicle has passengers.
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Eject any passenger.
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the distance this entity has fallen
    pub fn fall_distance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")F";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the fall distance for this entity
    pub fn set_fall_distance(&self, distance: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(distance);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFallDistance", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Record the last {@link EntityDamageEvent} inflicted on this entity
    pub fn set_last_damage_cause(
        &self,
        event: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(event.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Retrieve the last {@link EntityDamageEvent} inflicted on this entity.
    /// This event may have been cancelled.
    pub fn last_damage_cause(
        &self,
    ) -> Result<Option<crate::event::entity::EntityDamageEvent<'mc>>, Box<dyn std::error::Error>>
    {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/event/entity/EntityDamageEvent;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::event::entity::EntityDamageEvent::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Returns a unique and persistent id for this entity
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/UUID;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities.
    pub fn ticks_lived(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities. May not be less than one
    /// tick.
    pub fn set_ticks_lived(&self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(value);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setTicksLived", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Performs the specified {@link EntityEffect} for this entity.
    ///
    /// This will be viewable to all players near the entity.
    ///
    /// If the effect is not applicable to this class of entity, it will not play.
    pub fn play_effect(
        &self,
        val_type: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/EntityEffect;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "playEffect", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the type of the entity.
    pub fn get_type(&self) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntityType;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntityType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes while swimming.
    pub fn swim_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water. For most
    /// entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_splash_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water at high
    /// speeds. For most entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_high_speed_splash_sound(
        &self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns whether this entity is inside a vehicle.
    pub fn is_inside_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Leave the current vehicle. If the entity is currently in a vehicle (and
    /// is removed from it), true will be returned, otherwise false will be
    /// returned.
    pub fn leave_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the vehicle that this entity is inside. If there is no vehicle,
    /// null will be returned.
    pub fn vehicle(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets whether or not to display the mob's custom name client side. The
    /// name will be displayed above the mob similarly to a player.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn set_custom_name_visible(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not the mob's custom name is displayed client side.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn is_custom_name_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn set_visible_by_default(&self, visible: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(visible.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn is_visible_by_default(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get all players that are currently tracking this entity.
    ///
    /// 'Tracking' means that this entity has been sent to the player and that
    /// they are receiving updates on its state. Note that the client's {@code
    /// 'Entity Distance'} setting does not affect the range at which entities
    /// are tracked.
    pub fn tracked_by(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTrackedBy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets whether the entity has a team colored (default: white) glow.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn set_glowing(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGlowing", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is glowing or not.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn is_glowing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is invulnerable or not.
    ///
    /// When an entity is invulnerable it can only be damaged by players in
    /// creative mode.
    pub fn set_invulnerable(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setInvulnerable", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is invulnerable or not.
    pub fn is_invulnerable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether the entity is silent or not.
    pub fn is_silent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is silent or not.
    ///
    /// When an entity is silent it will not produce any sound.
    pub fn set_silent(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setSilent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether gravity applies to this entity.
    pub fn has_gravity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether gravity applies to this entity.
    pub fn set_gravity(&self, gravity: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(gravity.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGravity", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the period of time (in ticks) before this entity can use a portal.
    pub fn portal_cooldown(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPortalCooldown", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the period of time (in ticks) before this entity can use a portal.
    pub fn set_portal_cooldown(&self, cooldown: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(cooldown);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPortalCooldown", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a set of tags for this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn scoreboard_tags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getScoreboardTags", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Add a tag to this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn add_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addScoreboardTag", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes a given tag from this entity.
    pub fn remove_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the entity when moved by a piston.
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/PistonMoveReaction;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the closest cardinal {@link BlockFace} direction an entity is
    /// currently facing.
    ///
    /// This will not return any non-cardinal directions such as
    /// {@link BlockFace#UP} or {@link BlockFace#DOWN}.
    ///
    /// {@link Hanging} entities will override this call and thus their behavior
    /// may be different.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/BlockFace;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the entity's current pose.
    /// <b>Note that the pose is only updated at the end of a tick, so may be
    /// inconsistent with other methods. eg {@link Player#isSneaking()} being
    /// true does not imply the current pose will be {@link Pose#SNEAKING}</b>
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Pose;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Pose::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the category of spawn to which this entity belongs.
    pub fn spawn_category(
        &self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/SpawnCategory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnCategory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::SpawnCategory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this entity has been spawned in a world.
    ///
    /// Entities not spawned in a world will not tick, be sent to players, or be
    /// saved to the server files.
    pub fn is_in_world(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get this entity as an NBT string.
    ///
    /// This string should not be relied upon as a serializable value.
    pub fn as_string(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsString", sig.as_str(), args);
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
    /// Crates an {@link EntitySnapshot} representing the current state of this entity.
    pub fn create_snapshot(
        &self,
    ) -> Result<Option<crate::entity::EntitySnapshot<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntitySnapshot;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "createSnapshot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntitySnapshot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Creates a copy of this entity and all its data. Spawns the copy at the given location.
    ///
    /// <b>Note:</b> Players cannot be copied.
    pub fn copy(
        &self,
        to: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = to {
            sig += "Lorg/bukkit/Location;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets a metadata value in the implementing object's metadata store.
    pub fn set_metadata(
        &self,
        metadata_key: impl Into<String>,
        new_metadata_value: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/metadata/MetadataValue;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_metadata_value.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a list of previously set metadata values from the implementing
    /// object's metadata store.
    pub fn get_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Tests to see whether the implementing object contains the given
    /// metadata value in its metadata store.
    pub fn has_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes the given metadata value from the implementing object's
    /// metadata store.
    pub fn remove_metadata(
        &self,
        metadata_key: impl Into<String>,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sends this sender multiple messages
    pub fn send_message(
        &self,
        sender: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        messages: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = messages {
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
    /// Gets the name of this command sender
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the custom name on a mob or block. If there is no name this method
    /// will return null.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), args);
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
    /// Sets a custom name on a mob or block. This name will be used in death
    /// messages and can be sent to the client as a nameplate over the mob.
    ///
    /// Setting the name to null or an empty string will clear it.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn set_custom_name(
        &self,
        name: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(name.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setCustomName", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a custom tag container capable of storing tags on the object.
    /// Note that the tags stored on this container are all stored under their
    /// own custom namespace therefore modifying default tags using this
    /// {@link PersistentDataHolder} is impossible.
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/persistence/PersistentDataContainer;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for PoweredMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PoweredMinecart into crate::entity::Minecart")
    }
}
#[repr(C)]
pub struct RideableMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RideableMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RideableMinecart<'mc> {
    fn from_raw(
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
}

impl<'mc> RideableMinecart<'mc> {
    /// Sets a minecart's damage.
    pub fn set_damage(&self, damage: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(damage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a minecart's damage.
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum speed of a minecart. The speed is unrelated to the
    /// velocity.
    pub fn max_speed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Sets the maximum speed of a minecart. Must be nonnegative. Default is
    /// 0.4D.
    pub fn set_max_speed(&self, speed: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(speed);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn is_slow_when_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn set_slow_when_empty(&self, slow: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(slow.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn flying_velocity_mod(
        &self,
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
    /// Sets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn set_flying_velocity_mod(
        &self,
        flying: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(flying.into().jni_object().clone())
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
    /// Gets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails.
    ///
    /// A derailed minecart's velocity is multiplied by this factor each tick.
    pub fn derailed_velocity_mod(
        &self,
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
    /// Sets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails. A derailed minecart's velocity is multiplied
    /// by this factor each tick.
    pub fn set_derailed_velocity_mod(
        &self,
        derailed: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(derailed.into().jni_object().clone())
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block(
        &self,
        material: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(material.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block(
        &self,
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block_data(
        &self,
        block_data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_data.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block_data(
        &self,
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
    /// Sets the offset of the display block.
    pub fn set_display_block_offset(&self, offset: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(offset);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the offset of the display block.
    pub fn display_block_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
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
    /// Gets the vehicle's velocity.
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the vehicle's velocity in meters per tick.
    pub fn set_velocity(
        &self,
        vel: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(vel.into().jni_object().clone())
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
    /// Stores the entity's current position in the provided Location object.
    ///
    /// If the provided Location is null this method does nothing and returns
    /// null.
    pub fn get_location(
        &self,
        loc: impl Into<crate::Location<'mc>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(loc.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lorg/bukkit/Location;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLocation", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Gets the entity's height
    pub fn height(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's width
    pub fn width(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's current bounding box.
    ///
    /// The returned bounding box reflects the entity's current location and
    /// size.
    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/util/BoundingBox;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity is supported by a block. This value is a
    /// state updated by the server and is not recalculated unless the entity
    /// moves.
    pub fn is_on_ground(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns true if the entity is in water.
    pub fn is_in_water(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the current world this entity resides in
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/World;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the entity's rotation.
    ///
    /// Note that if the entity is affected by AI, it may override this rotation.
    pub fn set_rotation(&self, yaw: f32, pitch: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(yaw);
        args.push(val_1);
        sig += "F";
        let val_2 = jni::objects::JValueGen::Float(pitch);
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setRotation", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Teleports this entity to the given location. If this entity is riding a
    /// vehicle, it will be dismounted prior to teleportation.
    pub fn teleport(
        &self,
        location: impl Into<crate::Location<'mc>>,
        cause: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = cause {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;";
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
    /// Returns a list of entities within a bounding box centered around this
    /// entity
    pub fn get_nearby_entities(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(x);
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(y);
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(z);
        args.push(val_3);
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNearbyEntities", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Returns a unique id for this entity
    pub fn entity_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum fire ticks.
    pub fn max_fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn set_fire_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFireTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets if the entity has visual fire (it will always appear to be on fire).
    pub fn set_visual_fire(&self, fire: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(fire.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setVisualFire", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity has visual fire (it will always appear to be on fire).
    pub fn is_visual_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum freeze ticks (amount of ticks before it will
    /// be fully frozen)
    pub fn max_freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn set_freeze_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFreezeTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity is fully frozen (it has been in powdered snow for max
    /// freeze ticks).
    pub fn is_frozen(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Mark the entity's removal.
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if this entity has been marked for removal.
    pub fn is_dead(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns false if the entity has died, been despawned for some other
    /// reason, or has not been added to the world.
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the {@link Server} that contains this Entity
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Server;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity gets persisted.
    ///
    /// By default all entities are persistent. An entity will also not get
    /// persisted, if it is riding an entity that is not persistent.
    ///
    /// The persistent flag on players controls whether or not to save their
    /// playerdata file when they quit. If a player is directly or indirectly
    /// riding a non-persistent entity, the vehicle at the root and all its
    /// passengers won't get persisted.
    ///
    /// <b>This should not be confused with
    /// {@link LivingEntity#setRemoveWhenFarAway(boolean)} which controls
    /// despawning of living entities. </b>
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not the entity gets persisted.
    pub fn set_persistent(&self, persistent: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(persistent.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPersistent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Gets the primary passenger of a vehicle. For vehicles that could have multiple passengers, this will only return the primary passenger.
    pub fn passenger(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    #[deprecated]
    /// Set the passenger of a vehicle.
    pub fn set_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets a list of passengers of this vehicle.
    ///
    /// The returned list will not be directly linked to the entity's current
    /// passengers, and no guarantees are made as to its mutability.
    pub fn passengers(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Add a passenger to the vehicle.
    pub fn add_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Remove a passenger from the vehicle.
    pub fn remove_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removePassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check if a vehicle has passengers.
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Eject any passenger.
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the distance this entity has fallen
    pub fn fall_distance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")F";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the fall distance for this entity
    pub fn set_fall_distance(&self, distance: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(distance);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFallDistance", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Record the last {@link EntityDamageEvent} inflicted on this entity
    pub fn set_last_damage_cause(
        &self,
        event: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(event.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Retrieve the last {@link EntityDamageEvent} inflicted on this entity.
    /// This event may have been cancelled.
    pub fn last_damage_cause(
        &self,
    ) -> Result<Option<crate::event::entity::EntityDamageEvent<'mc>>, Box<dyn std::error::Error>>
    {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/event/entity/EntityDamageEvent;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::event::entity::EntityDamageEvent::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Returns a unique and persistent id for this entity
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/UUID;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities.
    pub fn ticks_lived(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities. May not be less than one
    /// tick.
    pub fn set_ticks_lived(&self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(value);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setTicksLived", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Performs the specified {@link EntityEffect} for this entity.
    ///
    /// This will be viewable to all players near the entity.
    ///
    /// If the effect is not applicable to this class of entity, it will not play.
    pub fn play_effect(
        &self,
        val_type: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/EntityEffect;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "playEffect", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the type of the entity.
    pub fn get_type(&self) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntityType;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntityType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes while swimming.
    pub fn swim_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water. For most
    /// entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_splash_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water at high
    /// speeds. For most entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_high_speed_splash_sound(
        &self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns whether this entity is inside a vehicle.
    pub fn is_inside_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Leave the current vehicle. If the entity is currently in a vehicle (and
    /// is removed from it), true will be returned, otherwise false will be
    /// returned.
    pub fn leave_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the vehicle that this entity is inside. If there is no vehicle,
    /// null will be returned.
    pub fn vehicle(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets whether or not to display the mob's custom name client side. The
    /// name will be displayed above the mob similarly to a player.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn set_custom_name_visible(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not the mob's custom name is displayed client side.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn is_custom_name_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn set_visible_by_default(&self, visible: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(visible.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn is_visible_by_default(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get all players that are currently tracking this entity.
    ///
    /// 'Tracking' means that this entity has been sent to the player and that
    /// they are receiving updates on its state. Note that the client's {@code
    /// 'Entity Distance'} setting does not affect the range at which entities
    /// are tracked.
    pub fn tracked_by(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTrackedBy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets whether the entity has a team colored (default: white) glow.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn set_glowing(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGlowing", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is glowing or not.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn is_glowing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is invulnerable or not.
    ///
    /// When an entity is invulnerable it can only be damaged by players in
    /// creative mode.
    pub fn set_invulnerable(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setInvulnerable", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is invulnerable or not.
    pub fn is_invulnerable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether the entity is silent or not.
    pub fn is_silent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is silent or not.
    ///
    /// When an entity is silent it will not produce any sound.
    pub fn set_silent(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setSilent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether gravity applies to this entity.
    pub fn has_gravity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether gravity applies to this entity.
    pub fn set_gravity(&self, gravity: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(gravity.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGravity", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the period of time (in ticks) before this entity can use a portal.
    pub fn portal_cooldown(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPortalCooldown", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the period of time (in ticks) before this entity can use a portal.
    pub fn set_portal_cooldown(&self, cooldown: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(cooldown);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPortalCooldown", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a set of tags for this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn scoreboard_tags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getScoreboardTags", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Add a tag to this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn add_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addScoreboardTag", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes a given tag from this entity.
    pub fn remove_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the entity when moved by a piston.
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/PistonMoveReaction;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the closest cardinal {@link BlockFace} direction an entity is
    /// currently facing.
    ///
    /// This will not return any non-cardinal directions such as
    /// {@link BlockFace#UP} or {@link BlockFace#DOWN}.
    ///
    /// {@link Hanging} entities will override this call and thus their behavior
    /// may be different.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/BlockFace;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the entity's current pose.
    /// <b>Note that the pose is only updated at the end of a tick, so may be
    /// inconsistent with other methods. eg {@link Player#isSneaking()} being
    /// true does not imply the current pose will be {@link Pose#SNEAKING}</b>
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Pose;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Pose::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the category of spawn to which this entity belongs.
    pub fn spawn_category(
        &self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/SpawnCategory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnCategory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::SpawnCategory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this entity has been spawned in a world.
    ///
    /// Entities not spawned in a world will not tick, be sent to players, or be
    /// saved to the server files.
    pub fn is_in_world(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get this entity as an NBT string.
    ///
    /// This string should not be relied upon as a serializable value.
    pub fn as_string(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsString", sig.as_str(), args);
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
    /// Crates an {@link EntitySnapshot} representing the current state of this entity.
    pub fn create_snapshot(
        &self,
    ) -> Result<Option<crate::entity::EntitySnapshot<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntitySnapshot;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "createSnapshot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntitySnapshot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Creates a copy of this entity and all its data. Spawns the copy at the given location.
    ///
    /// <b>Note:</b> Players cannot be copied.
    pub fn copy(
        &self,
        to: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = to {
            sig += "Lorg/bukkit/Location;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets a metadata value in the implementing object's metadata store.
    pub fn set_metadata(
        &self,
        metadata_key: impl Into<String>,
        new_metadata_value: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/metadata/MetadataValue;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_metadata_value.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a list of previously set metadata values from the implementing
    /// object's metadata store.
    pub fn get_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Tests to see whether the implementing object contains the given
    /// metadata value in its metadata store.
    pub fn has_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes the given metadata value from the implementing object's
    /// metadata store.
    pub fn remove_metadata(
        &self,
        metadata_key: impl Into<String>,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sends this sender multiple messages
    pub fn send_message(
        &self,
        sender: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        messages: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = messages {
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
    /// Gets the name of this command sender
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the custom name on a mob or block. If there is no name this method
    /// will return null.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), args);
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
    /// Sets a custom name on a mob or block. This name will be used in death
    /// messages and can be sent to the client as a nameplate over the mob.
    ///
    /// Setting the name to null or an empty string will clear it.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn set_custom_name(
        &self,
        name: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(name.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setCustomName", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a custom tag container capable of storing tags on the object.
    /// Note that the tags stored on this container are all stored under their
    /// own custom namespace therefore modifying default tags using this
    /// {@link PersistentDataHolder} is impossible.
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/persistence/PersistentDataContainer;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for RideableMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RideableMinecart into crate::entity::Minecart")
    }
}
#[repr(C)]
pub struct ExplosiveMinecart<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ExplosiveMinecart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ExplosiveMinecart<'mc> {
    fn from_raw(
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
}

impl<'mc> ExplosiveMinecart<'mc> {
    /// Set the fuse ticks of this minecart.
    /// If the fuse ticks are set to a non-zero value, this will ignite the
    /// explosive.
    pub fn set_fuse_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(ticks);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuseTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the fuse ticks of this minecart.
    /// If the fuse ticks reach 0, the minecart will explode.
    pub fn fuse_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFuseTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Ignite this minecart's fuse naturally.
    pub fn ignite(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ignite", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Check whether or not this minecart's fuse has been ignited.
    pub fn is_ignited(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isIgnited", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Immediately explode this minecart with the given power.
    pub fn explode(
        &self,
        power: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = power {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a);
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "explode", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets a minecart's damage.
    pub fn set_damage(&self, damage: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(damage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDamage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a minecart's damage.
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDamage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum speed of a minecart. The speed is unrelated to the
    /// velocity.
    pub fn max_speed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSpeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Sets the maximum speed of a minecart. Must be nonnegative. Default is
    /// 0.4D.
    pub fn set_max_speed(&self, speed: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(speed);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn is_slow_when_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSlowWhenEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this minecart will slow down faster without a passenger
    /// occupying it
    pub fn set_slow_when_empty(&self, slow: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(slow.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlowWhenEmpty",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn flying_velocity_mod(
        &self,
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
    /// Sets the flying velocity modifier. Used for minecarts that are in
    /// mid-air. A flying minecart's velocity is multiplied by this factor each
    /// tick.
    pub fn set_flying_velocity_mod(
        &self,
        flying: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(flying.into().jni_object().clone())
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
    /// Gets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails.
    ///
    /// A derailed minecart's velocity is multiplied by this factor each tick.
    pub fn derailed_velocity_mod(
        &self,
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
    /// Sets the derailed velocity modifier. Used for minecarts that are on the
    /// ground, but not on rails. A derailed minecart's velocity is multiplied
    /// by this factor each tick.
    pub fn set_derailed_velocity_mod(
        &self,
        derailed: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(derailed.into().jni_object().clone())
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block(
        &self,
        material: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/material/MaterialData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(material.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block(
        &self,
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
    /// Sets the display block for this minecart.
    /// Passing a null value will set the minecart to have no display block.
    pub fn set_display_block_data(
        &self,
        block_data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_data.into().jni_object().clone())
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
    /// Gets the display block for this minecart.
    /// This function will return the type AIR if none is set.
    pub fn display_block_data(
        &self,
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
    /// Sets the offset of the display block.
    pub fn set_display_block_offset(&self, offset: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(offset);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisplayBlockOffset",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the offset of the display block.
    pub fn display_block_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
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
    /// Gets the vehicle's velocity.
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/Vector;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getVelocity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::Vector::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the vehicle's velocity in meters per tick.
    pub fn set_velocity(
        &self,
        vel: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/util/Vector;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(vel.into().jni_object().clone())
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
    /// Stores the entity's current position in the provided Location object.
    ///
    /// If the provided Location is null this method does nothing and returns
    /// null.
    pub fn get_location(
        &self,
        loc: impl Into<crate::Location<'mc>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(loc.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Lorg/bukkit/Location;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLocation", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    /// Gets the entity's height
    pub fn height(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's width
    pub fn width(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the entity's current bounding box.
    ///
    /// The returned bounding box reflects the entity's current location and
    /// size.
    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/util/BoundingBox;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBoundingBox", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::BoundingBox::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity is supported by a block. This value is a
    /// state updated by the server and is not recalculated unless the entity
    /// moves.
    pub fn is_on_ground(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOnGround", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns true if the entity is in water.
    pub fn is_in_water(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWater", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the current world this entity resides in
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/World;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the entity's rotation.
    ///
    /// Note that if the entity is affected by AI, it may override this rotation.
    pub fn set_rotation(&self, yaw: f32, pitch: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(yaw);
        args.push(val_1);
        sig += "F";
        let val_2 = jni::objects::JValueGen::Float(pitch);
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setRotation", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Teleports this entity to the given location. If this entity is riding a
    /// vehicle, it will be dismounted prior to teleportation.
    pub fn teleport(
        &self,
        location: impl Into<crate::Location<'mc>>,
        cause: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = cause {
            sig += "Lorg/bukkit/event/player/PlayerTeleportEvent/TeleportCause;";
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
    /// Returns a list of entities within a bounding box centered around this
    /// entity
    pub fn get_nearby_entities(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(x);
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(y);
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(z);
        args.push(val_3);
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getNearbyEntities", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Returns a unique id for this entity
    pub fn entity_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEntityId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum fire ticks.
    pub fn max_fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current fire ticks (ticks before the entity stops
    /// being on fire).
    pub fn set_fire_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFireTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets if the entity has visual fire (it will always appear to be on fire).
    pub fn set_visual_fire(&self, fire: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(fire.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setVisualFire", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity has visual fire (it will always appear to be on fire).
    pub fn is_visual_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isVisualFire", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Returns the entity's maximum freeze ticks (amount of ticks before it will
    /// be fully frozen)
    pub fn max_freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFreezeTicks", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the entity's current freeze ticks (amount of ticks the entity has
    /// been in powdered snow).
    pub fn set_freeze_ticks(&self, ticks: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(ticks);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFreezeTicks", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets if the entity is fully frozen (it has been in powdered snow for max
    /// freeze ticks).
    pub fn is_frozen(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isFrozen", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Mark the entity's removal.
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns true if this entity has been marked for removal.
    pub fn is_dead(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDead", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns false if the entity has died, been despawned for some other
    /// reason, or has not been added to the world.
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isValid", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the {@link Server} that contains this Entity
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Server;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getServer", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns true if the entity gets persisted.
    ///
    /// By default all entities are persistent. An entity will also not get
    /// persisted, if it is riding an entity that is not persistent.
    ///
    /// The persistent flag on players controls whether or not to save their
    /// playerdata file when they quit. If a player is directly or indirectly
    /// riding a non-persistent entity, the vehicle at the root and all its
    /// passengers won't get persisted.
    ///
    /// <b>This should not be confused with
    /// {@link LivingEntity#setRemoveWhenFarAway(boolean)} which controls
    /// despawning of living entities. </b>
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not the entity gets persisted.
    pub fn set_persistent(&self, persistent: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(persistent.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPersistent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Gets the primary passenger of a vehicle. For vehicles that could have multiple passengers, this will only return the primary passenger.
    pub fn passenger(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    #[deprecated]
    /// Set the passenger of a vehicle.
    pub fn set_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets a list of passengers of this vehicle.
    ///
    /// The returned list will not be directly linked to the entity's current
    /// passengers, and no guarantees are made as to its mutability.
    pub fn passengers(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/List;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Add a passenger to the vehicle.
    pub fn add_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addPassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Remove a passenger from the vehicle.
    pub fn remove_passenger(
        &self,
        passenger: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/Entity;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(passenger.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removePassenger", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check if a vehicle has passengers.
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Eject any passenger.
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "eject", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the distance this entity has fallen
    pub fn fall_distance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")F";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFallDistance", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the fall distance for this entity
    pub fn set_fall_distance(&self, distance: f32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(distance);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setFallDistance", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// Record the last {@link EntityDamageEvent} inflicted on this entity
    pub fn set_last_damage_cause(
        &self,
        event: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/event/entity/EntityDamageEvent;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(event.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLastDamageCause",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Retrieve the last {@link EntityDamageEvent} inflicted on this entity.
    /// This event may have been cancelled.
    pub fn last_damage_cause(
        &self,
    ) -> Result<Option<crate::event::entity::EntityDamageEvent<'mc>>, Box<dyn std::error::Error>>
    {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/event/entity/EntityDamageEvent;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLastDamageCause",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::event::entity::EntityDamageEvent::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Returns a unique and persistent id for this entity
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/UUID;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getUniqueId", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities.
    pub fn ticks_lived(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTicksLived", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the amount of ticks this entity has lived for.
    ///
    /// This is the equivalent to "age" in entities. May not be less than one
    /// tick.
    pub fn set_ticks_lived(&self, value: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(value);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setTicksLived", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Performs the specified {@link EntityEffect} for this entity.
    ///
    /// This will be viewable to all players near the entity.
    ///
    /// If the effect is not applicable to this class of entity, it will not play.
    pub fn play_effect(
        &self,
        val_type: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/EntityEffect;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "playEffect", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the type of the entity.
    pub fn get_type(&self) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntityType;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntityType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes while swimming.
    pub fn swim_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSwimSound", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water. For most
    /// entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_splash_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link Sound} this entity makes when splashing in water at high
    /// speeds. For most entities, this is just {@link Sound#ENTITY_GENERIC_SPLASH}.
    pub fn swim_high_speed_splash_sound(
        &self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/Sound;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSwimHighSpeedSplashSound",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Sound::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns whether this entity is inside a vehicle.
    pub fn is_inside_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInsideVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Leave the current vehicle. If the entity is currently in a vehicle (and
    /// is removed from it), true will be returned, otherwise false will be
    /// returned.
    pub fn leave_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leaveVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the vehicle that this entity is inside. If there is no vehicle,
    /// null will be returned.
    pub fn vehicle(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getVehicle", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets whether or not to display the mob's custom name client side. The
    /// name will be displayed above the mob similarly to a player.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn set_custom_name_visible(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCustomNameVisible",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not the mob's custom name is displayed client side.
    ///
    /// This value has no effect on players, they will always display their
    /// name.
    pub fn is_custom_name_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isCustomNameVisible",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn set_visible_by_default(&self, visible: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(visible.into());
        args.push(val_1);
        sig += ")V";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisibleByDefault",
            sig.as_str(),
            args,
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether or not this entity is visible by default.
    /// If this entity is not visible by default, then
    /// {@link Player#showEntity(org.bukkit.plugin.Plugin, org.bukkit.entity.Entity)}
    /// will need to be called before the entity is visible to a given player.
    pub fn is_visible_by_default(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isVisibleByDefault",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get all players that are currently tracking this entity.
    ///
    /// 'Tracking' means that this entity has been sent to the player and that
    /// they are receiving updates on its state. Note that the client's {@code
    /// 'Entity Distance'} setting does not affect the range at which entities
    /// are tracked.
    pub fn tracked_by(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTrackedBy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets whether the entity has a team colored (default: white) glow.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn set_glowing(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGlowing", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is glowing or not.
    /// <b>nb: this refers to the 'Glowing' entity property, not whether a
    /// glowing potion effect is applied</b>
    pub fn is_glowing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isGlowing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is invulnerable or not.
    ///
    /// When an entity is invulnerable it can only be damaged by players in
    /// creative mode.
    pub fn set_invulnerable(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setInvulnerable", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets whether the entity is invulnerable or not.
    pub fn is_invulnerable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInvulnerable", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets whether the entity is silent or not.
    pub fn is_silent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSilent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the entity is silent or not.
    ///
    /// When an entity is silent it will not produce any sound.
    pub fn set_silent(&self, flag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(flag.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setSilent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns whether gravity applies to this entity.
    pub fn has_gravity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasGravity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether gravity applies to this entity.
    pub fn set_gravity(&self, gravity: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(gravity.into());
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setGravity", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the period of time (in ticks) before this entity can use a portal.
    pub fn portal_cooldown(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")I";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPortalCooldown", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the period of time (in ticks) before this entity can use a portal.
    pub fn set_portal_cooldown(&self, cooldown: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(cooldown);
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPortalCooldown", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a set of tags for this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn scoreboard_tags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Set;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getScoreboardTags", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Add a tag to this entity.
    ///
    /// Entities can have no more than 1024 tags.
    pub fn add_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addScoreboardTag", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes a given tag from this entity.
    pub fn remove_scoreboard_tag(
        &self,
        tag: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(tag.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeScoreboardTag",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the entity when moved by a piston.
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/PistonMoveReaction;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the closest cardinal {@link BlockFace} direction an entity is
    /// currently facing.
    ///
    /// This will not return any non-cardinal directions such as
    /// {@link BlockFace#UP} or {@link BlockFace#DOWN}.
    ///
    /// {@link Hanging} entities will override this call and thus their behavior
    /// may be different.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/BlockFace;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the entity's current pose.
    /// <b>Note that the pose is only updated at the end of a tick, so may be
    /// inconsistent with other methods. eg {@link Player#isSneaking()} being
    /// true does not imply the current pose will be {@link Pose#SNEAKING}</b>
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/Pose;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPose", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Pose::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the category of spawn to which this entity belongs.
    pub fn spawn_category(
        &self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/SpawnCategory;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnCategory", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::SpawnCategory::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this entity has been spawned in a world.
    ///
    /// Entities not spawned in a world will not tick, be sent to players, or be
    /// saved to the server files.
    pub fn is_in_world(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWorld", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get this entity as an NBT string.
    ///
    /// This string should not be relied upon as a serializable value.
    pub fn as_string(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsString", sig.as_str(), args);
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
    /// Crates an {@link EntitySnapshot} representing the current state of this entity.
    pub fn create_snapshot(
        &self,
    ) -> Result<Option<crate::entity::EntitySnapshot<'mc>>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/entity/EntitySnapshot;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "createSnapshot", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntitySnapshot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Creates a copy of this entity and all its data. Spawns the copy at the given location.
    ///
    /// <b>Note:</b> Players cannot be copied.
    pub fn copy(
        &self,
        to: std::option::Option<impl Into<crate::Location<'mc>>>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = to {
            sig += "Lorg/bukkit/Location;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets a metadata value in the implementing object's metadata store.
    pub fn set_metadata(
        &self,
        metadata_key: impl Into<String>,
        new_metadata_value: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/metadata/MetadataValue;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(new_metadata_value.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a list of previously set metadata values from the implementing
    /// object's metadata store.
    pub fn get_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Ljava/util/List;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::metadata::MetadataValue::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Tests to see whether the implementing object contains the given
    /// metadata value in its metadata store.
    pub fn has_metadata(
        &self,
        metadata_key: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMetadata", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Removes the given metadata value from the implementing object's
    /// metadata store.
    pub fn remove_metadata(
        &self,
        metadata_key: impl Into<String>,
        owning_plugin: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(metadata_key.into())?,
        ));
        args.push(val_1);
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(owning_plugin.into().jni_object().clone())
        });
        args.push(val_2);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "removeMetadata", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sends this sender multiple messages
    pub fn send_message(
        &self,
        sender: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
        messages: std::option::Option<impl Into<String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/UUID;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sender.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = messages {
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
    /// Gets the name of this command sender
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the custom name on a mob or block. If there is no name this method
    /// will return null.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/lang/String;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCustomName", sig.as_str(), args);
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
    /// Sets a custom name on a mob or block. This name will be used in death
    /// messages and can be sent to the client as a nameplate over the mob.
    ///
    /// Setting the name to null or an empty string will clear it.
    ///
    /// This value has no effect on players, they will always use their real
    /// name.
    pub fn set_custom_name(
        &self,
        name: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(name.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setCustomName", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Returns a custom tag container capable of storing tags on the object.
    /// Note that the tags stored on this container are all stored under their
    /// own custom namespace therefore modifying default tags using this
    /// {@link PersistentDataHolder} is impossible.
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/persistence/PersistentDataContainer;";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPersistentDataContainer",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::persistence::PersistentDataContainer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::entity::Minecart<'mc>> for ExplosiveMinecart<'mc> {
    fn into(self) -> crate::entity::Minecart<'mc> {
        crate::entity::Minecart::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ExplosiveMinecart into crate::entity::Minecart")
    }
}
