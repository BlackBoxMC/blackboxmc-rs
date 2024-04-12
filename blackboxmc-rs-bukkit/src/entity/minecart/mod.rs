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
