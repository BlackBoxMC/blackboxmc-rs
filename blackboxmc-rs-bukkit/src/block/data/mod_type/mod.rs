#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct Sapling<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Sapling<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Sapling<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sapling from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Sapling")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sapling object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Sapling<'mc> {
    /// Gets the value of the 'stage' property.
    pub fn stage(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getStage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'stage' property.
    pub fn set_stage(&self, stage: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(stage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'stage' property.
    pub fn maximum_stage(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumStage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Sapling<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Sapling into crate::block::data::BlockData")
    }
}
#[repr(C)]
pub struct Fence<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Fence<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Fence<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Fence from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Fence")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Fence object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Fence<'mc> {
    /// Checks if this block has the specified face enabled.
    pub fn has_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set whether this block has the specified face enabled.
    pub fn set_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        has: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(has.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFace",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get all of the faces which are enabled on this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets all of this faces which may be set on this block.
    pub fn allowed_faces(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAllowedFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::MultipleFacing<'mc>> for Fence<'mc> {
    fn into(self) -> crate::block::data::MultipleFacing<'mc> {
        crate::block::data::MultipleFacing::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Fence into crate::block::data::MultipleFacing")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Fence<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Fence into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct Light<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Light<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Light<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Light from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Light")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Light object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Light<'mc> {
    /// Gets the value of the 'level' property.
    pub fn level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'level' property.
    pub fn set_level(&self, level: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(level);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'level' property.
    pub fn maximum_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Levelled<'mc>> for Light<'mc> {
    fn into(self) -> crate::block::data::Levelled<'mc> {
        crate::block::data::Levelled::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Light into crate::block::data::Levelled")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Light<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Light into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct Jukebox<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Jukebox<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Jukebox<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Jukebox from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Jukebox")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Jukebox object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Jukebox<'mc> {
    /// Gets the value of the 'has_record' property.
    pub fn has_record(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasRecord", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Jukebox<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Jukebox into crate::block::data::BlockData")
    }
}
#[repr(C)]
pub struct Piston<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Piston<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Piston<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Piston from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Piston")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Piston object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Piston<'mc> {
    /// Gets the value of the 'extended' property.
    pub fn is_extended(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isExtended", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'extended' property.
    pub fn set_extended(&self, extended: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(extended.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExtended",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Piston<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Piston into crate::block::data::Directional")
    }
}
#[repr(C)]
pub struct HangingSign<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for HangingSign<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HangingSign<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HangingSign from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/HangingSign")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HangingSign object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HangingSign<'mc> {
    /// Gets the value of the 'attached' property.
    pub fn is_attached(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAttached", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'attached' property.
    pub fn set_attached(&self, attached: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(attached.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttached",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'rotation' property.
    pub fn rotation(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRotation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'rotation' property.
    pub fn set_rotation(
        &self,
        rotation: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Attachable<'mc>> for HangingSign<'mc> {
    fn into(self) -> crate::block::data::Attachable<'mc> {
        crate::block::data::Attachable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HangingSign into crate::block::data::Attachable")
    }
}
impl<'mc> Into<crate::block::data::Rotatable<'mc>> for HangingSign<'mc> {
    fn into(self) -> crate::block::data::Rotatable<'mc> {
        crate::block::data::Rotatable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HangingSign into crate::block::data::Rotatable")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for HangingSign<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting HangingSign into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct RedstoneRail<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RedstoneRail<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RedstoneRail<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RedstoneRail from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/RedstoneRail")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RedstoneRail object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> RedstoneRail<'mc> {
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'shape' property.
    pub fn shape(&self) -> Result<crate::block::data::RailShape<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/Rail/Shape;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getShape", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::RailShape::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'shape' property.
    pub fn set_shape(
        &self,
        shape: impl Into<crate::block::data::RailShape<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/Rail/Shape;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(shape.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setShape",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the shapes which are applicable to this block.
    pub fn shapes(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getShapes", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for RedstoneRail<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RedstoneRail into crate::block::data::Powerable")
    }
}
impl<'mc> Into<crate::block::data::Rail<'mc>> for RedstoneRail<'mc> {
    fn into(self) -> crate::block::data::Rail<'mc> {
        crate::block::data::Rail::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RedstoneRail into crate::block::data::Rail")
    }
}
#[repr(C)]
pub struct BigDripleaf<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BigDripleaf<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BigDripleaf<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BigDripleaf from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/BigDripleaf")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BigDripleaf object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BigDripleaf<'mc> {
    /// Gets the value of the 'tilt' property.
    pub fn tilt(
        &self,
    ) -> Result<crate::block::data::mod_type::BigDripleafTilt<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/BigDripleaf/Tilt;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTilt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::BigDripleafTilt::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'tilt' property.
    pub fn set_tilt(
        &self,
        tilt: impl Into<crate::block::data::mod_type::BigDripleafTilt<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/BigDripleaf/Tilt;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tilt.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTilt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::mod_type::Dripleaf<'mc>> for BigDripleaf<'mc> {
    fn into(self) -> crate::block::data::mod_type::Dripleaf<'mc> {
        crate::block::data::mod_type::Dripleaf::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BigDripleaf into crate::block::data::mod_type::Dripleaf")
    }
}
pub enum BigDripleafTilt<'mc> {
    None { inner: BigDripleafTiltStruct<'mc> },
    Unstable { inner: BigDripleafTiltStruct<'mc> },
    Partial { inner: BigDripleafTiltStruct<'mc> },
    Full { inner: BigDripleafTiltStruct<'mc> },
}
impl<'mc> std::fmt::Display for BigDripleafTilt<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BigDripleafTilt::None { .. } => f.write_str("NONE"),
            BigDripleafTilt::Unstable { .. } => f.write_str("UNSTABLE"),
            BigDripleafTilt::Partial { .. } => f.write_str("PARTIAL"),
            BigDripleafTilt::Full { .. } => f.write_str("FULL"),
        }
    }
}
impl<'mc> std::ops::Deref for BigDripleafTilt<'mc> {
    type Target = BigDripleafTiltStruct<'mc>;
    fn deref(&self) -> &<BigDripleafTilt<'mc> as std::ops::Deref>::Target {
        match self {
            BigDripleafTilt::None { inner } => inner,
            BigDripleafTilt::Unstable { inner } => inner,
            BigDripleafTilt::Partial { inner } => inner,
            BigDripleafTilt::Full { inner } => inner,
        }
    }
}

impl<'mc> BigDripleafTilt<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BigDripleafTilt<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/BigDripleaf/Tilt");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/BigDripleaf/Tilt;",
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
            "NONE" => Ok(BigDripleafTilt::None {
                inner: BigDripleafTiltStruct::from_raw(env, obj)?,
            }),
            "UNSTABLE" => Ok(BigDripleafTilt::Unstable {
                inner: BigDripleafTiltStruct::from_raw(env, obj)?,
            }),
            "PARTIAL" => Ok(BigDripleafTilt::Partial {
                inner: BigDripleafTiltStruct::from_raw(env, obj)?,
            }),
            "FULL" => Ok(BigDripleafTilt::Full {
                inner: BigDripleafTiltStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct BigDripleafTiltStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BigDripleafTilt<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::None { inner } => inner.0.clone(),
            Self::Unstable { inner } => inner.0.clone(),
            Self::Partial { inner } => inner.0.clone(),
            Self::Full { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::None { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unstable { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Partial { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Full { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BigDripleafTilt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BigDripleafTilt from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/BigDripleaf/Tilt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BigDripleafTilt object, got {}",
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
                "NONE" => Ok(BigDripleafTilt::None {
                    inner: BigDripleafTiltStruct::from_raw(env, obj)?,
                }),
                "UNSTABLE" => Ok(BigDripleafTilt::Unstable {
                    inner: BigDripleafTiltStruct::from_raw(env, obj)?,
                }),
                "PARTIAL" => Ok(BigDripleafTilt::Partial {
                    inner: BigDripleafTiltStruct::from_raw(env, obj)?,
                }),
                "FULL" => Ok(BigDripleafTilt::Full {
                    inner: BigDripleafTiltStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for BigDripleafTiltStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BigDripleafTiltStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BigDripleafTiltStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/BigDripleaf/Tilt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BigDripleafTiltStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BigDripleafTiltStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::BigDripleafTilt<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/BigDripleaf/Tilt;");
        let cls = jni.find_class("org/bukkit/block/data/type/BigDripleaf/Tilt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::BigDripleafTilt::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct Candle<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Candle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Candle<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Candle from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Candle")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Candle object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Candle<'mc> {
    /// Gets the value of the 'candles' property.
    pub fn candles(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCandles", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'candles' property.
    pub fn set_candles(&self, candles: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(candles);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCandles",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'candles' property.
    pub fn maximum_candles(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumCandles",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'lit' property.
    pub fn is_lit(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLit", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'lit' property.
    pub fn set_lit(&self, lit: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(lit.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLit",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Lightable<'mc>> for Candle<'mc> {
    fn into(self) -> crate::block::data::Lightable<'mc> {
        crate::block::data::Lightable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Candle into crate::block::data::Lightable")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Candle<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Candle into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct Stairs<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Stairs<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Stairs<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Stairs from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Stairs")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Stairs object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Stairs<'mc> {
    /// Gets the value of the 'shape' property.
    pub fn shape(
        &self,
    ) -> Result<crate::block::data::mod_type::StairsShape<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Stairs/Shape;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getShape", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::StairsShape::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'shape' property.
    pub fn set_shape(
        &self,
        shape: impl Into<crate::block::data::mod_type::StairsShape<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Stairs/Shape;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(shape.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setShape",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'half' property.
    pub fn half(
        &self,
    ) -> Result<crate::block::data::BisectedHalf<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/Bisected/Half;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHalf", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BisectedHalf::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'half' property.
    pub fn set_half(
        &self,
        half: impl Into<crate::block::data::BisectedHalf<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/Bisected/Half;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(half.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHalf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Bisected<'mc>> for Stairs<'mc> {
    fn into(self) -> crate::block::data::Bisected<'mc> {
        crate::block::data::Bisected::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Stairs into crate::block::data::Bisected")
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Stairs<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Stairs into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Stairs<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Stairs into crate::block::data::Waterlogged")
    }
}
pub enum StairsShape<'mc> {
    Straight { inner: StairsShapeStruct<'mc> },
    InnerLeft { inner: StairsShapeStruct<'mc> },
    InnerRight { inner: StairsShapeStruct<'mc> },
    OuterLeft { inner: StairsShapeStruct<'mc> },
    OuterRight { inner: StairsShapeStruct<'mc> },
}
impl<'mc> std::fmt::Display for StairsShape<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StairsShape::Straight { .. } => f.write_str("STRAIGHT"),
            StairsShape::InnerLeft { .. } => f.write_str("INNER_LEFT"),
            StairsShape::InnerRight { .. } => f.write_str("INNER_RIGHT"),
            StairsShape::OuterLeft { .. } => f.write_str("OUTER_LEFT"),
            StairsShape::OuterRight { .. } => f.write_str("OUTER_RIGHT"),
        }
    }
}
impl<'mc> std::ops::Deref for StairsShape<'mc> {
    type Target = StairsShapeStruct<'mc>;
    fn deref(&self) -> &<StairsShape<'mc> as std::ops::Deref>::Target {
        match self {
            StairsShape::Straight { inner } => inner,
            StairsShape::InnerLeft { inner } => inner,
            StairsShape::InnerRight { inner } => inner,
            StairsShape::OuterLeft { inner } => inner,
            StairsShape::OuterRight { inner } => inner,
        }
    }
}

impl<'mc> StairsShape<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<StairsShape<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Stairs/Shape");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Stairs/Shape;",
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
            "STRAIGHT" => Ok(StairsShape::Straight {
                inner: StairsShapeStruct::from_raw(env, obj)?,
            }),
            "INNER_LEFT" => Ok(StairsShape::InnerLeft {
                inner: StairsShapeStruct::from_raw(env, obj)?,
            }),
            "INNER_RIGHT" => Ok(StairsShape::InnerRight {
                inner: StairsShapeStruct::from_raw(env, obj)?,
            }),
            "OUTER_LEFT" => Ok(StairsShape::OuterLeft {
                inner: StairsShapeStruct::from_raw(env, obj)?,
            }),
            "OUTER_RIGHT" => Ok(StairsShape::OuterRight {
                inner: StairsShapeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct StairsShapeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StairsShape<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Straight { inner } => inner.0.clone(),
            Self::InnerLeft { inner } => inner.0.clone(),
            Self::InnerRight { inner } => inner.0.clone(),
            Self::OuterLeft { inner } => inner.0.clone(),
            Self::OuterRight { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Straight { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::InnerLeft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::InnerRight { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OuterLeft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OuterRight { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StairsShape<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate StairsShape from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Stairs/Shape")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StairsShape object, got {}",
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
                "STRAIGHT" => Ok(StairsShape::Straight {
                    inner: StairsShapeStruct::from_raw(env, obj)?,
                }),
                "INNER_LEFT" => Ok(StairsShape::InnerLeft {
                    inner: StairsShapeStruct::from_raw(env, obj)?,
                }),
                "INNER_RIGHT" => Ok(StairsShape::InnerRight {
                    inner: StairsShapeStruct::from_raw(env, obj)?,
                }),
                "OUTER_LEFT" => Ok(StairsShape::OuterLeft {
                    inner: StairsShapeStruct::from_raw(env, obj)?,
                }),
                "OUTER_RIGHT" => Ok(StairsShape::OuterRight {
                    inner: StairsShapeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for StairsShapeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StairsShapeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StairsShapeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Stairs/Shape")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StairsShapeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> StairsShapeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::StairsShape<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Stairs/Shape;");
        let cls = jni.find_class("org/bukkit/block/data/type/Stairs/Shape");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::StairsShape::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct Crafter<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Crafter<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Crafter<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Crafter from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Crafter")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Crafter object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Crafter<'mc> {
    /// Gets the value of the 'crafting' property.
    pub fn is_crafting(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCrafting", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'crafting' property.
    pub fn set_crafting(&self, crafting: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(crafting.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCrafting",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'triggered' property.
    pub fn is_triggered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isTriggered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'triggered' property.
    pub fn set_triggered(&self, triggered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(triggered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTriggered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'orientation' property.
    pub fn orientation(
        &self,
    ) -> Result<crate::block::data::mod_type::CrafterOrientation<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/Crafter/Orientation;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOrientation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::CrafterOrientation::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'orientation' property.
    pub fn set_orientation(
        &self,
        orientation: impl Into<crate::block::data::mod_type::CrafterOrientation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Crafter/Orientation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(orientation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOrientation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Crafter<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Crafter into crate::block::data::BlockData")
    }
}
pub enum CrafterOrientation<'mc> {
    DownEast {
        inner: CrafterOrientationStruct<'mc>,
    },
    DownNorth {
        inner: CrafterOrientationStruct<'mc>,
    },
    DownSouth {
        inner: CrafterOrientationStruct<'mc>,
    },
    DownWest {
        inner: CrafterOrientationStruct<'mc>,
    },
    UpEast {
        inner: CrafterOrientationStruct<'mc>,
    },
    UpNorth {
        inner: CrafterOrientationStruct<'mc>,
    },
    UpSouth {
        inner: CrafterOrientationStruct<'mc>,
    },
    UpWest {
        inner: CrafterOrientationStruct<'mc>,
    },
    WestUp {
        inner: CrafterOrientationStruct<'mc>,
    },
    EastUp {
        inner: CrafterOrientationStruct<'mc>,
    },
    NorthUp {
        inner: CrafterOrientationStruct<'mc>,
    },
    SouthUp {
        inner: CrafterOrientationStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for CrafterOrientation<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrafterOrientation::DownEast { .. } => f.write_str("DOWN_EAST"),
            CrafterOrientation::DownNorth { .. } => f.write_str("DOWN_NORTH"),
            CrafterOrientation::DownSouth { .. } => f.write_str("DOWN_SOUTH"),
            CrafterOrientation::DownWest { .. } => f.write_str("DOWN_WEST"),
            CrafterOrientation::UpEast { .. } => f.write_str("UP_EAST"),
            CrafterOrientation::UpNorth { .. } => f.write_str("UP_NORTH"),
            CrafterOrientation::UpSouth { .. } => f.write_str("UP_SOUTH"),
            CrafterOrientation::UpWest { .. } => f.write_str("UP_WEST"),
            CrafterOrientation::WestUp { .. } => f.write_str("WEST_UP"),
            CrafterOrientation::EastUp { .. } => f.write_str("EAST_UP"),
            CrafterOrientation::NorthUp { .. } => f.write_str("NORTH_UP"),
            CrafterOrientation::SouthUp { .. } => f.write_str("SOUTH_UP"),
        }
    }
}
impl<'mc> std::ops::Deref for CrafterOrientation<'mc> {
    type Target = CrafterOrientationStruct<'mc>;
    fn deref(&self) -> &<CrafterOrientation<'mc> as std::ops::Deref>::Target {
        match self {
            CrafterOrientation::DownEast { inner } => inner,
            CrafterOrientation::DownNorth { inner } => inner,
            CrafterOrientation::DownSouth { inner } => inner,
            CrafterOrientation::DownWest { inner } => inner,
            CrafterOrientation::UpEast { inner } => inner,
            CrafterOrientation::UpNorth { inner } => inner,
            CrafterOrientation::UpSouth { inner } => inner,
            CrafterOrientation::UpWest { inner } => inner,
            CrafterOrientation::WestUp { inner } => inner,
            CrafterOrientation::EastUp { inner } => inner,
            CrafterOrientation::NorthUp { inner } => inner,
            CrafterOrientation::SouthUp { inner } => inner,
        }
    }
}

impl<'mc> CrafterOrientation<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<CrafterOrientation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Crafter/Orientation");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Crafter/Orientation;",
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
            "DOWN_EAST" => Ok(CrafterOrientation::DownEast {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),
            "DOWN_NORTH" => Ok(CrafterOrientation::DownNorth {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),
            "DOWN_SOUTH" => Ok(CrafterOrientation::DownSouth {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),
            "DOWN_WEST" => Ok(CrafterOrientation::DownWest {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),
            "UP_EAST" => Ok(CrafterOrientation::UpEast {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),
            "UP_NORTH" => Ok(CrafterOrientation::UpNorth {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),
            "UP_SOUTH" => Ok(CrafterOrientation::UpSouth {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),
            "UP_WEST" => Ok(CrafterOrientation::UpWest {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),
            "WEST_UP" => Ok(CrafterOrientation::WestUp {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),
            "EAST_UP" => Ok(CrafterOrientation::EastUp {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),
            "NORTH_UP" => Ok(CrafterOrientation::NorthUp {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),
            "SOUTH_UP" => Ok(CrafterOrientation::SouthUp {
                inner: CrafterOrientationStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct CrafterOrientationStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CrafterOrientation<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::DownEast { inner } => inner.0.clone(),
            Self::DownNorth { inner } => inner.0.clone(),
            Self::DownSouth { inner } => inner.0.clone(),
            Self::DownWest { inner } => inner.0.clone(),
            Self::UpEast { inner } => inner.0.clone(),
            Self::UpNorth { inner } => inner.0.clone(),
            Self::UpSouth { inner } => inner.0.clone(),
            Self::UpWest { inner } => inner.0.clone(),
            Self::WestUp { inner } => inner.0.clone(),
            Self::EastUp { inner } => inner.0.clone(),
            Self::NorthUp { inner } => inner.0.clone(),
            Self::SouthUp { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::DownEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::DownNorth { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DownSouth { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DownWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::UpEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::UpNorth { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::UpSouth { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::UpWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WestUp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EastUp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::NorthUp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SouthUp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CrafterOrientation<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CrafterOrientation from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/Crafter/Orientation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CrafterOrientation object, got {}",
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
                "DOWN_EAST" => Ok(CrafterOrientation::DownEast {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                "DOWN_NORTH" => Ok(CrafterOrientation::DownNorth {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                "DOWN_SOUTH" => Ok(CrafterOrientation::DownSouth {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                "DOWN_WEST" => Ok(CrafterOrientation::DownWest {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                "UP_EAST" => Ok(CrafterOrientation::UpEast {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                "UP_NORTH" => Ok(CrafterOrientation::UpNorth {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                "UP_SOUTH" => Ok(CrafterOrientation::UpSouth {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                "UP_WEST" => Ok(CrafterOrientation::UpWest {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                "WEST_UP" => Ok(CrafterOrientation::WestUp {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                "EAST_UP" => Ok(CrafterOrientation::EastUp {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                "NORTH_UP" => Ok(CrafterOrientation::NorthUp {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                "SOUTH_UP" => Ok(CrafterOrientation::SouthUp {
                    inner: CrafterOrientationStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for CrafterOrientationStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CrafterOrientationStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CrafterOrientationStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/Crafter/Orientation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CrafterOrientationStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CrafterOrientationStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::CrafterOrientation<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/Crafter/Orientation;");
        let cls = jni.find_class("org/bukkit/block/data/type/Crafter/Orientation");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::CrafterOrientation::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct WallSign<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for WallSign<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for WallSign<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WallSign from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/WallSign")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WallSign object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> WallSign<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for WallSign<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WallSign into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for WallSign<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WallSign into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct SeaPickle<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SeaPickle<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SeaPickle<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SeaPickle from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/SeaPickle")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SeaPickle object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SeaPickle<'mc> {
    /// Gets the value of the 'pickles' property.
    pub fn pickles(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPickles", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'pickles' property.
    pub fn set_pickles(&self, pickles: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(pickles);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPickles",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the minimum allowed value of the 'pickles' property.
    pub fn minimum_pickles(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMinimumPickles",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the maximum allowed value of the 'pickles' property.
    pub fn maximum_pickles(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumPickles",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for SeaPickle<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SeaPickle into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct Vault<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Vault<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Vault<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Vault from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Vault")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Vault object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Vault<'mc> {
    /// Gets the value of the 'vault_state' property.
    pub fn trial_spawner_state(
        &self,
    ) -> Result<crate::block::data::mod_type::VaultState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Vault/State;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTrialSpawnerState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::VaultState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'vault_state' property.
    pub fn set_trial_spawner_state(
        &self,
        state: impl Into<crate::block::data::mod_type::VaultState<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Vault/State;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(state.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTrialSpawnerState",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'ominous' property.
    pub fn is_ominous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOminous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'ominous' property.
    pub fn set_ominous(&self, ominous: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(ominous.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOminous",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Vault<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Vault into crate::block::data::Directional")
    }
}
pub enum VaultState<'mc> {
    Inactive { inner: VaultStateStruct<'mc> },
    Active { inner: VaultStateStruct<'mc> },
    Unlocking { inner: VaultStateStruct<'mc> },
    Ejecting { inner: VaultStateStruct<'mc> },
}
impl<'mc> std::fmt::Display for VaultState<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaultState::Inactive { .. } => f.write_str("INACTIVE"),
            VaultState::Active { .. } => f.write_str("ACTIVE"),
            VaultState::Unlocking { .. } => f.write_str("UNLOCKING"),
            VaultState::Ejecting { .. } => f.write_str("EJECTING"),
        }
    }
}
impl<'mc> std::ops::Deref for VaultState<'mc> {
    type Target = VaultStateStruct<'mc>;
    fn deref(&self) -> &<VaultState<'mc> as std::ops::Deref>::Target {
        match self {
            VaultState::Inactive { inner } => inner,
            VaultState::Active { inner } => inner,
            VaultState::Unlocking { inner } => inner,
            VaultState::Ejecting { inner } => inner,
        }
    }
}

impl<'mc> VaultState<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<VaultState<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Vault/State");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Vault/State;",
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
            "INACTIVE" => Ok(VaultState::Inactive {
                inner: VaultStateStruct::from_raw(env, obj)?,
            }),
            "ACTIVE" => Ok(VaultState::Active {
                inner: VaultStateStruct::from_raw(env, obj)?,
            }),
            "UNLOCKING" => Ok(VaultState::Unlocking {
                inner: VaultStateStruct::from_raw(env, obj)?,
            }),
            "EJECTING" => Ok(VaultState::Ejecting {
                inner: VaultStateStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct VaultStateStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for VaultState<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Inactive { inner } => inner.0.clone(),
            Self::Active { inner } => inner.0.clone(),
            Self::Unlocking { inner } => inner.0.clone(),
            Self::Ejecting { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Inactive { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Active { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Unlocking { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Ejecting { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VaultState<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate VaultState from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Vault/State")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VaultState object, got {}",
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
                "INACTIVE" => Ok(VaultState::Inactive {
                    inner: VaultStateStruct::from_raw(env, obj)?,
                }),
                "ACTIVE" => Ok(VaultState::Active {
                    inner: VaultStateStruct::from_raw(env, obj)?,
                }),
                "UNLOCKING" => Ok(VaultState::Unlocking {
                    inner: VaultStateStruct::from_raw(env, obj)?,
                }),
                "EJECTING" => Ok(VaultState::Ejecting {
                    inner: VaultStateStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for VaultStateStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for VaultStateStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate VaultStateStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Vault/State")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a VaultStateStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> VaultStateStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::VaultState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Vault/State;");
        let cls = jni.find_class("org/bukkit/block/data/type/Vault/State");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::VaultState::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct SculkSensor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SculkSensor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SculkSensor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SculkSensor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/SculkSensor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SculkSensor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SculkSensor<'mc> {
    /// Gets the value of the 'sculk_sensor_phase' property.
    pub fn phase(
        &self,
    ) -> Result<crate::block::data::mod_type::SculkSensorPhase<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/SculkSensor/Phase;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPhase", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::SculkSensorPhase::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'sculk_sensor_phase' property.
    pub fn set_phase(
        &self,
        phase: impl Into<crate::block::data::mod_type::SculkSensorPhase<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/SculkSensor/Phase;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(phase.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPhase",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'power' property.
    pub fn power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'power' property.
    pub fn set_power(&self, power: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(power);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'power' property.
    pub fn maximum_power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::AnaloguePowerable<'mc>> for SculkSensor<'mc> {
    fn into(self) -> crate::block::data::AnaloguePowerable<'mc> {
        crate::block::data::AnaloguePowerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SculkSensor into crate::block::data::AnaloguePowerable")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for SculkSensor<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SculkSensor into crate::block::data::Waterlogged")
    }
}
pub enum SculkSensorPhase<'mc> {
    Inactive { inner: SculkSensorPhaseStruct<'mc> },
    Active { inner: SculkSensorPhaseStruct<'mc> },
    Cooldown { inner: SculkSensorPhaseStruct<'mc> },
}
impl<'mc> std::fmt::Display for SculkSensorPhase<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SculkSensorPhase::Inactive { .. } => f.write_str("INACTIVE"),
            SculkSensorPhase::Active { .. } => f.write_str("ACTIVE"),
            SculkSensorPhase::Cooldown { .. } => f.write_str("COOLDOWN"),
        }
    }
}
impl<'mc> std::ops::Deref for SculkSensorPhase<'mc> {
    type Target = SculkSensorPhaseStruct<'mc>;
    fn deref(&self) -> &<SculkSensorPhase<'mc> as std::ops::Deref>::Target {
        match self {
            SculkSensorPhase::Inactive { inner } => inner,
            SculkSensorPhase::Active { inner } => inner,
            SculkSensorPhase::Cooldown { inner } => inner,
        }
    }
}

impl<'mc> SculkSensorPhase<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SculkSensorPhase<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/SculkSensor/Phase");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/SculkSensor/Phase;",
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
            "INACTIVE" => Ok(SculkSensorPhase::Inactive {
                inner: SculkSensorPhaseStruct::from_raw(env, obj)?,
            }),
            "ACTIVE" => Ok(SculkSensorPhase::Active {
                inner: SculkSensorPhaseStruct::from_raw(env, obj)?,
            }),
            "COOLDOWN" => Ok(SculkSensorPhase::Cooldown {
                inner: SculkSensorPhaseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct SculkSensorPhaseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SculkSensorPhase<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Inactive { inner } => inner.0.clone(),
            Self::Active { inner } => inner.0.clone(),
            Self::Cooldown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Inactive { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Active { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cooldown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SculkSensorPhase<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SculkSensorPhase from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/SculkSensor/Phase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SculkSensorPhase object, got {}",
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
                "INACTIVE" => Ok(SculkSensorPhase::Inactive {
                    inner: SculkSensorPhaseStruct::from_raw(env, obj)?,
                }),
                "ACTIVE" => Ok(SculkSensorPhase::Active {
                    inner: SculkSensorPhaseStruct::from_raw(env, obj)?,
                }),
                "COOLDOWN" => Ok(SculkSensorPhase::Cooldown {
                    inner: SculkSensorPhaseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for SculkSensorPhaseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SculkSensorPhaseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate SculkSensorPhaseStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/SculkSensor/Phase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SculkSensorPhaseStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SculkSensorPhaseStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::SculkSensorPhase<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/SculkSensor/Phase;");
        let cls = jni.find_class("org/bukkit/block/data/type/SculkSensor/Phase");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::SculkSensorPhase::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct PistonHead<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PistonHead<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PistonHead<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PistonHead from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/PistonHead")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PistonHead object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PistonHead<'mc> {
    /// Gets the value of the 'short' property.
    pub fn is_short(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isShort", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'short' property.
    pub fn set_short(&self, _short: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(_short.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setShort",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'type' property.
    pub fn get_type(
        &self,
    ) -> Result<crate::block::data::mod_type::TechnicalPistonType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/TechnicalPiston/Type;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::TechnicalPistonType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'type' property.
    pub fn set_type(
        &self,
        val_type: impl Into<crate::block::data::mod_type::TechnicalPistonType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/TechnicalPiston/Type;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::mod_type::TechnicalPiston<'mc>> for PistonHead<'mc> {
    fn into(self) -> crate::block::data::mod_type::TechnicalPiston<'mc> {
        crate::block::data::mod_type::TechnicalPiston::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting PistonHead into crate::block::data::mod_type::TechnicalPiston",
        )
    }
}
#[repr(C)]
pub struct Observer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Observer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Observer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Observer from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Observer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Observer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Observer<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Observer<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Observer into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for Observer<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Observer into crate::block::data::Powerable")
    }
}
#[repr(C)]
pub struct Dripleaf<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Dripleaf<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Dripleaf<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Dripleaf from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Dripleaf")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Dripleaf object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Dripleaf<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Dripleaf<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Dripleaf into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Dripleaf<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Dripleaf into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct Tripwire<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Tripwire<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Tripwire<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Tripwire from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Tripwire")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Tripwire object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Tripwire<'mc> {
    /// Gets the value of the 'disarmed' property.
    pub fn is_disarmed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDisarmed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'disarmed' property.
    pub fn set_disarmed(&self, disarmed: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(disarmed.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisarmed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'attached' property.
    pub fn is_attached(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAttached", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'attached' property.
    pub fn set_attached(&self, attached: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(attached.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttached",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks if this block has the specified face enabled.
    pub fn has_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set whether this block has the specified face enabled.
    pub fn set_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        has: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(has.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFace",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get all of the faces which are enabled on this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets all of this faces which may be set on this block.
    pub fn allowed_faces(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAllowedFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Attachable<'mc>> for Tripwire<'mc> {
    fn into(self) -> crate::block::data::Attachable<'mc> {
        crate::block::data::Attachable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Tripwire into crate::block::data::Attachable")
    }
}
impl<'mc> Into<crate::block::data::MultipleFacing<'mc>> for Tripwire<'mc> {
    fn into(self) -> crate::block::data::MultipleFacing<'mc> {
        crate::block::data::MultipleFacing::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Tripwire into crate::block::data::MultipleFacing")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for Tripwire<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Tripwire into crate::block::data::Powerable")
    }
}
#[repr(C)]
pub struct Repeater<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Repeater<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Repeater<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Repeater from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Repeater")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Repeater object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Repeater<'mc> {
    /// Gets the value of the 'delay' property.
    pub fn delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDelay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'delay' property.
    pub fn set_delay(&self, delay: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(delay);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDelay",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the minimum allowed value of the 'delay' property.
    pub fn minimum_delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinimumDelay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the maximum allowed value of the 'delay' property.
    pub fn maximum_delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumDelay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'locked' property.
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'locked' property.
    pub fn set_locked(&self, locked: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(locked.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocked",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Repeater<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Repeater into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for Repeater<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Repeater into crate::block::data::Powerable")
    }
}
#[repr(C)]
pub struct Wall<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Wall<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Wall<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Wall from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Wall")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Wall object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Wall<'mc> {
    /// Gets the value of the 'up' property.
    pub fn is_up(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'up' property.
    pub fn set_up(&self, up: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(up.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the height of the specified face.
    pub fn get_height(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<crate::block::data::mod_type::WallHeight<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/BlockFace;)Lorg/bukkit/block/data/type/Wall/Height;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHeight",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::WallHeight::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the height of the specified face.
    pub fn set_height(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        height: impl Into<crate::block::data::mod_type::WallHeight<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/data/type/Wall/Height;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(height.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHeight",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Wall<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Wall into crate::block::data::Waterlogged")
    }
}
pub enum WallHeight<'mc> {
    None { inner: WallHeightStruct<'mc> },
    Low { inner: WallHeightStruct<'mc> },
    Tall { inner: WallHeightStruct<'mc> },
}
impl<'mc> std::fmt::Display for WallHeight<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallHeight::None { .. } => f.write_str("NONE"),
            WallHeight::Low { .. } => f.write_str("LOW"),
            WallHeight::Tall { .. } => f.write_str("TALL"),
        }
    }
}
impl<'mc> std::ops::Deref for WallHeight<'mc> {
    type Target = WallHeightStruct<'mc>;
    fn deref(&self) -> &<WallHeight<'mc> as std::ops::Deref>::Target {
        match self {
            WallHeight::None { inner } => inner,
            WallHeight::Low { inner } => inner,
            WallHeight::Tall { inner } => inner,
        }
    }
}

impl<'mc> WallHeight<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<WallHeight<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Wall/Height");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Wall/Height;",
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
            "NONE" => Ok(WallHeight::None {
                inner: WallHeightStruct::from_raw(env, obj)?,
            }),
            "LOW" => Ok(WallHeight::Low {
                inner: WallHeightStruct::from_raw(env, obj)?,
            }),
            "TALL" => Ok(WallHeight::Tall {
                inner: WallHeightStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct WallHeightStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for WallHeight<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::None { inner } => inner.0.clone(),
            Self::Low { inner } => inner.0.clone(),
            Self::Tall { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::None { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Low { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Tall { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for WallHeight<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WallHeight from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Wall/Height")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WallHeight object, got {}",
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
                "NONE" => Ok(WallHeight::None {
                    inner: WallHeightStruct::from_raw(env, obj)?,
                }),
                "LOW" => Ok(WallHeight::Low {
                    inner: WallHeightStruct::from_raw(env, obj)?,
                }),
                "TALL" => Ok(WallHeight::Tall {
                    inner: WallHeightStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for WallHeightStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for WallHeightStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate WallHeightStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Wall/Height")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WallHeightStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> WallHeightStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::WallHeight<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Wall/Height;");
        let cls = jni.find_class("org/bukkit/block/data/type/Wall/Height");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::WallHeight::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct MangrovePropagule<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MangrovePropagule<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MangrovePropagule<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MangrovePropagule from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/MangrovePropagule")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MangrovePropagule object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MangrovePropagule<'mc> {
    /// Gets the value of the 'age' property.
    pub fn age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'age' property.
    pub fn set_age(&self, age: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(age);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'age' property.
    pub fn maximum_age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'hanging' property.
    pub fn is_hanging(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isHanging", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'hanging' property.
    pub fn set_hanging(&self, hanging: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(hanging.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHanging",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'stage' property.
    pub fn stage(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getStage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'stage' property.
    pub fn set_stage(&self, stage: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(stage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'stage' property.
    pub fn maximum_stage(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumStage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Ageable<'mc>> for MangrovePropagule<'mc> {
    fn into(self) -> crate::block::data::Ageable<'mc> {
        crate::block::data::Ageable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MangrovePropagule into crate::block::data::Ageable")
    }
}
impl<'mc> Into<crate::block::data::Hangable<'mc>> for MangrovePropagule<'mc> {
    fn into(self) -> crate::block::data::Hangable<'mc> {
        crate::block::data::Hangable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MangrovePropagule into crate::block::data::Hangable")
    }
}
impl<'mc> Into<crate::block::data::mod_type::Sapling<'mc>> for MangrovePropagule<'mc> {
    fn into(self) -> crate::block::data::mod_type::Sapling<'mc> {
        crate::block::data::mod_type::Sapling::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MangrovePropagule into crate::block::data::mod_type::Sapling")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for MangrovePropagule<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MangrovePropagule into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct PointedDripstone<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PointedDripstone<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PointedDripstone<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PointedDripstone from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/PointedDripstone")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PointedDripstone object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PointedDripstone<'mc> {
    /// Gets the value of the 'vertical_direction' property.
    pub fn vertical_direction(
        &self,
    ) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVerticalDirection",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'vertical_direction' property.
    pub fn set_vertical_direction(
        &self,
        direction: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(direction.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVerticalDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn vertical_directions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVerticalDirections",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'thickness' property.
    pub fn thickness(
        &self,
    ) -> Result<
        crate::block::data::mod_type::PointedDripstoneThickness<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/block/data/type/PointedDripstone/Thickness;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getThickness", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::PointedDripstoneThickness::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'thickness' property.
    pub fn set_thickness(
        &self,
        thickness: impl Into<crate::block::data::mod_type::PointedDripstoneThickness<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/PointedDripstone/Thickness;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(thickness.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setThickness",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for PointedDripstone<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PointedDripstone into crate::block::data::Waterlogged")
    }
}
pub enum PointedDripstoneThickness<'mc> {
    TipMerge {
        inner: PointedDripstoneThicknessStruct<'mc>,
    },
    Tip {
        inner: PointedDripstoneThicknessStruct<'mc>,
    },
    Frustum {
        inner: PointedDripstoneThicknessStruct<'mc>,
    },
    Middle {
        inner: PointedDripstoneThicknessStruct<'mc>,
    },
    Base {
        inner: PointedDripstoneThicknessStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for PointedDripstoneThickness<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PointedDripstoneThickness::TipMerge { .. } => f.write_str("TIP_MERGE"),
            PointedDripstoneThickness::Tip { .. } => f.write_str("TIP"),
            PointedDripstoneThickness::Frustum { .. } => f.write_str("FRUSTUM"),
            PointedDripstoneThickness::Middle { .. } => f.write_str("MIDDLE"),
            PointedDripstoneThickness::Base { .. } => f.write_str("BASE"),
        }
    }
}
impl<'mc> std::ops::Deref for PointedDripstoneThickness<'mc> {
    type Target = PointedDripstoneThicknessStruct<'mc>;
    fn deref(&self) -> &<PointedDripstoneThickness<'mc> as std::ops::Deref>::Target {
        match self {
            PointedDripstoneThickness::TipMerge { inner } => inner,
            PointedDripstoneThickness::Tip { inner } => inner,
            PointedDripstoneThickness::Frustum { inner } => inner,
            PointedDripstoneThickness::Middle { inner } => inner,
            PointedDripstoneThickness::Base { inner } => inner,
        }
    }
}

impl<'mc> PointedDripstoneThickness<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PointedDripstoneThickness<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/PointedDripstone/Thickness");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/PointedDripstone/Thickness;",
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
            "TIP_MERGE" => Ok(PointedDripstoneThickness::TipMerge {
                inner: PointedDripstoneThicknessStruct::from_raw(env, obj)?,
            }),
            "TIP" => Ok(PointedDripstoneThickness::Tip {
                inner: PointedDripstoneThicknessStruct::from_raw(env, obj)?,
            }),
            "FRUSTUM" => Ok(PointedDripstoneThickness::Frustum {
                inner: PointedDripstoneThicknessStruct::from_raw(env, obj)?,
            }),
            "MIDDLE" => Ok(PointedDripstoneThickness::Middle {
                inner: PointedDripstoneThicknessStruct::from_raw(env, obj)?,
            }),
            "BASE" => Ok(PointedDripstoneThickness::Base {
                inner: PointedDripstoneThicknessStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PointedDripstoneThicknessStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PointedDripstoneThickness<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::TipMerge { inner } => inner.0.clone(),
            Self::Tip { inner } => inner.0.clone(),
            Self::Frustum { inner } => inner.0.clone(),
            Self::Middle { inner } => inner.0.clone(),
            Self::Base { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::TipMerge { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Tip { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Frustum { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Middle { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Base { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PointedDripstoneThickness<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PointedDripstoneThickness from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/block/data/type/PointedDripstone/Thickness",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PointedDripstoneThickness object, got {}",
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
                "TIP_MERGE" => Ok(PointedDripstoneThickness::TipMerge {
                    inner: PointedDripstoneThicknessStruct::from_raw(env, obj)?,
                }),
                "TIP" => Ok(PointedDripstoneThickness::Tip {
                    inner: PointedDripstoneThicknessStruct::from_raw(env, obj)?,
                }),
                "FRUSTUM" => Ok(PointedDripstoneThickness::Frustum {
                    inner: PointedDripstoneThicknessStruct::from_raw(env, obj)?,
                }),
                "MIDDLE" => Ok(PointedDripstoneThickness::Middle {
                    inner: PointedDripstoneThicknessStruct::from_raw(env, obj)?,
                }),
                "BASE" => Ok(PointedDripstoneThickness::Base {
                    inner: PointedDripstoneThicknessStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PointedDripstoneThicknessStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PointedDripstoneThicknessStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PointedDripstoneThicknessStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/block/data/type/PointedDripstone/Thickness",
        )?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a PointedDripstoneThicknessStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PointedDripstoneThicknessStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<
        crate::block::data::mod_type::PointedDripstoneThickness<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/block/data/type/PointedDripstone/Thickness;");
        let cls = jni.find_class("org/bukkit/block/data/type/PointedDripstone/Thickness");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::PointedDripstoneThickness::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct GlassPane<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for GlassPane<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for GlassPane<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate GlassPane from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/GlassPane")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a GlassPane object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> GlassPane<'mc> {
    /// Checks if this block has the specified face enabled.
    pub fn has_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set whether this block has the specified face enabled.
    pub fn set_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        has: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(has.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFace",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get all of the faces which are enabled on this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets all of this faces which may be set on this block.
    pub fn allowed_faces(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAllowedFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::MultipleFacing<'mc>> for GlassPane<'mc> {
    fn into(self) -> crate::block::data::MultipleFacing<'mc> {
        crate::block::data::MultipleFacing::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting GlassPane into crate::block::data::MultipleFacing")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for GlassPane<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting GlassPane into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct Chain<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Chain<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Chain<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Chain from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Chain")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Chain object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Chain<'mc> {
    /// Gets the value of the 'axis' property.
    pub fn axis(&self) -> Result<crate::Axis<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Axis;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAxis", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Axis::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'axis' property.
    pub fn set_axis(
        &self,
        axis: impl Into<crate::Axis<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Axis;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(axis.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAxis",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the axes which are applicable to this block.
    pub fn axes(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAxes", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Orientable<'mc>> for Chain<'mc> {
    fn into(self) -> crate::block::data::Orientable<'mc> {
        crate::block::data::Orientable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Chain into crate::block::data::Orientable")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Chain<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Chain into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct RedstoneWire<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RedstoneWire<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RedstoneWire<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RedstoneWire from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/RedstoneWire")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RedstoneWire object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> RedstoneWire<'mc> {
    /// Checks the type of connection on the specified face.
    pub fn get_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<crate::block::data::mod_type::RedstoneWireConnection<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Lorg/bukkit/block/BlockFace;)Lorg/bukkit/block/data/type/RedstoneWire/Connection;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::RedstoneWireConnection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the type of connection on the specified face.
    pub fn set_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        connection: impl Into<crate::block::data::mod_type::RedstoneWireConnection<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/data/type/RedstoneWire/Connection;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(connection.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFace",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets all of this faces which may be set on this block.
    pub fn allowed_faces(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAllowedFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'power' property.
    pub fn power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'power' property.
    pub fn set_power(&self, power: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(power);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'power' property.
    pub fn maximum_power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::AnaloguePowerable<'mc>> for RedstoneWire<'mc> {
    fn into(self) -> crate::block::data::AnaloguePowerable<'mc> {
        crate::block::data::AnaloguePowerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RedstoneWire into crate::block::data::AnaloguePowerable")
    }
}
pub enum RedstoneWireConnection<'mc> {
    Up {
        inner: RedstoneWireConnectionStruct<'mc>,
    },
    Side {
        inner: RedstoneWireConnectionStruct<'mc>,
    },
    None {
        inner: RedstoneWireConnectionStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for RedstoneWireConnection<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedstoneWireConnection::Up { .. } => f.write_str("UP"),
            RedstoneWireConnection::Side { .. } => f.write_str("SIDE"),
            RedstoneWireConnection::None { .. } => f.write_str("NONE"),
        }
    }
}
impl<'mc> std::ops::Deref for RedstoneWireConnection<'mc> {
    type Target = RedstoneWireConnectionStruct<'mc>;
    fn deref(&self) -> &<RedstoneWireConnection<'mc> as std::ops::Deref>::Target {
        match self {
            RedstoneWireConnection::Up { inner } => inner,
            RedstoneWireConnection::Side { inner } => inner,
            RedstoneWireConnection::None { inner } => inner,
        }
    }
}

impl<'mc> RedstoneWireConnection<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<RedstoneWireConnection<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/RedstoneWire/Connection");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/RedstoneWire/Connection;",
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
            "UP" => Ok(RedstoneWireConnection::Up {
                inner: RedstoneWireConnectionStruct::from_raw(env, obj)?,
            }),
            "SIDE" => Ok(RedstoneWireConnection::Side {
                inner: RedstoneWireConnectionStruct::from_raw(env, obj)?,
            }),
            "NONE" => Ok(RedstoneWireConnection::None {
                inner: RedstoneWireConnectionStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct RedstoneWireConnectionStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RedstoneWireConnection<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Up { inner } => inner.0.clone(),
            Self::Side { inner } => inner.0.clone(),
            Self::None { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Up { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Side { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::None { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RedstoneWireConnection<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate RedstoneWireConnection from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/RedstoneWire/Connection")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RedstoneWireConnection object, got {}",
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
                "UP" => Ok(RedstoneWireConnection::Up {
                    inner: RedstoneWireConnectionStruct::from_raw(env, obj)?,
                }),
                "SIDE" => Ok(RedstoneWireConnection::Side {
                    inner: RedstoneWireConnectionStruct::from_raw(env, obj)?,
                }),
                "NONE" => Ok(RedstoneWireConnection::None {
                    inner: RedstoneWireConnectionStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for RedstoneWireConnectionStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RedstoneWireConnectionStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate RedstoneWireConnectionStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/RedstoneWire/Connection")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RedstoneWireConnectionStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> RedstoneWireConnectionStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::RedstoneWireConnection<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/RedstoneWire/Connection;");
        let cls = jni.find_class("org/bukkit/block/data/type/RedstoneWire/Connection");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::RedstoneWireConnection::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct CoralWallFan<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CoralWallFan<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CoralWallFan<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CoralWallFan from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/CoralWallFan")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CoralWallFan object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CoralWallFan<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for CoralWallFan<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CoralWallFan into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for CoralWallFan<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CoralWallFan into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct SmallDripleaf<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SmallDripleaf<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SmallDripleaf<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SmallDripleaf from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/SmallDripleaf")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SmallDripleaf object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SmallDripleaf<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'half' property.
    pub fn half(
        &self,
    ) -> Result<crate::block::data::BisectedHalf<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/Bisected/Half;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHalf", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BisectedHalf::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'half' property.
    pub fn set_half(
        &self,
        half: impl Into<crate::block::data::BisectedHalf<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/Bisected/Half;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(half.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHalf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::mod_type::Dripleaf<'mc>> for SmallDripleaf<'mc> {
    fn into(self) -> crate::block::data::mod_type::Dripleaf<'mc> {
        crate::block::data::mod_type::Dripleaf::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SmallDripleaf into crate::block::data::mod_type::Dripleaf")
    }
}
impl<'mc> Into<crate::block::data::Bisected<'mc>> for SmallDripleaf<'mc> {
    fn into(self) -> crate::block::data::Bisected<'mc> {
        crate::block::data::Bisected::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SmallDripleaf into crate::block::data::Bisected")
    }
}
#[repr(C)]
pub struct CalibratedSculkSensor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CalibratedSculkSensor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CalibratedSculkSensor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate CalibratedSculkSensor from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/CalibratedSculkSensor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CalibratedSculkSensor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CalibratedSculkSensor<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'sculk_sensor_phase' property.
    pub fn phase(
        &self,
    ) -> Result<crate::block::data::mod_type::SculkSensorPhase<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/SculkSensor/Phase;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPhase", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::SculkSensorPhase::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'sculk_sensor_phase' property.
    pub fn set_phase(
        &self,
        phase: impl Into<crate::block::data::mod_type::SculkSensorPhase<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/SculkSensor/Phase;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(phase.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPhase",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'power' property.
    pub fn power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'power' property.
    pub fn set_power(&self, power: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(power);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'power' property.
    pub fn maximum_power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for CalibratedSculkSensor<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CalibratedSculkSensor into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::mod_type::SculkSensor<'mc>> for CalibratedSculkSensor<'mc> {
    fn into(self) -> crate::block::data::mod_type::SculkSensor<'mc> {
        crate::block::data::mod_type::SculkSensor::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting CalibratedSculkSensor into crate::block::data::mod_type::SculkSensor",
        )
    }
}
#[repr(C)]
pub struct Lantern<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Lantern<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Lantern<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Lantern from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Lantern")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Lantern object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Lantern<'mc> {
    /// Gets the value of the 'hanging' property.
    pub fn is_hanging(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isHanging", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'hanging' property.
    pub fn set_hanging(&self, hanging: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(hanging.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHanging",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Hangable<'mc>> for Lantern<'mc> {
    fn into(self) -> crate::block::data::Hangable<'mc> {
        crate::block::data::Hangable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Lantern into crate::block::data::Hangable")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Lantern<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Lantern into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct DaylightDetector<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DaylightDetector<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DaylightDetector<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DaylightDetector from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/DaylightDetector")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DaylightDetector object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DaylightDetector<'mc> {
    /// Gets the value of the 'inverted' property.
    pub fn is_inverted(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInverted", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'inverted' property.
    pub fn set_inverted(&self, inverted: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(inverted.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInverted",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'power' property.
    pub fn power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'power' property.
    pub fn set_power(&self, power: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(power);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'power' property.
    pub fn maximum_power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::AnaloguePowerable<'mc>> for DaylightDetector<'mc> {
    fn into(self) -> crate::block::data::AnaloguePowerable<'mc> {
        crate::block::data::AnaloguePowerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DaylightDetector into crate::block::data::AnaloguePowerable")
    }
}
#[repr(C)]
pub struct Door<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Door<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Door<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Door from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Door")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Door object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Door<'mc> {
    /// Gets the value of the 'hinge' property.
    pub fn hinge(
        &self,
    ) -> Result<crate::block::data::mod_type::DoorHinge<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Door/Hinge;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHinge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::DoorHinge::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'hinge' property.
    pub fn set_hinge(
        &self,
        hinge: impl Into<crate::block::data::mod_type::DoorHinge<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Door/Hinge;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(hinge.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHinge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'half' property.
    pub fn half(
        &self,
    ) -> Result<crate::block::data::BisectedHalf<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/Bisected/Half;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHalf", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BisectedHalf::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'half' property.
    pub fn set_half(
        &self,
        half: impl Into<crate::block::data::BisectedHalf<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/Bisected/Half;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(half.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHalf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'open' property.
    pub fn is_open(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'open' property.
    pub fn set_open(&self, open: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(open.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Bisected<'mc>> for Door<'mc> {
    fn into(self) -> crate::block::data::Bisected<'mc> {
        crate::block::data::Bisected::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Door into crate::block::data::Bisected")
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Door<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Door into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Openable<'mc>> for Door<'mc> {
    fn into(self) -> crate::block::data::Openable<'mc> {
        crate::block::data::Openable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Door into crate::block::data::Openable")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for Door<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Door into crate::block::data::Powerable")
    }
}
pub enum DoorHinge<'mc> {
    Left { inner: DoorHingeStruct<'mc> },
    Right { inner: DoorHingeStruct<'mc> },
}
impl<'mc> std::fmt::Display for DoorHinge<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DoorHinge::Left { .. } => f.write_str("LEFT"),
            DoorHinge::Right { .. } => f.write_str("RIGHT"),
        }
    }
}
impl<'mc> std::ops::Deref for DoorHinge<'mc> {
    type Target = DoorHingeStruct<'mc>;
    fn deref(&self) -> &<DoorHinge<'mc> as std::ops::Deref>::Target {
        match self {
            DoorHinge::Left { inner } => inner,
            DoorHinge::Right { inner } => inner,
        }
    }
}

impl<'mc> DoorHinge<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DoorHinge<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Door/Hinge");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Door/Hinge;",
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
            "LEFT" => Ok(DoorHinge::Left {
                inner: DoorHingeStruct::from_raw(env, obj)?,
            }),
            "RIGHT" => Ok(DoorHinge::Right {
                inner: DoorHingeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct DoorHingeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DoorHinge<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Left { inner } => inner.0.clone(),
            Self::Right { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Left { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Right { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DoorHinge<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DoorHinge from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Door/Hinge")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DoorHinge object, got {}",
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
                "LEFT" => Ok(DoorHinge::Left {
                    inner: DoorHingeStruct::from_raw(env, obj)?,
                }),
                "RIGHT" => Ok(DoorHinge::Right {
                    inner: DoorHingeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for DoorHingeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DoorHingeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate DoorHingeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Door/Hinge")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DoorHingeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DoorHingeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::DoorHinge<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Door/Hinge;");
        let cls = jni.find_class("org/bukkit/block/data/type/Door/Hinge");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::DoorHinge::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct EnderChest<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EnderChest<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EnderChest<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EnderChest from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/EnderChest")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EnderChest object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EnderChest<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for EnderChest<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnderChest into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for EnderChest<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EnderChest into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct PinkPetals<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PinkPetals<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PinkPetals<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PinkPetals from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/PinkPetals")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PinkPetals object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PinkPetals<'mc> {
    /// Gets the value of the 'flower_amount' property.
    pub fn flower_amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFlowerAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'flower_amount' property.
    pub fn set_flower_amount(&self, flower_amount: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(flower_amount);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlowerAmount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'flower_amount' property.
    pub fn maximum_flower_amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumFlowerAmount",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for PinkPetals<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PinkPetals into crate::block::data::Directional")
    }
}
#[repr(C)]
pub struct ChiseledBookshelf<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ChiseledBookshelf<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ChiseledBookshelf<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ChiseledBookshelf from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/ChiseledBookshelf")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChiseledBookshelf object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ChiseledBookshelf<'mc> {
    /// Checks if the following slot is occupied.
    pub fn is_slot_occupied(&self, slot: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Z");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isSlotOccupied",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether the following slot is occupied.
    pub fn set_slot_occupied(
        &self,
        slot: i32,
        occupied: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(IZ)V");
        let val_1 = jni::objects::JValueGen::Int(slot);
        let val_2 = jni::objects::JValueGen::Bool(occupied.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSlotOccupied",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the indexes of all the occupied slots present on this block.
    pub fn occupied_slots(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getOccupiedSlots",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the maximum amount of slots on this block.
    pub fn maximum_occupied_slots(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumOccupiedSlots",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for ChiseledBookshelf<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting ChiseledBookshelf into crate::block::data::Directional")
    }
}
#[repr(C)]
pub struct TrapDoor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TrapDoor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TrapDoor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TrapDoor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/TrapDoor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TrapDoor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TrapDoor<'mc> {
    /// Gets the value of the 'half' property.
    pub fn half(
        &self,
    ) -> Result<crate::block::data::BisectedHalf<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/Bisected/Half;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHalf", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BisectedHalf::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'half' property.
    pub fn set_half(
        &self,
        half: impl Into<crate::block::data::BisectedHalf<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/Bisected/Half;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(half.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHalf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'open' property.
    pub fn is_open(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'open' property.
    pub fn set_open(&self, open: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(open.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Bisected<'mc>> for TrapDoor<'mc> {
    fn into(self) -> crate::block::data::Bisected<'mc> {
        crate::block::data::Bisected::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TrapDoor into crate::block::data::Bisected")
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for TrapDoor<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TrapDoor into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Openable<'mc>> for TrapDoor<'mc> {
    fn into(self) -> crate::block::data::Openable<'mc> {
        crate::block::data::Openable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TrapDoor into crate::block::data::Openable")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for TrapDoor<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TrapDoor into crate::block::data::Powerable")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for TrapDoor<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TrapDoor into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct Skull<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Skull<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Skull<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Skull from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Skull")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Skull object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Skull<'mc> {
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'rotation' property.
    pub fn rotation(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRotation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'rotation' property.
    pub fn set_rotation(
        &self,
        rotation: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for Skull<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Skull into crate::block::data::Powerable")
    }
}
impl<'mc> Into<crate::block::data::Rotatable<'mc>> for Skull<'mc> {
    fn into(self) -> crate::block::data::Rotatable<'mc> {
        crate::block::data::Rotatable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Skull into crate::block::data::Rotatable")
    }
}
#[repr(C)]
pub struct PitcherCrop<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PitcherCrop<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PitcherCrop<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PitcherCrop from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/PitcherCrop")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PitcherCrop object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PitcherCrop<'mc> {
    /// Gets the value of the 'age' property.
    pub fn age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'age' property.
    pub fn set_age(&self, age: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(age);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'age' property.
    pub fn maximum_age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'half' property.
    pub fn half(
        &self,
    ) -> Result<crate::block::data::BisectedHalf<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/Bisected/Half;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHalf", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BisectedHalf::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'half' property.
    pub fn set_half(
        &self,
        half: impl Into<crate::block::data::BisectedHalf<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/Bisected/Half;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(half.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHalf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Ageable<'mc>> for PitcherCrop<'mc> {
    fn into(self) -> crate::block::data::Ageable<'mc> {
        crate::block::data::Ageable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PitcherCrop into crate::block::data::Ageable")
    }
}
impl<'mc> Into<crate::block::data::Bisected<'mc>> for PitcherCrop<'mc> {
    fn into(self) -> crate::block::data::Bisected<'mc> {
        crate::block::data::Bisected::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PitcherCrop into crate::block::data::Bisected")
    }
}
#[repr(C)]
pub struct NoteBlock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for NoteBlock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for NoteBlock<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate NoteBlock from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/NoteBlock")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a NoteBlock object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> NoteBlock<'mc> {
    /// Gets the value of the 'instrument' property.
    pub fn instrument(&self) -> Result<crate::Instrument<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Instrument;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInstrument", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Instrument::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'instrument' property.
    pub fn set_instrument(
        &self,
        instrument: impl Into<crate::Instrument<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Instrument;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(instrument.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInstrument",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'note' property.
    pub fn note(&self) -> Result<crate::Note<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Note;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getNote", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Note::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'note' property.
    pub fn set_note(
        &self,
        note: impl Into<crate::Note<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Note;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(note.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNote",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for NoteBlock<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting NoteBlock into crate::block::data::Powerable")
    }
}
#[repr(C)]
pub struct AmethystCluster<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AmethystCluster<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AmethystCluster<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AmethystCluster from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/AmethystCluster")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AmethystCluster object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AmethystCluster<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for AmethystCluster<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting AmethystCluster into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for AmethystCluster<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting AmethystCluster into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct Snow<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Snow<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Snow<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Snow from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Snow")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Snow object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Snow<'mc> {
    /// Gets the value of the 'layers' property.
    pub fn layers(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLayers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'layers' property.
    pub fn set_layers(&self, layers: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(layers);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLayers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the minimum allowed value of the 'layers' property.
    pub fn minimum_layers(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMinimumLayers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the maximum allowed value of the 'layers' property.
    pub fn maximum_layers(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumLayers",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Snow<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Snow into crate::block::data::BlockData")
    }
}
#[repr(C)]
pub struct TripwireHook<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TripwireHook<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TripwireHook<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TripwireHook from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/TripwireHook")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TripwireHook object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TripwireHook<'mc> {
    /// Gets the value of the 'attached' property.
    pub fn is_attached(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAttached", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'attached' property.
    pub fn set_attached(&self, attached: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(attached.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttached",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Attachable<'mc>> for TripwireHook<'mc> {
    fn into(self) -> crate::block::data::Attachable<'mc> {
        crate::block::data::Attachable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TripwireHook into crate::block::data::Attachable")
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for TripwireHook<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TripwireHook into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for TripwireHook<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TripwireHook into crate::block::data::Powerable")
    }
}
#[repr(C)]
pub struct TNT<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TNT<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TNT<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TNT from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/TNT")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TNT object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TNT<'mc> {
    /// Gets the value of the 'unstable' property.
    pub fn is_unstable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnstable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'unstable' property.
    pub fn set_unstable(&self, unstable: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(unstable.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnstable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for TNT<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TNT into crate::block::data::BlockData")
    }
}
#[repr(C)]
pub struct Leaves<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Leaves<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Leaves<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Leaves from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Leaves")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Leaves object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Leaves<'mc> {
    /// Gets the value of the 'persistent' property.
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPersistent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'persistent' property.
    pub fn set_persistent(&self, persistent: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(persistent.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPersistent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'distance' property.
    pub fn distance(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'distance' property.
    pub fn set_distance(&self, distance: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(distance);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDistance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Leaves<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Leaves into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct GlowLichen<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for GlowLichen<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for GlowLichen<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate GlowLichen from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/GlowLichen")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a GlowLichen object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> GlowLichen<'mc> {
    /// Checks if this block has the specified face enabled.
    pub fn has_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set whether this block has the specified face enabled.
    pub fn set_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        has: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(has.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFace",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get all of the faces which are enabled on this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets all of this faces which may be set on this block.
    pub fn allowed_faces(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAllowedFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::MultipleFacing<'mc>> for GlowLichen<'mc> {
    fn into(self) -> crate::block::data::MultipleFacing<'mc> {
        crate::block::data::MultipleFacing::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting GlowLichen into crate::block::data::MultipleFacing")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for GlowLichen<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting GlowLichen into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct SculkVein<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SculkVein<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SculkVein<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SculkVein from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/SculkVein")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SculkVein object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SculkVein<'mc> {
    /// Checks if this block has the specified face enabled.
    pub fn has_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set whether this block has the specified face enabled.
    pub fn set_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        has: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(has.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFace",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get all of the faces which are enabled on this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets all of this faces which may be set on this block.
    pub fn allowed_faces(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAllowedFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::MultipleFacing<'mc>> for SculkVein<'mc> {
    fn into(self) -> crate::block::data::MultipleFacing<'mc> {
        crate::block::data::MultipleFacing::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SculkVein into crate::block::data::MultipleFacing")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for SculkVein<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SculkVein into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct Farmland<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Farmland<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Farmland<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Farmland from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Farmland")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Farmland object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Farmland<'mc> {
    /// Gets the value of the 'moisture' property.
    pub fn moisture(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMoisture", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'moisture' property.
    pub fn set_moisture(&self, moisture: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(moisture);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMoisture",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'moisture' property.
    pub fn maximum_moisture(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumMoisture",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Farmland<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Farmland into crate::block::data::BlockData")
    }
}
#[repr(C)]
pub struct RespawnAnchor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RespawnAnchor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RespawnAnchor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RespawnAnchor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/RespawnAnchor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RespawnAnchor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> RespawnAnchor<'mc> {
    /// Gets the value of the 'charges' property.
    pub fn charges(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCharges", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'charges' property.
    pub fn set_charges(&self, charges: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(charges);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCharges",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'charges' property.
    pub fn maximum_charges(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumCharges",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for RespawnAnchor<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RespawnAnchor into crate::block::data::BlockData")
    }
}
#[repr(C)]
pub struct Bed<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Bed<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Bed<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Bed from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Bed")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Bed object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Bed<'mc> {
    /// Gets the value of the 'part' property.
    pub fn part(
        &self,
    ) -> Result<crate::block::data::mod_type::BedPart<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Bed/Part;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPart", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::BedPart::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'part' property.
    pub fn set_part(
        &self,
        part: impl Into<crate::block::data::mod_type::BedPart<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Bed/Part;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(part.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPart",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'occupied' property.
    pub fn is_occupied(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccupied", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Bed<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Bed into crate::block::data::Directional")
    }
}
pub enum BedPart<'mc> {
    Head { inner: BedPartStruct<'mc> },
    Foot { inner: BedPartStruct<'mc> },
}
impl<'mc> std::fmt::Display for BedPart<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BedPart::Head { .. } => f.write_str("HEAD"),
            BedPart::Foot { .. } => f.write_str("FOOT"),
        }
    }
}
impl<'mc> std::ops::Deref for BedPart<'mc> {
    type Target = BedPartStruct<'mc>;
    fn deref(&self) -> &<BedPart<'mc> as std::ops::Deref>::Target {
        match self {
            BedPart::Head { inner } => inner,
            BedPart::Foot { inner } => inner,
        }
    }
}

impl<'mc> BedPart<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BedPart<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Bed/Part");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Bed/Part;",
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
            "HEAD" => Ok(BedPart::Head {
                inner: BedPartStruct::from_raw(env, obj)?,
            }),
            "FOOT" => Ok(BedPart::Foot {
                inner: BedPartStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct BedPartStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BedPart<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Head { inner } => inner.0.clone(),
            Self::Foot { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Head { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Foot { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BedPart<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BedPart from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Bed/Part")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BedPart object, got {}",
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
                "HEAD" => Ok(BedPart::Head {
                    inner: BedPartStruct::from_raw(env, obj)?,
                }),
                "FOOT" => Ok(BedPart::Foot {
                    inner: BedPartStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for BedPartStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BedPartStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BedPartStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Bed/Part")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BedPartStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BedPartStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::BedPart<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Bed/Part;");
        let cls = jni.find_class("org/bukkit/block/data/type/Bed/Part");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::BedPart::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct TurtleEgg<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TurtleEgg<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TurtleEgg<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TurtleEgg from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/TurtleEgg")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TurtleEgg object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TurtleEgg<'mc> {
    /// Gets the value of the 'eggs' property.
    pub fn eggs(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEggs", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'eggs' property.
    pub fn set_eggs(&self, eggs: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(eggs);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEggs",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the minimum allowed value of the 'eggs' property.
    pub fn minimum_eggs(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinimumEggs", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the maximum allowed value of the 'eggs' property.
    pub fn maximum_eggs(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumEggs", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'hatch' property.
    pub fn hatch(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHatch", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'hatch' property.
    pub fn set_hatch(&self, hatch: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(hatch);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHatch",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'hatch' property.
    pub fn maximum_hatch(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumHatch", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Hatchable<'mc>> for TurtleEgg<'mc> {
    fn into(self) -> crate::block::data::Hatchable<'mc> {
        crate::block::data::Hatchable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TurtleEgg into crate::block::data::Hatchable")
    }
}
#[repr(C)]
pub struct Scaffolding<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Scaffolding<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Scaffolding<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Scaffolding from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Scaffolding")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Scaffolding object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Scaffolding<'mc> {
    /// Gets the value of the 'bottom' property.
    pub fn is_bottom(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBottom", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'bottom' property.
    pub fn set_bottom(&self, bottom: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(bottom.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBottom",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'distance' property.
    pub fn distance(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'distance' property.
    pub fn set_distance(&self, distance: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(distance);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDistance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'distance' property.
    pub fn maximum_distance(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumDistance",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Scaffolding<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Scaffolding into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct WallHangingSign<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for WallHangingSign<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for WallHangingSign<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate WallHangingSign from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/WallHangingSign")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WallHangingSign object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> WallHangingSign<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for WallHangingSign<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WallHangingSign into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for WallHangingSign<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WallHangingSign into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct WallSkull<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for WallSkull<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for WallSkull<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WallSkull from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/WallSkull")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WallSkull object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> WallSkull<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for WallSkull<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WallSkull into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for WallSkull<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting WallSkull into crate::block::data::Powerable")
    }
}
#[repr(C)]
pub struct Beehive<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Beehive<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Beehive<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Beehive from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Beehive")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Beehive object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Beehive<'mc> {
    /// Gets the value of the 'honey_level' property.
    pub fn honey_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHoneyLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'honey_level' property.
    pub fn set_honey_level(&self, honey_level: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(honey_level);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHoneyLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'honey_level' property.
    pub fn maximum_honey_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumHoneyLevel",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Beehive<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Beehive into crate::block::data::Directional")
    }
}
#[repr(C)]
pub struct Cocoa<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Cocoa<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Cocoa<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cocoa from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Cocoa")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cocoa object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Cocoa<'mc> {
    /// Gets the value of the 'age' property.
    pub fn age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'age' property.
    pub fn set_age(&self, age: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(age);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'age' property.
    pub fn maximum_age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Ageable<'mc>> for Cocoa<'mc> {
    fn into(self) -> crate::block::data::Ageable<'mc> {
        crate::block::data::Ageable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Cocoa into crate::block::data::Ageable")
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Cocoa<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Cocoa into crate::block::data::Directional")
    }
}
#[repr(C)]
pub struct Chest<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Chest<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Chest<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Chest from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Chest")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Chest object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Chest<'mc> {
    /// Gets the value of the 'type' property.
    pub fn get_type(
        &self,
    ) -> Result<crate::block::data::mod_type::ChestType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Chest/Type;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::ChestType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'type' property.
    pub fn set_type(
        &self,
        val_type: impl Into<crate::block::data::mod_type::ChestType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Chest/Type;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Chest<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Chest into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Chest<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Chest into crate::block::data::Waterlogged")
    }
}
pub enum ChestType<'mc> {
    Single { inner: ChestTypeStruct<'mc> },
    Left { inner: ChestTypeStruct<'mc> },
    Right { inner: ChestTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for ChestType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChestType::Single { .. } => f.write_str("SINGLE"),
            ChestType::Left { .. } => f.write_str("LEFT"),
            ChestType::Right { .. } => f.write_str("RIGHT"),
        }
    }
}
impl<'mc> std::ops::Deref for ChestType<'mc> {
    type Target = ChestTypeStruct<'mc>;
    fn deref(&self) -> &<ChestType<'mc> as std::ops::Deref>::Target {
        match self {
            ChestType::Single { inner } => inner,
            ChestType::Left { inner } => inner,
            ChestType::Right { inner } => inner,
        }
    }
}

impl<'mc> ChestType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ChestType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Chest/Type");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Chest/Type;",
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
            "SINGLE" => Ok(ChestType::Single {
                inner: ChestTypeStruct::from_raw(env, obj)?,
            }),
            "LEFT" => Ok(ChestType::Left {
                inner: ChestTypeStruct::from_raw(env, obj)?,
            }),
            "RIGHT" => Ok(ChestType::Right {
                inner: ChestTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ChestTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ChestType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Single { inner } => inner.0.clone(),
            Self::Left { inner } => inner.0.clone(),
            Self::Right { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Single { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Left { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Right { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ChestType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ChestType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Chest/Type")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChestType object, got {}",
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
                "SINGLE" => Ok(ChestType::Single {
                    inner: ChestTypeStruct::from_raw(env, obj)?,
                }),
                "LEFT" => Ok(ChestType::Left {
                    inner: ChestTypeStruct::from_raw(env, obj)?,
                }),
                "RIGHT" => Ok(ChestType::Right {
                    inner: ChestTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ChestTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ChestTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ChestTypeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Chest/Type")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChestTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ChestTypeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::ChestType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Chest/Type;");
        let cls = jni.find_class("org/bukkit/block/data/type/Chest/Type");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::ChestType::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct Bell<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Bell<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Bell<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Bell from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Bell")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Bell object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Bell<'mc> {
    /// Gets the value of the 'attachment' property.
    pub fn attachment(
        &self,
    ) -> Result<crate::block::data::mod_type::BellAttachment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Bell/Attachment;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachment", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::BellAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'attachment' property.
    pub fn set_attachment(
        &self,
        attachment: impl Into<crate::block::data::mod_type::BellAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Bell/Attachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(attachment.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Bell<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Bell into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for Bell<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Bell into crate::block::data::Powerable")
    }
}
pub enum BellAttachment<'mc> {
    Floor { inner: BellAttachmentStruct<'mc> },
    Ceiling { inner: BellAttachmentStruct<'mc> },
    SingleWall { inner: BellAttachmentStruct<'mc> },
    DoubleWall { inner: BellAttachmentStruct<'mc> },
}
impl<'mc> std::fmt::Display for BellAttachment<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BellAttachment::Floor { .. } => f.write_str("FLOOR"),
            BellAttachment::Ceiling { .. } => f.write_str("CEILING"),
            BellAttachment::SingleWall { .. } => f.write_str("SINGLE_WALL"),
            BellAttachment::DoubleWall { .. } => f.write_str("DOUBLE_WALL"),
        }
    }
}
impl<'mc> std::ops::Deref for BellAttachment<'mc> {
    type Target = BellAttachmentStruct<'mc>;
    fn deref(&self) -> &<BellAttachment<'mc> as std::ops::Deref>::Target {
        match self {
            BellAttachment::Floor { inner } => inner,
            BellAttachment::Ceiling { inner } => inner,
            BellAttachment::SingleWall { inner } => inner,
            BellAttachment::DoubleWall { inner } => inner,
        }
    }
}

impl<'mc> BellAttachment<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BellAttachment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Bell/Attachment");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Bell/Attachment;",
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
            "FLOOR" => Ok(BellAttachment::Floor {
                inner: BellAttachmentStruct::from_raw(env, obj)?,
            }),
            "CEILING" => Ok(BellAttachment::Ceiling {
                inner: BellAttachmentStruct::from_raw(env, obj)?,
            }),
            "SINGLE_WALL" => Ok(BellAttachment::SingleWall {
                inner: BellAttachmentStruct::from_raw(env, obj)?,
            }),
            "DOUBLE_WALL" => Ok(BellAttachment::DoubleWall {
                inner: BellAttachmentStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct BellAttachmentStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BellAttachment<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Floor { inner } => inner.0.clone(),
            Self::Ceiling { inner } => inner.0.clone(),
            Self::SingleWall { inner } => inner.0.clone(),
            Self::DoubleWall { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Floor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ceiling { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SingleWall { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DoubleWall { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BellAttachment<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BellAttachment from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/Bell/Attachment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BellAttachment object, got {}",
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
                "FLOOR" => Ok(BellAttachment::Floor {
                    inner: BellAttachmentStruct::from_raw(env, obj)?,
                }),
                "CEILING" => Ok(BellAttachment::Ceiling {
                    inner: BellAttachmentStruct::from_raw(env, obj)?,
                }),
                "SINGLE_WALL" => Ok(BellAttachment::SingleWall {
                    inner: BellAttachmentStruct::from_raw(env, obj)?,
                }),
                "DOUBLE_WALL" => Ok(BellAttachment::DoubleWall {
                    inner: BellAttachmentStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for BellAttachmentStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BellAttachmentStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BellAttachmentStruct from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/Bell/Attachment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BellAttachmentStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BellAttachmentStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::BellAttachment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Bell/Attachment;");
        let cls = jni.find_class("org/bukkit/block/data/type/Bell/Attachment");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::BellAttachment::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct SculkCatalyst<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SculkCatalyst<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SculkCatalyst<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SculkCatalyst from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/SculkCatalyst")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SculkCatalyst object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SculkCatalyst<'mc> {
    /// Gets the value of the 'bloom' property.
    pub fn is_bloom(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBloom", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'bloom' property.
    pub fn set_bloom(&self, bloom: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(bloom.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBloom",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for SculkCatalyst<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SculkCatalyst into crate::block::data::BlockData")
    }
}
#[repr(C)]
pub struct Cake<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Cake<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Cake<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Cake from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Cake")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Cake object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Cake<'mc> {
    /// Gets the value of the 'bites' property.
    pub fn bites(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBites", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'bites' property.
    pub fn set_bites(&self, bites: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(bites);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBites",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'bites' property.
    pub fn maximum_bites(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumBites", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Cake<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Cake into crate::block::data::BlockData")
    }
}
#[repr(C)]
pub struct Gate<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Gate<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Gate<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Gate from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Gate")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Gate object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Gate<'mc> {
    /// Gets the value of the 'in_wall' property.
    pub fn is_in_wall(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWall", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'in_wall' property.
    pub fn set_in_wall(&self, in_wall: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(in_wall.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInWall",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'open' property.
    pub fn is_open(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'open' property.
    pub fn set_open(&self, open: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(open.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Gate<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Gate into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Openable<'mc>> for Gate<'mc> {
    fn into(self) -> crate::block::data::Openable<'mc> {
        crate::block::data::Openable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Gate into crate::block::data::Openable")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for Gate<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Gate into crate::block::data::Powerable")
    }
}
#[repr(C)]
pub struct Lectern<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Lectern<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Lectern<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Lectern from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Lectern")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Lectern object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Lectern<'mc> {
    /// Gets the value of the 'has_book' property.
    pub fn has_book(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasBook", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Lectern<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Lectern into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for Lectern<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Lectern into crate::block::data::Powerable")
    }
}
#[repr(C)]
pub struct TechnicalPiston<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TechnicalPiston<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TechnicalPiston<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TechnicalPiston from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/TechnicalPiston")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TechnicalPiston object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TechnicalPiston<'mc> {
    /// Gets the value of the 'type' property.
    pub fn get_type(
        &self,
    ) -> Result<crate::block::data::mod_type::TechnicalPistonType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/TechnicalPiston/Type;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::TechnicalPistonType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'type' property.
    pub fn set_type(
        &self,
        val_type: impl Into<crate::block::data::mod_type::TechnicalPistonType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/TechnicalPiston/Type;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for TechnicalPiston<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TechnicalPiston into crate::block::data::Directional")
    }
}
pub enum TechnicalPistonType<'mc> {
    Normal {
        inner: TechnicalPistonTypeStruct<'mc>,
    },
    Sticky {
        inner: TechnicalPistonTypeStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for TechnicalPistonType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TechnicalPistonType::Normal { .. } => f.write_str("NORMAL"),
            TechnicalPistonType::Sticky { .. } => f.write_str("STICKY"),
        }
    }
}
impl<'mc> std::ops::Deref for TechnicalPistonType<'mc> {
    type Target = TechnicalPistonTypeStruct<'mc>;
    fn deref(&self) -> &<TechnicalPistonType<'mc> as std::ops::Deref>::Target {
        match self {
            TechnicalPistonType::Normal { inner } => inner,
            TechnicalPistonType::Sticky { inner } => inner,
        }
    }
}

impl<'mc> TechnicalPistonType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<TechnicalPistonType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/TechnicalPiston/Type");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/TechnicalPiston/Type;",
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
            "NORMAL" => Ok(TechnicalPistonType::Normal {
                inner: TechnicalPistonTypeStruct::from_raw(env, obj)?,
            }),
            "STICKY" => Ok(TechnicalPistonType::Sticky {
                inner: TechnicalPistonTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct TechnicalPistonTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TechnicalPistonType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Normal { inner } => inner.0.clone(),
            Self::Sticky { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Normal { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Sticky { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TechnicalPistonType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TechnicalPistonType from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/TechnicalPiston/Type")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TechnicalPistonType object, got {}",
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
                "NORMAL" => Ok(TechnicalPistonType::Normal {
                    inner: TechnicalPistonTypeStruct::from_raw(env, obj)?,
                }),
                "STICKY" => Ok(TechnicalPistonType::Sticky {
                    inner: TechnicalPistonTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for TechnicalPistonTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TechnicalPistonTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TechnicalPistonTypeStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/TechnicalPiston/Type")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TechnicalPistonTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TechnicalPistonTypeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::TechnicalPistonType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/TechnicalPiston/Type;");
        let cls = jni.find_class("org/bukkit/block/data/type/TechnicalPiston/Type");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::TechnicalPistonType::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct DecoratedPot<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for DecoratedPot<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for DecoratedPot<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DecoratedPot from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/DecoratedPot")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DecoratedPot object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> DecoratedPot<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for DecoratedPot<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DecoratedPot into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for DecoratedPot<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting DecoratedPot into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct RedstoneWallTorch<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RedstoneWallTorch<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RedstoneWallTorch<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate RedstoneWallTorch from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/RedstoneWallTorch")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RedstoneWallTorch object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> RedstoneWallTorch<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'lit' property.
    pub fn is_lit(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLit", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'lit' property.
    pub fn set_lit(&self, lit: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(lit.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLit",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for RedstoneWallTorch<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RedstoneWallTorch into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Lightable<'mc>> for RedstoneWallTorch<'mc> {
    fn into(self) -> crate::block::data::Lightable<'mc> {
        crate::block::data::Lightable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting RedstoneWallTorch into crate::block::data::Lightable")
    }
}
#[repr(C)]
pub struct CaveVinesPlant<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CaveVinesPlant<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CaveVinesPlant<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CaveVinesPlant from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/CaveVinesPlant")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CaveVinesPlant object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CaveVinesPlant<'mc> {
    /// Gets the value of the 'berries' property.
    pub fn is_berries(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBerries", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'berries' property.
    pub fn set_berries(&self, berries: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(berries.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBerries",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for CaveVinesPlant<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CaveVinesPlant into crate::block::data::BlockData")
    }
}
#[repr(C)]
pub struct Sign<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Sign<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Sign<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Sign from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Sign")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Sign object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Sign<'mc> {
    /// Gets the value of the 'rotation' property.
    pub fn rotation(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRotation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'rotation' property.
    pub fn set_rotation(
        &self,
        rotation: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRotation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Rotatable<'mc>> for Sign<'mc> {
    fn into(self) -> crate::block::data::Rotatable<'mc> {
        crate::block::data::Rotatable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Sign into crate::block::data::Rotatable")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Sign<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Sign into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct Fire<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Fire<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Fire<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Fire from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Fire")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Fire object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Fire<'mc> {
    /// Gets the value of the 'age' property.
    pub fn age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'age' property.
    pub fn set_age(&self, age: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(age);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'age' property.
    pub fn maximum_age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Checks if this block has the specified face enabled.
    pub fn has_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set whether this block has the specified face enabled.
    pub fn set_face(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        has: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(has.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFace",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get all of the faces which are enabled on this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets all of this faces which may be set on this block.
    pub fn allowed_faces(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAllowedFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Ageable<'mc>> for Fire<'mc> {
    fn into(self) -> crate::block::data::Ageable<'mc> {
        crate::block::data::Ageable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Fire into crate::block::data::Ageable")
    }
}
impl<'mc> Into<crate::block::data::MultipleFacing<'mc>> for Fire<'mc> {
    fn into(self) -> crate::block::data::MultipleFacing<'mc> {
        crate::block::data::MultipleFacing::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Fire into crate::block::data::MultipleFacing")
    }
}
#[repr(C)]
pub struct Comparator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Comparator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Comparator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Comparator from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Comparator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Comparator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Comparator<'mc> {
    /// Gets the value of the 'mode' property.
    pub fn mode(
        &self,
    ) -> Result<crate::block::data::mod_type::ComparatorMode<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Comparator/Mode;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::ComparatorMode::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'mode' property.
    pub fn set_mode(
        &self,
        mode: impl Into<crate::block::data::mod_type::ComparatorMode<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Comparator/Mode;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mode.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMode",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Comparator<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Comparator into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for Comparator<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Comparator into crate::block::data::Powerable")
    }
}
pub enum ComparatorMode<'mc> {
    Compare { inner: ComparatorModeStruct<'mc> },
    Subtract { inner: ComparatorModeStruct<'mc> },
}
impl<'mc> std::fmt::Display for ComparatorMode<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparatorMode::Compare { .. } => f.write_str("COMPARE"),
            ComparatorMode::Subtract { .. } => f.write_str("SUBTRACT"),
        }
    }
}
impl<'mc> std::ops::Deref for ComparatorMode<'mc> {
    type Target = ComparatorModeStruct<'mc>;
    fn deref(&self) -> &<ComparatorMode<'mc> as std::ops::Deref>::Target {
        match self {
            ComparatorMode::Compare { inner } => inner,
            ComparatorMode::Subtract { inner } => inner,
        }
    }
}

impl<'mc> ComparatorMode<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ComparatorMode<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Comparator/Mode");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Comparator/Mode;",
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
            "COMPARE" => Ok(ComparatorMode::Compare {
                inner: ComparatorModeStruct::from_raw(env, obj)?,
            }),
            "SUBTRACT" => Ok(ComparatorMode::Subtract {
                inner: ComparatorModeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ComparatorModeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ComparatorMode<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Compare { inner } => inner.0.clone(),
            Self::Subtract { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Compare { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Subtract { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ComparatorMode<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ComparatorMode from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/Comparator/Mode")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ComparatorMode object, got {}",
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
                "COMPARE" => Ok(ComparatorMode::Compare {
                    inner: ComparatorModeStruct::from_raw(env, obj)?,
                }),
                "SUBTRACT" => Ok(ComparatorMode::Subtract {
                    inner: ComparatorModeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ComparatorModeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ComparatorModeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ComparatorModeStruct from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/Comparator/Mode")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ComparatorModeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ComparatorModeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::ComparatorMode<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Comparator/Mode;");
        let cls = jni.find_class("org/bukkit/block/data/type/Comparator/Mode");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::ComparatorMode::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct LightningRod<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LightningRod<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LightningRod<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LightningRod from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/LightningRod")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LightningRod object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LightningRod<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for LightningRod<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LightningRod into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for LightningRod<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LightningRod into crate::block::data::Powerable")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for LightningRod<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LightningRod into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct SculkShrieker<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SculkShrieker<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SculkShrieker<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SculkShrieker from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/SculkShrieker")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SculkShrieker object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SculkShrieker<'mc> {
    /// Gets the value of the 'can_summon' property.
    pub fn is_can_summon(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCanSummon", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'can_summon' property.
    pub fn set_can_summon(&self, can_summon: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(can_summon.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCanSummon",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'shrieking' property.
    pub fn is_shrieking(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isShrieking", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'shrieking' property.
    pub fn set_shrieking(&self, shrieking: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(shrieking.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setShrieking",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for SculkShrieker<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting SculkShrieker into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct EndPortalFrame<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for EndPortalFrame<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for EndPortalFrame<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate EndPortalFrame from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/EndPortalFrame")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EndPortalFrame object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> EndPortalFrame<'mc> {
    /// Gets the value of the 'eye' property.
    pub fn has_eye(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEye", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'eye' property.
    pub fn set_eye(&self, eye: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(eye.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEye",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for EndPortalFrame<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting EndPortalFrame into crate::block::data::Directional")
    }
}
#[repr(C)]
pub struct CommandBlock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CommandBlock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CommandBlock<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CommandBlock from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/CommandBlock")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CommandBlock object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CommandBlock<'mc> {
    /// Gets the value of the 'conditional' property.
    pub fn is_conditional(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isConditional", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'conditional' property.
    pub fn set_conditional(&self, conditional: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(conditional.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConditional",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for CommandBlock<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CommandBlock into crate::block::data::Directional")
    }
}
#[repr(C)]
pub struct Campfire<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Campfire<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Campfire<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Campfire from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Campfire")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Campfire object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Campfire<'mc> {
    /// Gets the value of the 'signal_fire' property.
    pub fn is_signal_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSignalFire", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'signal_fire' property.
    pub fn set_signal_fire(&self, signal_fire: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(signal_fire.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSignalFire",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'lit' property.
    pub fn is_lit(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLit", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'lit' property.
    pub fn set_lit(&self, lit: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(lit.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLit",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Campfire<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Campfire into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Lightable<'mc>> for Campfire<'mc> {
    fn into(self) -> crate::block::data::Lightable<'mc> {
        crate::block::data::Lightable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Campfire into crate::block::data::Lightable")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Campfire<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Campfire into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct BrewingStand<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BrewingStand<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BrewingStand<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BrewingStand from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/BrewingStand")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BrewingStand object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BrewingStand<'mc> {
    /// Checks if the stand has the following bottle
    pub fn has_bottle(&self, bottle: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Z");
        let val_1 = jni::objects::JValueGen::Int(bottle);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasBottle",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Set whether the stand has this bottle present.
    pub fn set_bottle(&self, bottle: i32, has: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(IZ)V");
        let val_1 = jni::objects::JValueGen::Int(bottle);
        let val_2 = jni::objects::JValueGen::Bool(has.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBottle",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the indexes of all the bottles present on this block.
    pub fn bottles(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBottles", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the maximum amount of bottles present on this stand.
    pub fn maximum_bottles(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumBottles",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for BrewingStand<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BrewingStand into crate::block::data::BlockData")
    }
}
#[repr(C)]
pub struct Switch<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Switch<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Switch<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Switch from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Switch")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Switch object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Switch<'mc> {
    #[deprecated]
    /// Gets the value of the 'face' property.
    pub fn face(
        &self,
    ) -> Result<crate::block::data::mod_type::SwitchFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Switch/Face;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::SwitchFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Sets the value of the 'face' property.
    pub fn set_face(
        &self,
        face: impl Into<crate::block::data::mod_type::SwitchFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Switch/Face;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'face' property.
    pub fn attached_face(
        &self,
    ) -> Result<crate::block::data::FaceAttachableAttachedFace<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/FaceAttachable/AttachedFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::FaceAttachableAttachedFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'face' property.
    pub fn set_attached_face(
        &self,
        face: impl Into<crate::block::data::FaceAttachableAttachedFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/FaceAttachable/AttachedFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttachedFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Switch<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Switch into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::FaceAttachable<'mc>> for Switch<'mc> {
    fn into(self) -> crate::block::data::FaceAttachable<'mc> {
        crate::block::data::FaceAttachable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Switch into crate::block::data::FaceAttachable")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for Switch<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Switch into crate::block::data::Powerable")
    }
}
pub enum SwitchFace<'mc> {
    Floor { inner: SwitchFaceStruct<'mc> },
    Wall { inner: SwitchFaceStruct<'mc> },
    Ceiling { inner: SwitchFaceStruct<'mc> },
}
impl<'mc> std::fmt::Display for SwitchFace<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwitchFace::Floor { .. } => f.write_str("FLOOR"),
            SwitchFace::Wall { .. } => f.write_str("WALL"),
            SwitchFace::Ceiling { .. } => f.write_str("CEILING"),
        }
    }
}
impl<'mc> std::ops::Deref for SwitchFace<'mc> {
    type Target = SwitchFaceStruct<'mc>;
    fn deref(&self) -> &<SwitchFace<'mc> as std::ops::Deref>::Target {
        match self {
            SwitchFace::Floor { inner } => inner,
            SwitchFace::Wall { inner } => inner,
            SwitchFace::Ceiling { inner } => inner,
        }
    }
}

impl<'mc> SwitchFace<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SwitchFace<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Switch/Face");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Switch/Face;",
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
            "FLOOR" => Ok(SwitchFace::Floor {
                inner: SwitchFaceStruct::from_raw(env, obj)?,
            }),
            "WALL" => Ok(SwitchFace::Wall {
                inner: SwitchFaceStruct::from_raw(env, obj)?,
            }),
            "CEILING" => Ok(SwitchFace::Ceiling {
                inner: SwitchFaceStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct SwitchFaceStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SwitchFace<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Floor { inner } => inner.0.clone(),
            Self::Wall { inner } => inner.0.clone(),
            Self::Ceiling { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Floor { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Wall { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ceiling { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SwitchFace<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SwitchFace from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Switch/Face")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SwitchFace object, got {}",
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
                "FLOOR" => Ok(SwitchFace::Floor {
                    inner: SwitchFaceStruct::from_raw(env, obj)?,
                }),
                "WALL" => Ok(SwitchFace::Wall {
                    inner: SwitchFaceStruct::from_raw(env, obj)?,
                }),
                "CEILING" => Ok(SwitchFace::Ceiling {
                    inner: SwitchFaceStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for SwitchFaceStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SwitchFaceStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SwitchFaceStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Switch/Face")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SwitchFaceStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SwitchFaceStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::SwitchFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Switch/Face;");
        let cls = jni.find_class("org/bukkit/block/data/type/Switch/Face");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::SwitchFace::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct Dispenser<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Dispenser<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Dispenser<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Dispenser from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Dispenser")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Dispenser object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Dispenser<'mc> {
    /// Gets the value of the 'triggered' property.
    pub fn is_triggered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isTriggered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'triggered' property.
    pub fn set_triggered(&self, triggered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(triggered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTriggered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Dispenser<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Dispenser into crate::block::data::Directional")
    }
}
#[repr(C)]
pub struct Hopper<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Hopper<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Hopper<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Hopper from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Hopper")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Hopper object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Hopper<'mc> {
    /// Gets the value of the 'enabled' property.
    pub fn is_enabled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'enabled' property.
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
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Hopper<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Hopper into crate::block::data::Directional")
    }
}
#[repr(C)]
pub struct CaveVines<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CaveVines<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CaveVines<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CaveVines from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/CaveVines")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CaveVines object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CaveVines<'mc> {
    /// Gets the value of the 'age' property.
    pub fn age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'age' property.
    pub fn set_age(&self, age: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(age);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'age' property.
    pub fn maximum_age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'berries' property.
    pub fn is_berries(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBerries", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'berries' property.
    pub fn set_berries(&self, berries: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(berries.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBerries",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Ageable<'mc>> for CaveVines<'mc> {
    fn into(self) -> crate::block::data::Ageable<'mc> {
        crate::block::data::Ageable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CaveVines into crate::block::data::Ageable")
    }
}
impl<'mc> Into<crate::block::data::mod_type::CaveVinesPlant<'mc>> for CaveVines<'mc> {
    fn into(self) -> crate::block::data::mod_type::CaveVinesPlant<'mc> {
        crate::block::data::mod_type::CaveVinesPlant::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CaveVines into crate::block::data::mod_type::CaveVinesPlant")
    }
}
#[repr(C)]
pub struct Grindstone<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Grindstone<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Grindstone<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Grindstone from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Grindstone")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Grindstone object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Grindstone<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'face' property.
    pub fn attached_face(
        &self,
    ) -> Result<crate::block::data::FaceAttachableAttachedFace<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/FaceAttachable/AttachedFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::FaceAttachableAttachedFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'face' property.
    pub fn set_attached_face(
        &self,
        face: impl Into<crate::block::data::FaceAttachableAttachedFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/FaceAttachable/AttachedFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttachedFace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Grindstone<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Grindstone into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::FaceAttachable<'mc>> for Grindstone<'mc> {
    fn into(self) -> crate::block::data::FaceAttachable<'mc> {
        crate::block::data::FaceAttachable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Grindstone into crate::block::data::FaceAttachable")
    }
}
#[repr(C)]
pub struct Jigsaw<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Jigsaw<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Jigsaw<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Jigsaw from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Jigsaw")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Jigsaw object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Jigsaw<'mc> {
    /// Gets the value of the 'orientation' property.
    pub fn orientation(
        &self,
    ) -> Result<crate::block::data::mod_type::JigsawOrientation<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/Jigsaw/Orientation;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOrientation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::JigsawOrientation::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'orientation' property.
    pub fn set_orientation(
        &self,
        orientation: impl Into<crate::block::data::mod_type::JigsawOrientation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Jigsaw/Orientation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(orientation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOrientation",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Jigsaw<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Jigsaw into crate::block::data::BlockData")
    }
}
pub enum JigsawOrientation<'mc> {
    DownEast { inner: JigsawOrientationStruct<'mc> },
    DownNorth { inner: JigsawOrientationStruct<'mc> },
    DownSouth { inner: JigsawOrientationStruct<'mc> },
    DownWest { inner: JigsawOrientationStruct<'mc> },
    UpEast { inner: JigsawOrientationStruct<'mc> },
    UpNorth { inner: JigsawOrientationStruct<'mc> },
    UpSouth { inner: JigsawOrientationStruct<'mc> },
    UpWest { inner: JigsawOrientationStruct<'mc> },
    WestUp { inner: JigsawOrientationStruct<'mc> },
    EastUp { inner: JigsawOrientationStruct<'mc> },
    NorthUp { inner: JigsawOrientationStruct<'mc> },
    SouthUp { inner: JigsawOrientationStruct<'mc> },
}
impl<'mc> std::fmt::Display for JigsawOrientation<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JigsawOrientation::DownEast { .. } => f.write_str("DOWN_EAST"),
            JigsawOrientation::DownNorth { .. } => f.write_str("DOWN_NORTH"),
            JigsawOrientation::DownSouth { .. } => f.write_str("DOWN_SOUTH"),
            JigsawOrientation::DownWest { .. } => f.write_str("DOWN_WEST"),
            JigsawOrientation::UpEast { .. } => f.write_str("UP_EAST"),
            JigsawOrientation::UpNorth { .. } => f.write_str("UP_NORTH"),
            JigsawOrientation::UpSouth { .. } => f.write_str("UP_SOUTH"),
            JigsawOrientation::UpWest { .. } => f.write_str("UP_WEST"),
            JigsawOrientation::WestUp { .. } => f.write_str("WEST_UP"),
            JigsawOrientation::EastUp { .. } => f.write_str("EAST_UP"),
            JigsawOrientation::NorthUp { .. } => f.write_str("NORTH_UP"),
            JigsawOrientation::SouthUp { .. } => f.write_str("SOUTH_UP"),
        }
    }
}
impl<'mc> std::ops::Deref for JigsawOrientation<'mc> {
    type Target = JigsawOrientationStruct<'mc>;
    fn deref(&self) -> &<JigsawOrientation<'mc> as std::ops::Deref>::Target {
        match self {
            JigsawOrientation::DownEast { inner } => inner,
            JigsawOrientation::DownNorth { inner } => inner,
            JigsawOrientation::DownSouth { inner } => inner,
            JigsawOrientation::DownWest { inner } => inner,
            JigsawOrientation::UpEast { inner } => inner,
            JigsawOrientation::UpNorth { inner } => inner,
            JigsawOrientation::UpSouth { inner } => inner,
            JigsawOrientation::UpWest { inner } => inner,
            JigsawOrientation::WestUp { inner } => inner,
            JigsawOrientation::EastUp { inner } => inner,
            JigsawOrientation::NorthUp { inner } => inner,
            JigsawOrientation::SouthUp { inner } => inner,
        }
    }
}

impl<'mc> JigsawOrientation<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<JigsawOrientation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Jigsaw/Orientation");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Jigsaw/Orientation;",
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
            "DOWN_EAST" => Ok(JigsawOrientation::DownEast {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),
            "DOWN_NORTH" => Ok(JigsawOrientation::DownNorth {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),
            "DOWN_SOUTH" => Ok(JigsawOrientation::DownSouth {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),
            "DOWN_WEST" => Ok(JigsawOrientation::DownWest {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),
            "UP_EAST" => Ok(JigsawOrientation::UpEast {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),
            "UP_NORTH" => Ok(JigsawOrientation::UpNorth {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),
            "UP_SOUTH" => Ok(JigsawOrientation::UpSouth {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),
            "UP_WEST" => Ok(JigsawOrientation::UpWest {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),
            "WEST_UP" => Ok(JigsawOrientation::WestUp {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),
            "EAST_UP" => Ok(JigsawOrientation::EastUp {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),
            "NORTH_UP" => Ok(JigsawOrientation::NorthUp {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),
            "SOUTH_UP" => Ok(JigsawOrientation::SouthUp {
                inner: JigsawOrientationStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct JigsawOrientationStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JigsawOrientation<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::DownEast { inner } => inner.0.clone(),
            Self::DownNorth { inner } => inner.0.clone(),
            Self::DownSouth { inner } => inner.0.clone(),
            Self::DownWest { inner } => inner.0.clone(),
            Self::UpEast { inner } => inner.0.clone(),
            Self::UpNorth { inner } => inner.0.clone(),
            Self::UpSouth { inner } => inner.0.clone(),
            Self::UpWest { inner } => inner.0.clone(),
            Self::WestUp { inner } => inner.0.clone(),
            Self::EastUp { inner } => inner.0.clone(),
            Self::NorthUp { inner } => inner.0.clone(),
            Self::SouthUp { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::DownEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::DownNorth { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DownSouth { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DownWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::UpEast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::UpNorth { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::UpSouth { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::UpWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WestUp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::EastUp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::NorthUp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SouthUp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JigsawOrientation<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JigsawOrientation from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/Jigsaw/Orientation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JigsawOrientation object, got {}",
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
                "DOWN_EAST" => Ok(JigsawOrientation::DownEast {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                "DOWN_NORTH" => Ok(JigsawOrientation::DownNorth {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                "DOWN_SOUTH" => Ok(JigsawOrientation::DownSouth {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                "DOWN_WEST" => Ok(JigsawOrientation::DownWest {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                "UP_EAST" => Ok(JigsawOrientation::UpEast {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                "UP_NORTH" => Ok(JigsawOrientation::UpNorth {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                "UP_SOUTH" => Ok(JigsawOrientation::UpSouth {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                "UP_WEST" => Ok(JigsawOrientation::UpWest {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                "WEST_UP" => Ok(JigsawOrientation::WestUp {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                "EAST_UP" => Ok(JigsawOrientation::EastUp {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                "NORTH_UP" => Ok(JigsawOrientation::NorthUp {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                "SOUTH_UP" => Ok(JigsawOrientation::SouthUp {
                    inner: JigsawOrientationStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for JigsawOrientationStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JigsawOrientationStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JigsawOrientationStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/Jigsaw/Orientation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JigsawOrientationStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JigsawOrientationStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::JigsawOrientation<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/Jigsaw/Orientation;");
        let cls = jni.find_class("org/bukkit/block/data/type/Jigsaw/Orientation");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::JigsawOrientation::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct Bamboo<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Bamboo<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Bamboo<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Bamboo from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Bamboo")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Bamboo object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Bamboo<'mc> {
    /// Gets the value of the 'leaves' property.
    pub fn leaves(
        &self,
    ) -> Result<crate::block::data::mod_type::BambooLeaves<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Bamboo/Leaves;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLeaves", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::BambooLeaves::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'leaves' property.
    pub fn set_leaves(
        &self,
        leaves: impl Into<crate::block::data::mod_type::BambooLeaves<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Bamboo/Leaves;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(leaves.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLeaves",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'age' property.
    pub fn age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'age' property.
    pub fn set_age(&self, age: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(age);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'age' property.
    pub fn maximum_age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the value of the 'stage' property.
    pub fn stage(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getStage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'stage' property.
    pub fn set_stage(&self, stage: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(stage);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum allowed value of the 'stage' property.
    pub fn maximum_stage(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumStage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Ageable<'mc>> for Bamboo<'mc> {
    fn into(self) -> crate::block::data::Ageable<'mc> {
        crate::block::data::Ageable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Bamboo into crate::block::data::Ageable")
    }
}
impl<'mc> Into<crate::block::data::mod_type::Sapling<'mc>> for Bamboo<'mc> {
    fn into(self) -> crate::block::data::mod_type::Sapling<'mc> {
        crate::block::data::mod_type::Sapling::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Bamboo into crate::block::data::mod_type::Sapling")
    }
}
pub enum BambooLeaves<'mc> {
    None { inner: BambooLeavesStruct<'mc> },
    Small { inner: BambooLeavesStruct<'mc> },
    Large { inner: BambooLeavesStruct<'mc> },
}
impl<'mc> std::fmt::Display for BambooLeaves<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BambooLeaves::None { .. } => f.write_str("NONE"),
            BambooLeaves::Small { .. } => f.write_str("SMALL"),
            BambooLeaves::Large { .. } => f.write_str("LARGE"),
        }
    }
}
impl<'mc> std::ops::Deref for BambooLeaves<'mc> {
    type Target = BambooLeavesStruct<'mc>;
    fn deref(&self) -> &<BambooLeaves<'mc> as std::ops::Deref>::Target {
        match self {
            BambooLeaves::None { inner } => inner,
            BambooLeaves::Small { inner } => inner,
            BambooLeaves::Large { inner } => inner,
        }
    }
}

impl<'mc> BambooLeaves<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BambooLeaves<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Bamboo/Leaves");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Bamboo/Leaves;",
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
            "NONE" => Ok(BambooLeaves::None {
                inner: BambooLeavesStruct::from_raw(env, obj)?,
            }),
            "SMALL" => Ok(BambooLeaves::Small {
                inner: BambooLeavesStruct::from_raw(env, obj)?,
            }),
            "LARGE" => Ok(BambooLeaves::Large {
                inner: BambooLeavesStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct BambooLeavesStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BambooLeaves<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::None { inner } => inner.0.clone(),
            Self::Small { inner } => inner.0.clone(),
            Self::Large { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::None { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Small { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Large { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BambooLeaves<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BambooLeaves from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Bamboo/Leaves")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BambooLeaves object, got {}",
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
                "NONE" => Ok(BambooLeaves::None {
                    inner: BambooLeavesStruct::from_raw(env, obj)?,
                }),
                "SMALL" => Ok(BambooLeaves::Small {
                    inner: BambooLeavesStruct::from_raw(env, obj)?,
                }),
                "LARGE" => Ok(BambooLeaves::Large {
                    inner: BambooLeavesStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for BambooLeavesStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BambooLeavesStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BambooLeavesStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Bamboo/Leaves")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BambooLeavesStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BambooLeavesStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::BambooLeaves<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Bamboo/Leaves;");
        let cls = jni.find_class("org/bukkit/block/data/type/Bamboo/Leaves");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::BambooLeaves::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct StructureBlock<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StructureBlock<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StructureBlock<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StructureBlock from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/StructureBlock")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StructureBlock object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> StructureBlock<'mc> {
    /// Gets the value of the 'mode' property.
    pub fn mode(
        &self,
    ) -> Result<crate::block::data::mod_type::StructureBlockMode<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/StructureBlock/Mode;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::StructureBlockMode::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'mode' property.
    pub fn set_mode(
        &self,
        mode: impl Into<crate::block::data::mod_type::StructureBlockMode<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/StructureBlock/Mode;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mode.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMode",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for StructureBlock<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting StructureBlock into crate::block::data::BlockData")
    }
}
pub enum StructureBlockMode<'mc> {
    Save {
        inner: StructureBlockModeStruct<'mc>,
    },
    Load {
        inner: StructureBlockModeStruct<'mc>,
    },
    Corner {
        inner: StructureBlockModeStruct<'mc>,
    },
    Data {
        inner: StructureBlockModeStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for StructureBlockMode<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructureBlockMode::Save { .. } => f.write_str("SAVE"),
            StructureBlockMode::Load { .. } => f.write_str("LOAD"),
            StructureBlockMode::Corner { .. } => f.write_str("CORNER"),
            StructureBlockMode::Data { .. } => f.write_str("DATA"),
        }
    }
}
impl<'mc> std::ops::Deref for StructureBlockMode<'mc> {
    type Target = StructureBlockModeStruct<'mc>;
    fn deref(&self) -> &<StructureBlockMode<'mc> as std::ops::Deref>::Target {
        match self {
            StructureBlockMode::Save { inner } => inner,
            StructureBlockMode::Load { inner } => inner,
            StructureBlockMode::Corner { inner } => inner,
            StructureBlockMode::Data { inner } => inner,
        }
    }
}

impl<'mc> StructureBlockMode<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<StructureBlockMode<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/StructureBlock/Mode");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/StructureBlock/Mode;",
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
            "SAVE" => Ok(StructureBlockMode::Save {
                inner: StructureBlockModeStruct::from_raw(env, obj)?,
            }),
            "LOAD" => Ok(StructureBlockMode::Load {
                inner: StructureBlockModeStruct::from_raw(env, obj)?,
            }),
            "CORNER" => Ok(StructureBlockMode::Corner {
                inner: StructureBlockModeStruct::from_raw(env, obj)?,
            }),
            "DATA" => Ok(StructureBlockMode::Data {
                inner: StructureBlockModeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct StructureBlockModeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for StructureBlockMode<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Save { inner } => inner.0.clone(),
            Self::Load { inner } => inner.0.clone(),
            Self::Corner { inner } => inner.0.clone(),
            Self::Data { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Save { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Load { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Corner { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Data { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StructureBlockMode<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate StructureBlockMode from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/StructureBlock/Mode")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StructureBlockMode object, got {}",
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
                "SAVE" => Ok(StructureBlockMode::Save {
                    inner: StructureBlockModeStruct::from_raw(env, obj)?,
                }),
                "LOAD" => Ok(StructureBlockMode::Load {
                    inner: StructureBlockModeStruct::from_raw(env, obj)?,
                }),
                "CORNER" => Ok(StructureBlockMode::Corner {
                    inner: StructureBlockModeStruct::from_raw(env, obj)?,
                }),
                "DATA" => Ok(StructureBlockMode::Data {
                    inner: StructureBlockModeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for StructureBlockModeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for StructureBlockModeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate StructureBlockModeStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/StructureBlock/Mode")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a StructureBlockModeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> StructureBlockModeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::StructureBlockMode<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/StructureBlock/Mode;");
        let cls = jni.find_class("org/bukkit/block/data/type/StructureBlock/Mode");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::StructureBlockMode::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct Ladder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Ladder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Ladder<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Ladder from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Ladder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Ladder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Ladder<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Ladder<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Ladder into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Ladder<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Ladder into crate::block::data::Waterlogged")
    }
}
#[repr(C)]
pub struct Slab<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Slab<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Slab<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Slab from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Slab")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Slab object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Slab<'mc> {
    /// Gets the value of the 'type' property.
    pub fn get_type(
        &self,
    ) -> Result<crate::block::data::mod_type::SlabType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Slab/Type;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::SlabType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'type' property.
    pub fn set_type(
        &self,
        val_type: impl Into<crate::block::data::mod_type::SlabType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Slab/Type;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'waterlogged' property.
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, waterlogged: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(waterlogged.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Slab<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Slab into crate::block::data::Waterlogged")
    }
}
pub enum SlabType<'mc> {
    Top { inner: SlabTypeStruct<'mc> },
    Bottom { inner: SlabTypeStruct<'mc> },
    Double { inner: SlabTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for SlabType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SlabType::Top { .. } => f.write_str("TOP"),
            SlabType::Bottom { .. } => f.write_str("BOTTOM"),
            SlabType::Double { .. } => f.write_str("DOUBLE"),
        }
    }
}
impl<'mc> std::ops::Deref for SlabType<'mc> {
    type Target = SlabTypeStruct<'mc>;
    fn deref(&self) -> &<SlabType<'mc> as std::ops::Deref>::Target {
        match self {
            SlabType::Top { inner } => inner,
            SlabType::Bottom { inner } => inner,
            SlabType::Double { inner } => inner,
        }
    }
}

impl<'mc> SlabType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SlabType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Slab/Type");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Slab/Type;",
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
            "TOP" => Ok(SlabType::Top {
                inner: SlabTypeStruct::from_raw(env, obj)?,
            }),
            "BOTTOM" => Ok(SlabType::Bottom {
                inner: SlabTypeStruct::from_raw(env, obj)?,
            }),
            "DOUBLE" => Ok(SlabType::Double {
                inner: SlabTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct SlabTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SlabType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Top { inner } => inner.0.clone(),
            Self::Bottom { inner } => inner.0.clone(),
            Self::Double { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Top { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Bottom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Double { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SlabType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SlabType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Slab/Type")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SlabType object, got {}",
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
                "TOP" => Ok(SlabType::Top {
                    inner: SlabTypeStruct::from_raw(env, obj)?,
                }),
                "BOTTOM" => Ok(SlabType::Bottom {
                    inner: SlabTypeStruct::from_raw(env, obj)?,
                }),
                "DOUBLE" => Ok(SlabType::Double {
                    inner: SlabTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for SlabTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SlabTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SlabTypeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Slab/Type")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SlabTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SlabTypeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::SlabType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Slab/Type;");
        let cls = jni.find_class("org/bukkit/block/data/type/Slab/Type");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::SlabType::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct Barrel<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Barrel<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Barrel<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Barrel from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Barrel")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Barrel object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Barrel<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'open' property.
    pub fn is_open(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'open' property.
    pub fn set_open(&self, open: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(open.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Barrel<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Barrel into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Openable<'mc>> for Barrel<'mc> {
    fn into(self) -> crate::block::data::Openable<'mc> {
        crate::block::data::Openable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Barrel into crate::block::data::Openable")
    }
}
#[repr(C)]
pub struct TrialSpawner<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TrialSpawner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TrialSpawner<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TrialSpawner from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/TrialSpawner")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TrialSpawner object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TrialSpawner<'mc> {
    /// Gets the value of the 'trial_spawner_state' property.
    pub fn trial_spawner_state(
        &self,
    ) -> Result<crate::block::data::mod_type::TrialSpawnerState<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/TrialSpawner/State;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTrialSpawnerState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::TrialSpawnerState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'trial_spawner_state' property.
    pub fn set_trial_spawner_state(
        &self,
        state: impl Into<crate::block::data::mod_type::TrialSpawnerState<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/TrialSpawner/State;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(state.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTrialSpawnerState",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'ominous' property.
    pub fn is_ominous(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOminous", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'ominous' property.
    pub fn set_ominous(&self, ominous: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(ominous.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOminous",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for TrialSpawner<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TrialSpawner into crate::block::data::BlockData")
    }
}
pub enum TrialSpawnerState<'mc> {
    Inactive { inner: TrialSpawnerStateStruct<'mc> },
    WaitingForPlayers { inner: TrialSpawnerStateStruct<'mc> },
    Active { inner: TrialSpawnerStateStruct<'mc> },
    WaitingForRewardEjection { inner: TrialSpawnerStateStruct<'mc> },
    EjectingReward { inner: TrialSpawnerStateStruct<'mc> },
    Cooldown { inner: TrialSpawnerStateStruct<'mc> },
}
impl<'mc> std::fmt::Display for TrialSpawnerState<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrialSpawnerState::Inactive { .. } => f.write_str("INACTIVE"),
            TrialSpawnerState::WaitingForPlayers { .. } => f.write_str("WAITING_FOR_PLAYERS"),
            TrialSpawnerState::Active { .. } => f.write_str("ACTIVE"),
            TrialSpawnerState::WaitingForRewardEjection { .. } => {
                f.write_str("WAITING_FOR_REWARD_EJECTION")
            }
            TrialSpawnerState::EjectingReward { .. } => f.write_str("EJECTING_REWARD"),
            TrialSpawnerState::Cooldown { .. } => f.write_str("COOLDOWN"),
        }
    }
}
impl<'mc> std::ops::Deref for TrialSpawnerState<'mc> {
    type Target = TrialSpawnerStateStruct<'mc>;
    fn deref(&self) -> &<TrialSpawnerState<'mc> as std::ops::Deref>::Target {
        match self {
            TrialSpawnerState::Inactive { inner } => inner,
            TrialSpawnerState::WaitingForPlayers { inner } => inner,
            TrialSpawnerState::Active { inner } => inner,
            TrialSpawnerState::WaitingForRewardEjection { inner } => inner,
            TrialSpawnerState::EjectingReward { inner } => inner,
            TrialSpawnerState::Cooldown { inner } => inner,
        }
    }
}

impl<'mc> TrialSpawnerState<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<TrialSpawnerState<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/TrialSpawner/State");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/TrialSpawner/State;",
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
            "INACTIVE" => Ok(TrialSpawnerState::Inactive {
                inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
            }),
            "WAITING_FOR_PLAYERS" => Ok(TrialSpawnerState::WaitingForPlayers {
                inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
            }),
            "ACTIVE" => Ok(TrialSpawnerState::Active {
                inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
            }),
            "WAITING_FOR_REWARD_EJECTION" => Ok(TrialSpawnerState::WaitingForRewardEjection {
                inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
            }),
            "EJECTING_REWARD" => Ok(TrialSpawnerState::EjectingReward {
                inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
            }),
            "COOLDOWN" => Ok(TrialSpawnerState::Cooldown {
                inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct TrialSpawnerStateStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TrialSpawnerState<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Inactive { inner } => inner.0.clone(),
            Self::WaitingForPlayers { inner } => inner.0.clone(),
            Self::Active { inner } => inner.0.clone(),
            Self::WaitingForRewardEjection { inner } => inner.0.clone(),
            Self::EjectingReward { inner } => inner.0.clone(),
            Self::Cooldown { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Inactive { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WaitingForPlayers { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Active { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WaitingForRewardEjection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EjectingReward { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Cooldown { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TrialSpawnerState<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TrialSpawnerState from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/TrialSpawner/State")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TrialSpawnerState object, got {}",
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
                "INACTIVE" => Ok(TrialSpawnerState::Inactive {
                    inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
                }),
                "WAITING_FOR_PLAYERS" => Ok(TrialSpawnerState::WaitingForPlayers {
                    inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
                }),
                "ACTIVE" => Ok(TrialSpawnerState::Active {
                    inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
                }),
                "WAITING_FOR_REWARD_EJECTION" => Ok(TrialSpawnerState::WaitingForRewardEjection {
                    inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
                }),
                "EJECTING_REWARD" => Ok(TrialSpawnerState::EjectingReward {
                    inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
                }),
                "COOLDOWN" => Ok(TrialSpawnerState::Cooldown {
                    inner: TrialSpawnerStateStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for TrialSpawnerStateStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TrialSpawnerStateStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TrialSpawnerStateStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/type/TrialSpawner/State")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TrialSpawnerStateStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TrialSpawnerStateStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::data::mod_type::TrialSpawnerState<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/TrialSpawner/State;");
        let cls = jni.find_class("org/bukkit/block/data/type/TrialSpawner/State");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::data::mod_type::TrialSpawnerState::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct CopperBulb<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for CopperBulb<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for CopperBulb<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate CopperBulb from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/CopperBulb")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CopperBulb object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> CopperBulb<'mc> {
    /// Gets the value of the 'lit' property.
    pub fn is_lit(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLit", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'lit' property.
    pub fn set_lit(&self, lit: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(lit.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLit",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the value of the 'powered' property.
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, powered: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(powered.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Lightable<'mc>> for CopperBulb<'mc> {
    fn into(self) -> crate::block::data::Lightable<'mc> {
        crate::block::data::Lightable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CopperBulb into crate::block::data::Lightable")
    }
}
impl<'mc> Into<crate::block::data::Powerable<'mc>> for CopperBulb<'mc> {
    fn into(self) -> crate::block::data::Powerable<'mc> {
        crate::block::data::Powerable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting CopperBulb into crate::block::data::Powerable")
    }
}
#[repr(C)]
pub struct BubbleColumn<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BubbleColumn<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BubbleColumn<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BubbleColumn from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/BubbleColumn")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BubbleColumn object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BubbleColumn<'mc> {
    /// Gets the value of the 'drag' property.
    pub fn is_drag(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDrag", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'drag' property.
    pub fn set_drag(&self, drag: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(drag.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDrag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Material represented by this block data.
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a string, which when passed into a method such as
    /// {@link Server#createBlockData(java.lang.String)} will recreate this or a
    /// similar instance where unspecified states (if any) may be optionally
    /// omitted. If this instance was parsed and states are omitted, this exact
    /// instance will be creatable when parsed again, else their equality cannot
    /// be guaranteed.
    ///
    /// This method will only take effect for BlockData instances created by
    /// methods such as {@link Server#createBlockData(String)} or any similar
    /// method whereby states are optionally defined. If otherwise, the result of
    /// {@link #getAsString()} will be returned. The following behaviour would be
    /// expected:
    /// <pre>{@code
    /// String dataString = "minecraft:chest[waterlogged=true]"
    /// BlockData data = Bukkit.createBlockData(dataString);
    /// dataString.equals(data.getAsString(true)); // This would return true
    /// dataString.equals(data.getAsString(false)); // This would return false as all states are present
    /// dataString.equals(data.getAsString()); // This is equivalent to the above, "getAsString(false)"
    /// }</pre>
    pub fn get_as_string(
        &self,
        hide_unspecified: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Bool(hide_unspecified.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAsString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Merges all explicitly set states from the given data with this BlockData.
    ///
    /// Note that the given data MUST have been created from one of the String
    /// parse methods, e.g. {@link Server#createBlockData(java.lang.String)} and
    /// not have been subsequently modified.
    ///
    /// Note also that the block types must match identically.
    pub fn merge(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "merge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the specified BlockData matches this block data.
    ///
    /// The semantics of this method are such that for manually created or
    /// modified BlockData it has the same effect as
    /// {@link Object#equals(java.lang.Object)}, whilst for parsed data (that to
    /// which {@link #merge(org.bukkit.block.data.BlockData)} applies, it will
    /// return true when the type and all explicitly set states match.
    ///
    /// <b>Note that these semantics mean that a.matches(b) may not be the same
    /// as b.matches(a)</b>
    pub fn matches(
        &self,
        data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns a copy of this BlockData.
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/BlockData;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the block's {@link SoundGroup} which can be used to get its step
    /// sound, hit sound, and others.
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/SoundGroup;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSoundGroup", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::SoundGroup::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the amount of light emitted by this state when in the world.
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLightEmission",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Check whether or not this state will occlude other blocks.
    ///
    /// Block state occlusion affects visual features of other blocks (e.g. leaves and
    /// wet sponges will not spawn dripping water particles if an occluding state is
    /// below it), or whether light will pass through it.
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Check whether or not this state requires a specific item to be used to drop
    /// items when broken. For example, diamond ore requires an iron pickaxe and will
    /// not drop diamonds when broken with a wooden or stone pickaxe.
    pub fn requires_correct_tool_for_drops(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "requiresCorrectToolForDrops",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns if the given item is a preferred choice to break this Block.
    /// In some cases this determines if a block will drop anything or extra
    /// loot.
    pub fn is_preferred_tool(
        &self,
        tool: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(tool.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPreferredTool",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Returns the reaction of the block when moved by a piston
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/PistonMoveReaction;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPistonMoveReaction",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::PistonMoveReaction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this state would be properly supported if it were placed at
    /// the block at the given {@link Location}.
    ///
    /// This may be useful, for instance, to check whether or not a wall torch is
    /// capable of surviving on its neighbouring block states.
    pub fn is_supported(
        &self,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Checks if a state's {@link BlockFace} is capable of providing a given level
    /// of {@link BlockSupport} for neighbouring block states.
    ///
    /// Any given state may support either none, one, or more than one level of block
    /// support depending on its states. A common example would be a wall's ability to support
    /// torches only on the center of the upper block face, whereas a grass block would
    /// support all levels of block support on all block faces.
    pub fn is_face_sturdy(
        &self,
        face: impl Into<crate::block::BlockFace<'mc>>,
        support: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(face.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(support.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isFaceSturdy",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the color this block should appear as when rendered on a map.
    pub fn map_color(&self) -> Result<crate::Color<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Color;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Color::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the material that a player would use to place this block.
    ///
    /// For most blocks this is the same as {@link #getMaterial()} but some blocks
    /// have different materials used to place them.
    /// For example:
    /// <pre>
    /// {@link Material#REDSTONE_WIRE} -> {@link Material#REDSTONE}
    /// {@link Material#CARROTS} -> {@link Material#CARROT}
    /// </pre>
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Material;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlacementMaterial",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Rotates this blockdata by the specified {@link StructureRotation}.
    ///
    /// This has no effect on blocks that do not have any rotatable states.
    pub fn rotate(
        &self,
        rotation: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rotation.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "rotate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Mirrors this blockdata using the specified {@link Mirror}.
    ///
    /// This has no effect on blocks that do not have any mirrorable states.
    pub fn mirror(
        &self,
        mirror: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(mirror.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "mirror",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Copies all applicable properties from this BlockData to the provided
    /// BlockData.
    ///
    /// Only modifies properties that both blocks share in common.
    pub fn copy_to(
        &self,
        other: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(other.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyTo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a new default {@link BlockState} for this type of Block, not
    /// bound to a location.
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockState;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createBlockState",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::BlockData<'mc>> for BubbleColumn<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting BubbleColumn into crate::block::data::BlockData")
    }
}
#[repr(C)]
pub struct Furnace<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Furnace<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Furnace<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Furnace from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Furnace")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Furnace object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Furnace<'mc> {
    /// Gets the value of the 'facing' property.
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/BlockFace;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFacing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the value of the 'facing' property.
    pub fn set_facing(
        &self,
        facing: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(facing.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFacing",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the faces which are applicable to this block.
    pub fn faces(&self) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFaces", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of the 'lit' property.
    pub fn is_lit(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLit", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'lit' property.
    pub fn set_lit(&self, lit: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(lit.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLit",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::block::data::Directional<'mc>> for Furnace<'mc> {
    fn into(self) -> crate::block::data::Directional<'mc> {
        crate::block::data::Directional::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Furnace into crate::block::data::Directional")
    }
}
impl<'mc> Into<crate::block::data::Lightable<'mc>> for Furnace<'mc> {
    fn into(self) -> crate::block::data::Lightable<'mc> {
        crate::block::data::Lightable::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Furnace into crate::block::data::Lightable")
    }
}
