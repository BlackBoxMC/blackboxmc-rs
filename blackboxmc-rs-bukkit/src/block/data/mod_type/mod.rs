#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// 'short' denotes this piston head is shorter than the usual amount because it is currently retracting.
///
/// This is a representation of an abstract class.
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
    pub fn is_short(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isShort", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_short(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setShort",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_type(
        &self,
        arg0: impl Into<crate::block::data::mod_type::TechnicalPistonType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PistonHead::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::mod_type::TechnicalPiston = temp_clone.into();
        real.set_type(arg0)
    }
    pub fn get_type(
        &self,
    ) -> Result<crate::block::data::mod_type::TechnicalPistonType<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = PistonHead::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::mod_type::TechnicalPiston = temp_clone.into();
        real.get_type()
    }
    // SUPER CLASS: Directional
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
impl<'mc> Into<crate::block::data::mod_type::TechnicalPiston<'mc>> for PistonHead<'mc> {
    fn into(self) -> crate::block::data::mod_type::TechnicalPiston<'mc> {
        crate::block::data::mod_type::TechnicalPiston::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting PistonHead into crate::block::data::mod_type::TechnicalPiston",
        )
    }
}
/// 'type' represents which part of a double chest this block is, or if it is a single chest.
///
/// This is a representation of an abstract class.
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
    pub fn set_type(
        &self,
        arg0: impl Into<crate::block::data::mod_type::ChestType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Chest$Type;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn get_type(
        &self,
    ) -> Result<crate::block::data::mod_type::ChestType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Chest$Type;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::ChestType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Chest::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
    }
    // SUPER CLASS: BlockData

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

///
/// This is a representation of an abstract class.
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

///
/// This is a representation of an abstract class.
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

///
/// This is a representation of an abstract class.
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
/// 'has_book' is a quick flag to check whether this lectern has a book inside it.
///
/// This is a representation of an abstract class.
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
    pub fn has_book(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasBook", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Lectern::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Lectern::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Lectern::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.is_powered()
    }
    pub fn set_powered(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Lectern::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.set_powered(arg0)
    }
    // SUPER CLASS: BlockData

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
/// 'inverted' denotes whether this daylight detector is in the inverted mode, i.e. activates in the absence of light rather than presence."
///
/// This is a representation of an abstract class.
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
    pub fn is_inverted(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isInverted", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'inverted' property.
    pub fn set_inverted(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInverted",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = DaylightDetector::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::AnaloguePowerable = temp_clone.into();
        real.power()
    }
    pub fn set_power(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = DaylightDetector::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::AnaloguePowerable = temp_clone.into();
        real.set_power(arg0)
    }
    pub fn maximum_power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = DaylightDetector::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::AnaloguePowerable = temp_clone.into();
        real.maximum_power()
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
/// 'mode' represents the different modes in which this structure block may operate.
///
/// This is a representation of an abstract class.
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
    pub fn mode(
        &self,
    ) -> Result<crate::block::data::mod_type::StructureBlockMode<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/StructureBlock$Mode;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::StructureBlockMode::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_mode(
        &self,
        arg0: impl Into<crate::block::data::mod_type::StructureBlockMode<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/StructureBlock$Mode;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = StructureBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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
/// 'flower_amount' represents the number of petals.
///
/// This is a representation of an abstract class.
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
    pub fn flower_amount(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFlowerAmount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'flower_amount' property.
    pub fn set_flower_amount(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFlowerAmount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = PinkPetals::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PinkPetals::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
/// 'extended' denotes whether the piston head is currently extended or not.
///
/// This is a representation of an abstract class.
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
    pub fn is_extended(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isExtended", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'extended' property.
    pub fn set_extended(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setExtended",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Piston::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Piston::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
/// 'unstable' indicates whether this TNT will explode on punching.
///
/// This is a representation of an abstract class.
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
    pub fn is_unstable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnstable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'unstable' property.
    pub fn set_unstable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnstable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TNT::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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

///
/// This is a representation of an abstract class.
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
/// The tilt of a leaf.
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

impl<'mc> BigDripleafTilt<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BigDripleafTilt<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/BigDripleaf$Tilt");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/BigDripleaf$Tilt;",
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
            env.validate_name(&obj, "org/bukkit/block/data/type/BigDripleaf$Tilt")?;
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
            env.validate_name(&obj, "org/bukkit/block/data/type/BigDripleaf$Tilt")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::BigDripleafTilt<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Lorg/bukkit/block/data/type/BigDripleaf$Tilt;");
        let cls = jni.find_class("org/bukkit/block/data/type/BigDripleaf$Tilt");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::BigDripleafTilt::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum Hinge<'mc> {
    Left { inner: HingeStruct<'mc> },
    Right { inner: HingeStruct<'mc> },
}
impl<'mc> std::fmt::Display for Hinge<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hinge::Left { .. } => f.write_str("LEFT"),
            Hinge::Right { .. } => f.write_str("RIGHT"),
        }
    }
}

impl<'mc> Hinge<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Hinge<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Hinge");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Hinge;",
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
            "LEFT" => Ok(Hinge::Left {
                inner: HingeStruct::from_raw(env, obj)?,
            }),
            "RIGHT" => Ok(Hinge::Right {
                inner: HingeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct HingeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Hinge<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Hinge<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Hinge from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Hinge")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Hinge object, got {}",
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
                "LEFT" => Ok(Hinge::Left {
                    inner: HingeStruct::from_raw(env, obj)?,
                }),
                "RIGHT" => Ok(Hinge::Right {
                    inner: HingeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for HingeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HingeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HingeStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Hinge")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HingeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HingeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The different heights a face of a wall may have.
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

impl<'mc> WallHeight<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<WallHeight<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Wall$Height");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Wall$Height;",
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Wall$Height")?;
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Wall$Height")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::WallHeight<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Lorg/bukkit/block/data/type/Wall$Height;");
        let cls = jni.find_class("org/bukkit/block/data/type/Wall$Height");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::WallHeight::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'bloom' indicates whether the sculk catalyst is actively spreading the sculk or not.
///
/// This is a representation of an abstract class.
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
    pub fn is_bloom(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBloom", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'bloom' property.
    pub fn set_bloom(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBloom",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = SculkCatalyst::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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
/// The directions the Jigsaw can be oriented.
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

impl<'mc> JigsawOrientation<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<JigsawOrientation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Jigsaw$Orientation");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Jigsaw$Orientation;",
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
            env.validate_name(&obj, "org/bukkit/block/data/type/Jigsaw$Orientation")?;
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
            env.validate_name(&obj, "org/bukkit/block/data/type/Jigsaw$Orientation")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::JigsawOrientation<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Lorg/bukkit/block/data/type/Jigsaw$Orientation;");
        let cls = jni.find_class("org/bukkit/block/data/type/Jigsaw$Orientation");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::JigsawOrientation::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum Orientation<'mc> {
    DownEast { inner: OrientationStruct<'mc> },
    DownNorth { inner: OrientationStruct<'mc> },
    DownSouth { inner: OrientationStruct<'mc> },
    DownWest { inner: OrientationStruct<'mc> },
    UpEast { inner: OrientationStruct<'mc> },
    UpNorth { inner: OrientationStruct<'mc> },
    UpSouth { inner: OrientationStruct<'mc> },
    UpWest { inner: OrientationStruct<'mc> },
    WestUp { inner: OrientationStruct<'mc> },
    EastUp { inner: OrientationStruct<'mc> },
    NorthUp { inner: OrientationStruct<'mc> },
    SouthUp { inner: OrientationStruct<'mc> },
}
impl<'mc> std::fmt::Display for Orientation<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::DownEast { .. } => f.write_str("DOWN_EAST"),
            Orientation::DownNorth { .. } => f.write_str("DOWN_NORTH"),
            Orientation::DownSouth { .. } => f.write_str("DOWN_SOUTH"),
            Orientation::DownWest { .. } => f.write_str("DOWN_WEST"),
            Orientation::UpEast { .. } => f.write_str("UP_EAST"),
            Orientation::UpNorth { .. } => f.write_str("UP_NORTH"),
            Orientation::UpSouth { .. } => f.write_str("UP_SOUTH"),
            Orientation::UpWest { .. } => f.write_str("UP_WEST"),
            Orientation::WestUp { .. } => f.write_str("WEST_UP"),
            Orientation::EastUp { .. } => f.write_str("EAST_UP"),
            Orientation::NorthUp { .. } => f.write_str("NORTH_UP"),
            Orientation::SouthUp { .. } => f.write_str("SOUTH_UP"),
        }
    }
}

impl<'mc> Orientation<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Orientation<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Orientation");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Orientation;",
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
            "DOWN_EAST" => Ok(Orientation::DownEast {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),
            "DOWN_NORTH" => Ok(Orientation::DownNorth {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),
            "DOWN_SOUTH" => Ok(Orientation::DownSouth {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),
            "DOWN_WEST" => Ok(Orientation::DownWest {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),
            "UP_EAST" => Ok(Orientation::UpEast {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),
            "UP_NORTH" => Ok(Orientation::UpNorth {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),
            "UP_SOUTH" => Ok(Orientation::UpSouth {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),
            "UP_WEST" => Ok(Orientation::UpWest {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),
            "WEST_UP" => Ok(Orientation::WestUp {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),
            "EAST_UP" => Ok(Orientation::EastUp {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),
            "NORTH_UP" => Ok(Orientation::NorthUp {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),
            "SOUTH_UP" => Ok(Orientation::SouthUp {
                inner: OrientationStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct OrientationStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Orientation<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Orientation<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Orientation from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Orientation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Orientation object, got {}",
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
                "DOWN_EAST" => Ok(Orientation::DownEast {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                "DOWN_NORTH" => Ok(Orientation::DownNorth {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                "DOWN_SOUTH" => Ok(Orientation::DownSouth {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                "DOWN_WEST" => Ok(Orientation::DownWest {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                "UP_EAST" => Ok(Orientation::UpEast {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                "UP_NORTH" => Ok(Orientation::UpNorth {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                "UP_SOUTH" => Ok(Orientation::UpSouth {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                "UP_WEST" => Ok(Orientation::UpWest {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                "WEST_UP" => Ok(Orientation::WestUp {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                "EAST_UP" => Ok(Orientation::EastUp {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                "NORTH_UP" => Ok(Orientation::NorthUp {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                "SOUTH_UP" => Ok(Orientation::SouthUp {
                    inner: OrientationStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for OrientationStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for OrientationStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate OrientationStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Orientation")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a OrientationStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> OrientationStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
/// 'stage' represents the growth stage of a sapling.
///
/// When the sapling reaches <a href="#getMaximumStage()"><code>getMaximumStage()</code></a> it will attempt to grow into a tree as the next stage.
///
/// This is a representation of an abstract class.
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
    pub fn stage(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getStage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'stage' property.
    pub fn set_stage(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setStage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn maximum_stage(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumStage", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Sapling::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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
pub enum Thickness<'mc> {
    TipMerge { inner: ThicknessStruct<'mc> },
    Tip { inner: ThicknessStruct<'mc> },
    Frustum { inner: ThicknessStruct<'mc> },
    Middle { inner: ThicknessStruct<'mc> },
    Base { inner: ThicknessStruct<'mc> },
}
impl<'mc> std::fmt::Display for Thickness<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Thickness::TipMerge { .. } => f.write_str("TIP_MERGE"),
            Thickness::Tip { .. } => f.write_str("TIP"),
            Thickness::Frustum { .. } => f.write_str("FRUSTUM"),
            Thickness::Middle { .. } => f.write_str("MIDDLE"),
            Thickness::Base { .. } => f.write_str("BASE"),
        }
    }
}

impl<'mc> Thickness<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Thickness<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Thickness");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Thickness;",
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
            "TIP_MERGE" => Ok(Thickness::TipMerge {
                inner: ThicknessStruct::from_raw(env, obj)?,
            }),
            "TIP" => Ok(Thickness::Tip {
                inner: ThicknessStruct::from_raw(env, obj)?,
            }),
            "FRUSTUM" => Ok(Thickness::Frustum {
                inner: ThicknessStruct::from_raw(env, obj)?,
            }),
            "MIDDLE" => Ok(Thickness::Middle {
                inner: ThicknessStruct::from_raw(env, obj)?,
            }),
            "BASE" => Ok(Thickness::Base {
                inner: ThicknessStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ThicknessStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Thickness<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Thickness<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Thickness from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Thickness")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Thickness object, got {}",
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
                "TIP_MERGE" => Ok(Thickness::TipMerge {
                    inner: ThicknessStruct::from_raw(env, obj)?,
                }),
                "TIP" => Ok(Thickness::Tip {
                    inner: ThicknessStruct::from_raw(env, obj)?,
                }),
                "FRUSTUM" => Ok(Thickness::Frustum {
                    inner: ThicknessStruct::from_raw(env, obj)?,
                }),
                "MIDDLE" => Ok(Thickness::Middle {
                    inner: ThicknessStruct::from_raw(env, obj)?,
                }),
                "BASE" => Ok(Thickness::Base {
                    inner: ThicknessStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ThicknessStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ThicknessStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ThicknessStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Thickness")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ThicknessStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ThicknessStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The Phase of the sensor.
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

impl<'mc> SculkSensorPhase<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SculkSensorPhase<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/SculkSensor$Phase");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/SculkSensor$Phase;",
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
            env.validate_name(&obj, "org/bukkit/block/data/type/SculkSensor$Phase")?;
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
            env.validate_name(&obj, "org/bukkit/block/data/type/SculkSensor$Phase")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::SculkSensorPhase<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Lorg/bukkit/block/data/type/SculkSensor$Phase;");
        let cls = jni.find_class("org/bukkit/block/data/type/SculkSensor$Phase");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::SculkSensorPhase::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
/// 'type' represents the type of piston which this (technical) block corresponds to.
///
/// This is a representation of an abstract class.
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
    pub fn set_type(
        &self,
        arg0: impl Into<crate::block::data::mod_type::TechnicalPistonType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/TechnicalPiston$Type;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn get_type(
        &self,
    ) -> Result<crate::block::data::mod_type::TechnicalPistonType<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/TechnicalPiston$Type;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::TechnicalPistonType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = TechnicalPiston::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TechnicalPiston::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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

///
/// This is a representation of an abstract class.
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

    pub fn face(
        &self,
    ) -> Result<crate::block::data::mod_type::SwitchFace<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Switch$Face;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::SwitchFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_face(
        &self,
        arg0: impl Into<crate::block::data::mod_type::SwitchFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Switch$Face;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Switch::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Switch::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn attached_face(
        &self,
    ) -> Result<crate::block::data::FaceAttachableAttachedFace<'mc>, Box<dyn std::error::Error>>
    {
        let temp_clone = Switch::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::FaceAttachable = temp_clone.into();
        real.attached_face()
    }
    pub fn set_attached_face(
        &self,
        arg0: impl Into<crate::block::data::FaceAttachableAttachedFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Switch::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::FaceAttachable = temp_clone.into();
        real.set_attached_face(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Switch::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.is_powered()
    }
    pub fn set_powered(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Switch::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.set_powered(arg0)
    }
    // SUPER CLASS: BlockData

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
pub enum Part<'mc> {
    Head { inner: PartStruct<'mc> },
    Foot { inner: PartStruct<'mc> },
}
impl<'mc> std::fmt::Display for Part<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Head { .. } => f.write_str("HEAD"),
            Part::Foot { .. } => f.write_str("FOOT"),
        }
    }
}

impl<'mc> Part<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Part<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Part");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Part;",
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
            "HEAD" => Ok(Part::Head {
                inner: PartStruct::from_raw(env, obj)?,
            }),
            "FOOT" => Ok(Part::Foot {
                inner: PartStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PartStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Part<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Part<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Part from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Part")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Part object, got {}",
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
                "HEAD" => Ok(Part::Head {
                    inner: PartStruct::from_raw(env, obj)?,
                }),
                "FOOT" => Ok(Part::Foot {
                    inner: PartStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PartStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PartStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PartStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Part")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PartStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PartStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'drag' indicates whether a force will be applied on entities moving through this block.
///
/// This is a representation of an abstract class.
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
    pub fn is_drag(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isDrag", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'drag' property.
    pub fn set_drag(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDrag",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BubbleColumn::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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
/// 'attachment' denotes how the bell is attached to its block.
///
/// This is a representation of an abstract class.
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
    pub fn attachment(
        &self,
    ) -> Result<crate::block::data::mod_type::BellAttachment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Bell$Attachment;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachment", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::BellAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_attachment(
        &self,
        arg0: impl Into<crate::block::data::mod_type::BellAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Bell$Attachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Bell::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Bell::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Bell::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.is_powered()
    }
    pub fn set_powered(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Bell::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.set_powered(arg0)
    }
    // SUPER CLASS: BlockData

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
/// 'leaves' represents the size of the leaves on this bamboo block.
///
/// This is a representation of an abstract class.
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
    pub fn leaves(
        &self,
    ) -> Result<crate::block::data::mod_type::BambooLeaves<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Bamboo$Leaves;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLeaves", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::BambooLeaves::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_leaves(
        &self,
        arg0: impl Into<crate::block::data::mod_type::BambooLeaves<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Bamboo$Leaves;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Bamboo::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Ageable = temp_clone.into();
        real.age()
    }
    pub fn set_age(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Bamboo::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Ageable = temp_clone.into();
        real.set_age(arg0)
    }
    pub fn maximum_age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Bamboo::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Ageable = temp_clone.into();
        real.maximum_age()
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn stage(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Bamboo::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::mod_type::Sapling = temp_clone.into();
        real.stage()
    }
    pub fn set_stage(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Bamboo::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::mod_type::Sapling = temp_clone.into();
        real.set_stage(arg0)
    }
    pub fn maximum_stage(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Bamboo::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::mod_type::Sapling = temp_clone.into();
        real.maximum_stage()
    }
    // SUPER CLASS: BlockData

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
/// 'orientation' is the direction the block is facing.
///
/// This is a representation of an abstract class.
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
    pub fn orientation(
        &self,
    ) -> Result<crate::block::data::mod_type::JigsawOrientation<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/Jigsaw$Orientation;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOrientation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::JigsawOrientation::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_orientation(
        &self,
        arg0: impl Into<crate::block::data::mod_type::JigsawOrientation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Jigsaw$Orientation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jigsaw::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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

///
/// This is a representation of an abstract class.
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

///
/// This is a representation of an abstract class.
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
/// Similar to <a title="interface in org.bukkit.block.data" href="../Powerable.html"><code>Powerable</code></a>, 'triggered' indicates whether or not the dispenser is currently activated.
///
/// This is a representation of an abstract class.
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
    pub fn is_triggered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isTriggered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'triggered' property.
    pub fn set_triggered(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTriggered",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Dispenser::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Dispenser::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
pub enum Shape<'mc> {
    Straight { inner: ShapeStruct<'mc> },
    InnerLeft { inner: ShapeStruct<'mc> },
    InnerRight { inner: ShapeStruct<'mc> },
    OuterLeft { inner: ShapeStruct<'mc> },
    OuterRight { inner: ShapeStruct<'mc> },
}
impl<'mc> std::fmt::Display for Shape<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Straight { .. } => f.write_str("STRAIGHT"),
            Shape::InnerLeft { .. } => f.write_str("INNER_LEFT"),
            Shape::InnerRight { .. } => f.write_str("INNER_RIGHT"),
            Shape::OuterLeft { .. } => f.write_str("OUTER_LEFT"),
            Shape::OuterRight { .. } => f.write_str("OUTER_RIGHT"),
        }
    }
}

impl<'mc> Shape<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Shape<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Shape");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Shape;",
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
            "STRAIGHT" => Ok(Shape::Straight {
                inner: ShapeStruct::from_raw(env, obj)?,
            }),
            "INNER_LEFT" => Ok(Shape::InnerLeft {
                inner: ShapeStruct::from_raw(env, obj)?,
            }),
            "INNER_RIGHT" => Ok(Shape::InnerRight {
                inner: ShapeStruct::from_raw(env, obj)?,
            }),
            "OUTER_LEFT" => Ok(Shape::OuterLeft {
                inner: ShapeStruct::from_raw(env, obj)?,
            }),
            "OUTER_RIGHT" => Ok(Shape::OuterRight {
                inner: ShapeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ShapeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Shape<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Shape<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Shape from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Shape")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Shape object, got {}",
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
                "STRAIGHT" => Ok(Shape::Straight {
                    inner: ShapeStruct::from_raw(env, obj)?,
                }),
                "INNER_LEFT" => Ok(Shape::InnerLeft {
                    inner: ShapeStruct::from_raw(env, obj)?,
                }),
                "INNER_RIGHT" => Ok(Shape::InnerRight {
                    inner: ShapeStruct::from_raw(env, obj)?,
                }),
                "OUTER_LEFT" => Ok(Shape::OuterLeft {
                    inner: ShapeStruct::from_raw(env, obj)?,
                }),
                "OUTER_RIGHT" => Ok(Shape::OuterRight {
                    inner: ShapeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ShapeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ShapeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ShapeStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Shape")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ShapeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ShapeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Operating mode of a structure block.
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

impl<'mc> StructureBlockMode<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<StructureBlockMode<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/StructureBlock$Mode");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/StructureBlock$Mode;",
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
            env.validate_name(&obj, "org/bukkit/block/data/type/StructureBlock$Mode")?;
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
            env.validate_name(&obj, "org/bukkit/block/data/type/StructureBlock$Mode")?;
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
    ) -> Result<
        Vec<crate::block::data::mod_type::StructureBlockMode<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/block/data/type/StructureBlock$Mode;");
        let cls = jni.find_class("org/bukkit/block/data/type/StructureBlock$Mode");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::StructureBlockMode::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'charges' represents the amount of times the anchor may still be used.
///
/// This is a representation of an abstract class.
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
    pub fn charges(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCharges", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'charges' property.
    pub fn set_charges(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCharges",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = RespawnAnchor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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
/// 'bites' represents the amount of bites which have been taken from this slice of cake.
///
/// A value of 0 indicates that the cake has not been eaten, whilst a value of <a href="#getMaximumBites()"><code>getMaximumBites()</code></a> indicates that it is all gone :(
///
/// This is a representation of an abstract class.
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
    pub fn bites(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBites", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'bites' property.
    pub fn set_bites(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBites",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn maximum_bites(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumBites", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Cake::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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
/// 'eye' denotes whether this end portal frame has been activated by having an eye of ender placed in it.
///
/// This is a representation of an abstract class.
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
    pub fn has_eye(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasEye", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'eye' property.
    pub fn set_eye(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEye",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = EndPortalFrame::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = EndPortalFrame::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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

///
/// This is a representation of an abstract class.
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

///
/// This is a representation of an abstract class.
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
/// The mode in which a comparator will operate in.
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

impl<'mc> ComparatorMode<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ComparatorMode<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Comparator$Mode");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Comparator$Mode;",
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
            env.validate_name(&obj, "org/bukkit/block/data/type/Comparator$Mode")?;
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
            env.validate_name(&obj, "org/bukkit/block/data/type/Comparator$Mode")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::ComparatorMode<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Lorg/bukkit/block/data/type/Comparator$Mode;");
        let cls = jni.find_class("org/bukkit/block/data/type/Comparator$Mode");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::ComparatorMode::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
/// What the bell is attached to.
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

impl<'mc> BellAttachment<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BellAttachment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Bell$Attachment");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Bell$Attachment;",
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
            env.validate_name(&obj, "org/bukkit/block/data/type/Bell$Attachment")?;
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
            env.validate_name(&obj, "org/bukkit/block/data/type/Bell$Attachment")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::BellAttachment<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Lorg/bukkit/block/data/type/Bell$Attachment;");
        let cls = jni.find_class("org/bukkit/block/data/type/Bell$Attachment");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::BellAttachment::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'shape' represents the texture and bounding box shape of these stairs.
///
/// This is a representation of an abstract class.
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
    pub fn shape(
        &self,
    ) -> Result<crate::block::data::mod_type::StairsShape<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Stairs$Shape;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getShape", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::StairsShape::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_shape(
        &self,
        arg0: impl Into<crate::block::data::mod_type::StairsShape<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Stairs$Shape;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn half(
        &self,
    ) -> Result<crate::block::data::BisectedHalf<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Stairs::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Bisected = temp_clone.into();
        real.half()
    }
    pub fn set_half(
        &self,
        arg0: impl Into<crate::block::data::BisectedHalf<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Stairs::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Bisected = temp_clone.into();
        real.set_half(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Stairs::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Stairs::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Stairs::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Stairs::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
    }
    // SUPER CLASS: BlockData

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
/// The hinge of a door.
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

impl<'mc> DoorHinge<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<DoorHinge<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Door$Hinge");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Door$Hinge;",
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Door$Hinge")?;
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Door$Hinge")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::DoorHinge<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/block/data/type/Door$Hinge;");
        let cls = jni.find_class("org/bukkit/block/data/type/Door$Hinge");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::DoorHinge::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
pub enum Face<'mc> {
    Floor { inner: FaceStruct<'mc> },
    Wall { inner: FaceStruct<'mc> },
    Ceiling { inner: FaceStruct<'mc> },
}
impl<'mc> std::fmt::Display for Face<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Face::Floor { .. } => f.write_str("FLOOR"),
            Face::Wall { .. } => f.write_str("WALL"),
            Face::Ceiling { .. } => f.write_str("CEILING"),
        }
    }
}

impl<'mc> Face<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Face<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Face");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Face;",
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
            "FLOOR" => Ok(Face::Floor {
                inner: FaceStruct::from_raw(env, obj)?,
            }),
            "WALL" => Ok(Face::Wall {
                inner: FaceStruct::from_raw(env, obj)?,
            }),
            "CEILING" => Ok(Face::Ceiling {
                inner: FaceStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct FaceStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Face<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Face<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Face from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Face")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Face object, got {}",
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
                "FLOOR" => Ok(Face::Floor {
                    inner: FaceStruct::from_raw(env, obj)?,
                }),
                "WALL" => Ok(Face::Wall {
                    inner: FaceStruct::from_raw(env, obj)?,
                }),
                "CEILING" => Ok(Face::Ceiling {
                    inner: FaceStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for FaceStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FaceStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate FaceStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Face")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FaceStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FaceStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
/// Similar to <a title="interface in org.bukkit.block.data" href="../Bisected.html"><code>Bisected</code></a>, 'part' denotes which half of the bed this block corresponds to.
///
/// 'occupied' property is a quick flag to check if a player is currently sleeping in this bed block.
///
/// This is a representation of an abstract class.
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
    pub fn part(
        &self,
    ) -> Result<crate::block::data::mod_type::BedPart<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Bed$Part;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPart", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::BedPart::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_part(
        &self,
        arg0: impl Into<crate::block::data::mod_type::BedPart<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Bed$Part;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn is_occupied(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccupied", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Bed::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Bed::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
/// Type of this chest block.
///
/// NB: Left and right are relative to the chest itself, i.e opposite to what a player placing the appropriate block would see.
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

impl<'mc> ChestType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<ChestType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Chest$Type");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Chest$Type;",
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Chest$Type")?;
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Chest$Type")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::ChestType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/block/data/type/Chest$Type;");
        let cls = jni.find_class("org/bukkit/block/data/type/Chest$Type");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::ChestType::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
/// 'instrument' is the type of sound made when this note block is activated.
///
/// 'note' is the specified tuned pitch that the instrument will be played in.
///
/// This is a representation of an abstract class.
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
    pub fn instrument(&self) -> Result<Option<crate::Instrument<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Instrument;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getInstrument", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Instrument::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn set_instrument(
        &self,
        arg0: impl Into<crate::Instrument<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Instrument;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn set_note(
        &self,
        arg0: impl Into<crate::Note<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Note;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = NoteBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.is_powered()
    }
    pub fn set_powered(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = NoteBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.set_powered(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
pub enum Connection<'mc> {
    Up { inner: ConnectionStruct<'mc> },
    Side { inner: ConnectionStruct<'mc> },
    None { inner: ConnectionStruct<'mc> },
}
impl<'mc> std::fmt::Display for Connection<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Connection::Up { .. } => f.write_str("UP"),
            Connection::Side { .. } => f.write_str("SIDE"),
            Connection::None { .. } => f.write_str("NONE"),
        }
    }
}

impl<'mc> Connection<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Connection<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Connection");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Connection;",
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
            "UP" => Ok(Connection::Up {
                inner: ConnectionStruct::from_raw(env, obj)?,
            }),
            "SIDE" => Ok(Connection::Side {
                inner: ConnectionStruct::from_raw(env, obj)?,
            }),
            "NONE" => Ok(Connection::None {
                inner: ConnectionStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ConnectionStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Connection<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Connection<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Connection from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Connection")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Connection object, got {}",
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
                "UP" => Ok(Connection::Up {
                    inner: ConnectionStruct::from_raw(env, obj)?,
                }),
                "SIDE" => Ok(Connection::Side {
                    inner: ConnectionStruct::from_raw(env, obj)?,
                }),
                "NONE" => Ok(Connection::None {
                    inner: ConnectionStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ConnectionStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ConnectionStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ConnectionStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Connection")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ConnectionStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ConnectionStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
/// 'sculk_sensor_phase' indicates the current operational phase of the sensor.
///
/// This is a representation of an abstract class.
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
    pub fn phase(
        &self,
    ) -> Result<crate::block::data::mod_type::SculkSensorPhase<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/SculkSensor$Phase;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPhase", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::SculkSensorPhase::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_phase(
        &self,
        arg0: impl Into<crate::block::data::mod_type::SculkSensorPhase<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/SculkSensor$Phase;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SculkSensor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::AnaloguePowerable = temp_clone.into();
        real.power()
    }
    pub fn set_power(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SculkSensor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::AnaloguePowerable = temp_clone.into();
        real.set_power(arg0)
    }
    pub fn maximum_power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = SculkSensor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::AnaloguePowerable = temp_clone.into();
        real.maximum_power()
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SculkSensor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SculkSensor::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
    }
    // SUPER CLASS: BlockData

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
/// 'tilt' indicates how far the leaf is tilted.
///
/// This is a representation of an abstract class.
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
    pub fn tilt(
        &self,
    ) -> Result<crate::block::data::mod_type::BigDripleafTilt<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/type/BigDripleaf$Tilt;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTilt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::BigDripleafTilt::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_tilt(
        &self,
        arg0: impl Into<crate::block::data::mod_type::BigDripleafTilt<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/BigDripleaf$Tilt;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    // SUPER CLASS: Directional
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: Waterlogged
    // SUPER CLASS: Cloneable
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Waterlogged::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::Waterlogged::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
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

///
/// This is a representation of an abstract class.
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
/// 'bottom' indicates whether the scaffolding is floating or not.
///
/// 'distance' indicates the distance from a scaffolding block placed above a 'bottom' scaffold.
///
/// When 'distance' reaches <a href="#getMaximumDistance()"><code>getMaximumDistance()</code></a> the block will drop.
///
/// This is a representation of an abstract class.
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
    pub fn distance(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'distance' property.
    pub fn set_distance(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDistance",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_bottom(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBottom", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'bottom' property.
    pub fn set_bottom(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBottom",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Scaffolding::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Scaffolding::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
/// Interface to the 'has_bottle_0', 'has_bottle_1', 'has_bottle_2' flags on a brewing stand which indicate which bottles are rendered on the outside.
///
/// Stand may have 0, 1... <a href="#getMaximumBottles()"><code>getMaximumBottles()</code></a>-1 bottles.
///
/// This is a representation of an abstract class.
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
    pub fn has_bottle(&self, arg0: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Z");
        let val_1 = jni::objects::JValueGen::Int(arg0);
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
    pub fn set_bottle(&self, arg0: i32, arg1: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(IZ)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = BrewingStand::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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
/// md_5's mixtape.
///
/// This is a representation of an abstract class.
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
/// Similar to <a href="../Powerable.html" title="interface in org.bukkit.block.data"><code>Powerable</code></a>, 'enabled' indicates whether or not the hopper is currently activated.
///
/// Unlike most other blocks, a hopper is only enabled when it is <b>not</b> receiving any power.
///
/// This is a representation of an abstract class.
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
    pub fn is_enabled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'enabled' property.
    pub fn set_enabled(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
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
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Hopper::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Hopper::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
/// 'delay' is the propagation delay of a repeater, i.e. how many ticks before it will be activated from a current change and propagate it to the next block.
///
/// Delay may not be lower than <a href="#getMinimumDelay()"><code>getMinimumDelay()</code></a> or higher than <a href="#getMaximumDelay()"><code>getMaximumDelay()</code></a>.
///
/// 'locked' denotes whether the repeater is in the locked state or not.
///
/// A locked repeater will not change its output until it is unlocked. In game, a locked repeater is created by having a constant current perpendicularly entering the block.
///
/// This is a representation of an abstract class.
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
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'locked' property.
    pub fn set_locked(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocked",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDelay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'delay' property.
    pub fn set_delay(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDelay",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn minimum_delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinimumDelay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn maximum_delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumDelay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Repeater::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Repeater::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Repeater::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.is_powered()
    }
    pub fn set_powered(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Repeater::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.set_powered(arg0)
    }
    // SUPER CLASS: BlockData

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
/// 'type' represents what state the slab is in - either top, bottom, or a double slab occupying the full block.
///
/// This is a representation of an abstract class.
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
    pub fn set_type(
        &self,
        arg0: impl Into<crate::block::data::mod_type::SlabType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Slab$Type;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn get_type(
        &self,
    ) -> Result<crate::block::data::mod_type::SlabType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Slab$Type;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::SlabType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Slab::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Slab::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
/// 'layers' represents the amount of layers of snow which are present in this block.
///
/// May not be lower than <a href="#getMinimumLayers()"><code>getMinimumLayers()</code></a> or higher than <a href="#getMaximumLayers()"><code>getMaximumLayers()</code></a>.
///
/// This is a representation of an abstract class.
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
    pub fn layers(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLayers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'layers' property.
    pub fn set_layers(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLayers",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snow::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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
/// The way in which a redstone wire can connect to an adjacent block face.
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

impl<'mc> RedstoneWireConnection<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<RedstoneWireConnection<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/RedstoneWire$Connection");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/RedstoneWire$Connection;",
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
            env.validate_name(&obj, "org/bukkit/block/data/type/RedstoneWire$Connection")?;
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
            env.validate_name(&obj, "org/bukkit/block/data/type/RedstoneWire$Connection")?;
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
    ) -> Result<
        Vec<crate::block::data::mod_type::RedstoneWireConnection<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/block/data/type/RedstoneWire$Connection;");
        let cls = jni.find_class("org/bukkit/block/data/type/RedstoneWire$Connection");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({
                crate::block::data::mod_type::RedstoneWireConnection::from_raw(&jni, res)?
            });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum Attachment<'mc> {
    Floor { inner: AttachmentStruct<'mc> },
    Ceiling { inner: AttachmentStruct<'mc> },
    SingleWall { inner: AttachmentStruct<'mc> },
    DoubleWall { inner: AttachmentStruct<'mc> },
}
impl<'mc> std::fmt::Display for Attachment<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attachment::Floor { .. } => f.write_str("FLOOR"),
            Attachment::Ceiling { .. } => f.write_str("CEILING"),
            Attachment::SingleWall { .. } => f.write_str("SINGLE_WALL"),
            Attachment::DoubleWall { .. } => f.write_str("DOUBLE_WALL"),
        }
    }
}

impl<'mc> Attachment<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Attachment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Attachment");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Attachment;",
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
            "FLOOR" => Ok(Attachment::Floor {
                inner: AttachmentStruct::from_raw(env, obj)?,
            }),
            "CEILING" => Ok(Attachment::Ceiling {
                inner: AttachmentStruct::from_raw(env, obj)?,
            }),
            "SINGLE_WALL" => Ok(Attachment::SingleWall {
                inner: AttachmentStruct::from_raw(env, obj)?,
            }),
            "DOUBLE_WALL" => Ok(Attachment::DoubleWall {
                inner: AttachmentStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct AttachmentStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Attachment<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Attachment<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Attachment from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Attachment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Attachment object, got {}",
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
                "FLOOR" => Ok(Attachment::Floor {
                    inner: AttachmentStruct::from_raw(env, obj)?,
                }),
                "CEILING" => Ok(Attachment::Ceiling {
                    inner: AttachmentStruct::from_raw(env, obj)?,
                }),
                "SINGLE_WALL" => Ok(Attachment::SingleWall {
                    inner: AttachmentStruct::from_raw(env, obj)?,
                }),
                "DOUBLE_WALL" => Ok(Attachment::DoubleWall {
                    inner: AttachmentStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for AttachmentStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AttachmentStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AttachmentStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Attachment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AttachmentStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AttachmentStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
/// 'mode' indicates what mode this comparator will operate in.
///
/// This is a representation of an abstract class.
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
    pub fn mode(
        &self,
    ) -> Result<crate::block::data::mod_type::ComparatorMode<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Comparator$Mode;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::ComparatorMode::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_mode(
        &self,
        arg0: impl Into<crate::block::data::mod_type::ComparatorMode<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Comparator$Mode;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Comparator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Comparator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Comparator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.is_powered()
    }
    pub fn set_powered(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Comparator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.set_powered(arg0)
    }
    // SUPER CLASS: BlockData

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
/// A type of minecart rail which interacts with redstone in one way or another.
///
/// This is a representation of an abstract class.
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
/// 'berries' indicates whether the block has berries.
///
/// This is a representation of an abstract class.
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
    pub fn is_berries(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBerries", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'berries' property.
    pub fn set_berries(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBerries",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CaveVinesPlant::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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

///
/// This is a representation of an abstract class.
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

///
/// This is a representation of an abstract class.
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

///
/// This is a representation of an abstract class.
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

///
/// This is a representation of an abstract class.
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
/// Represents the thickness of the dripstone, corresponding to its position within a multi-block dripstone formation.
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

impl<'mc> PointedDripstoneThickness<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PointedDripstoneThickness<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/PointedDripstone$Thickness");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/PointedDripstone$Thickness;",
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
            "org/bukkit/block/data/type/PointedDripstone$Thickness",
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
            "org/bukkit/block/data/type/PointedDripstone$Thickness",
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
        Vec<crate::block::data::mod_type::PointedDripstoneThickness<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/block/data/type/PointedDripstone$Thickness;");
        let cls = jni.find_class("org/bukkit/block/data/type/PointedDripstone$Thickness");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({
                crate::block::data::mod_type::PointedDripstoneThickness::from_raw(&jni, res)?
            });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'honey_level' represents the amount of honey stored in the hive.
///
/// This is a representation of an abstract class.
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

    pub fn honey_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHoneyLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'honey_level' property.
    pub fn set_honey_level(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHoneyLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Beehive::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Beehive::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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

///
/// This is a representation of an abstract class.
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

///
/// This is a representation of an abstract class.
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
/// The type of the slab.
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

impl<'mc> SlabType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SlabType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Slab$Type");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Slab$Type;",
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Slab$Type")?;
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Slab$Type")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::SlabType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/block/data/type/Slab$Type;");
        let cls = jni.find_class("org/bukkit/block/data/type/Slab$Type");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::SlabType::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'disarmed' denotes that the tripwire was broken with shears and will not subsequently produce a current when destroyed.
///
/// This is a representation of an abstract class.
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
    pub fn is_disarmed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDisarmed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'disarmed' property.
    pub fn set_disarmed(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDisarmed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_attached(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Tripwire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Attachable = temp_clone.into();
        real.is_attached()
    }
    pub fn set_attached(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Tripwire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Attachable = temp_clone.into();
        real.set_attached(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
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
    pub fn set_face(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Tripwire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::MultipleFacing = temp_clone.into();
        real.set_face(arg0, arg1)
    }
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
    pub fn has_face(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Tripwire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::MultipleFacing = temp_clone.into();
        real.has_face(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Tripwire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.is_powered()
    }
    pub fn set_powered(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Tripwire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.set_powered(arg0)
    }
    // SUPER CLASS: BlockData

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
/// This class encompasses the 'north', 'east', 'south', 'west', height flags which are used to set the height of a wall. 'up' denotes whether the well has a center post.
///
/// This is a representation of an abstract class.
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
    pub fn get_height(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<crate::block::data::mod_type::WallHeight<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/BlockFace;)Lorg/bukkit/block/data/type/Wall$Height;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn is_up(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'up' property.
    pub fn set_up(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_height(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::data::mod_type::WallHeight<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/data/type/Wall$Height;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
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
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Wall::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Wall::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
pub enum Phase<'mc> {
    Inactive { inner: PhaseStruct<'mc> },
    Active { inner: PhaseStruct<'mc> },
    Cooldown { inner: PhaseStruct<'mc> },
}
impl<'mc> std::fmt::Display for Phase<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Phase::Inactive { .. } => f.write_str("INACTIVE"),
            Phase::Active { .. } => f.write_str("ACTIVE"),
            Phase::Cooldown { .. } => f.write_str("COOLDOWN"),
        }
    }
}

impl<'mc> Phase<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Phase<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Phase");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Phase;",
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
            "INACTIVE" => Ok(Phase::Inactive {
                inner: PhaseStruct::from_raw(env, obj)?,
            }),
            "ACTIVE" => Ok(Phase::Active {
                inner: PhaseStruct::from_raw(env, obj)?,
            }),
            "COOLDOWN" => Ok(Phase::Cooldown {
                inner: PhaseStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PhaseStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Phase<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Phase<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Phase from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Phase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Phase object, got {}",
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
                "INACTIVE" => Ok(Phase::Inactive {
                    inner: PhaseStruct::from_raw(env, obj)?,
                }),
                "ACTIVE" => Ok(Phase::Active {
                    inner: PhaseStruct::from_raw(env, obj)?,
                }),
                "COOLDOWN" => Ok(Phase::Cooldown {
                    inner: PhaseStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PhaseStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PhaseStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PhaseStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Phase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PhaseStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PhaseStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The shape of a stair block - used for constructing corners.
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

impl<'mc> StairsShape<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<StairsShape<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Stairs$Shape");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Stairs$Shape;",
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Stairs$Shape")?;
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Stairs$Shape")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::StairsShape<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Lorg/bukkit/block/data/type/Stairs$Shape;");
        let cls = jni.find_class("org/bukkit/block/data/type/Stairs$Shape");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::StairsShape::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The 'moisture' level of farmland indicates how close it is to a water source (if any).
///
/// A higher moisture level leads, to faster growth of crops on this block, but cannot be higher than <a href="#getMaximumMoisture()"><code>getMaximumMoisture()</code></a>.
///
/// This is a representation of an abstract class.
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
    pub fn moisture(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMoisture", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'moisture' property.
    pub fn set_moisture(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMoisture",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Farmland::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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
/// 'in_wall" indicates if the fence gate is attached to a wall, and if true the texture is lowered by a small amount to blend in better.
///
/// This is a representation of an abstract class.
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
    pub fn is_in_wall(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInWall", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'in_wall' property.
    pub fn set_in_wall(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setInWall",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Gate::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Gate::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn set_open(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Gate::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Openable = temp_clone.into();
        real.set_open(arg0)
    }
    pub fn is_open(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Gate::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Openable = temp_clone.into();
        real.is_open()
    }
    // SUPER CLASS: BlockData
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Gate::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.is_powered()
    }
    pub fn set_powered(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Gate::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.set_powered(arg0)
    }
    // SUPER CLASS: BlockData

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
/// The face to which a switch type block is stuck.
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

impl<'mc> SwitchFace<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<SwitchFace<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Switch$Face");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Switch$Face;",
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Switch$Face")?;
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Switch$Face")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::SwitchFace<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Lorg/bukkit/block/data/type/Switch$Face;");
        let cls = jni.find_class("org/bukkit/block/data/type/Switch$Face");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::SwitchFace::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'can_summon' indicates whether the sculk shrieker can summon the warden.
/// <p>'shrieking' indicated whether the sculk shrieker is shrieking or not.</p>
///
/// This is a representation of an abstract class.
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
    pub fn is_can_summon(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isCanSummon", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'can_summon' property.
    pub fn set_can_summon(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCanSummon",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_shrieking(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isShrieking", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'shrieking' property.
    pub fn set_shrieking(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setShrieking",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SculkShrieker::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SculkShrieker::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
pub enum Leaves<'mc> {
    None { inner: LeavesStruct<'mc> },
    Small { inner: LeavesStruct<'mc> },
    Large { inner: LeavesStruct<'mc> },
}
impl<'mc> std::fmt::Display for Leaves<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Leaves::None { .. } => f.write_str("NONE"),
            Leaves::Small { .. } => f.write_str("SMALL"),
            Leaves::Large { .. } => f.write_str("LARGE"),
        }
    }
}

impl<'mc> Leaves<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Leaves<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Leaves");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Leaves;",
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
            "NONE" => Ok(Leaves::None {
                inner: LeavesStruct::from_raw(env, obj)?,
            }),
            "SMALL" => Ok(Leaves::Small {
                inner: LeavesStruct::from_raw(env, obj)?,
            }),
            "LARGE" => Ok(Leaves::Large {
                inner: LeavesStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct LeavesStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Leaves<'mc> {
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "NONE" => Ok(Leaves::None {
                    inner: LeavesStruct::from_raw(env, obj)?,
                }),
                "SMALL" => Ok(Leaves::Small {
                    inner: LeavesStruct::from_raw(env, obj)?,
                }),
                "LARGE" => Ok(Leaves::Large {
                    inner: LeavesStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for LeavesStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LeavesStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LeavesStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Leaves")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LeavesStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LeavesStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'signal_fire' denotes whether the fire is extra smokey due to having a hay bale placed beneath it.
///
/// This is a representation of an abstract class.
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
    pub fn is_signal_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSignalFire", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'signal_fire' property.
    pub fn set_signal_fire(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSignalFire",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Campfire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Campfire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn is_lit(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Campfire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Lightable = temp_clone.into();
        real.is_lit()
    }
    pub fn set_lit(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Campfire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Lightable = temp_clone.into();
        real.set_lit(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Campfire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Campfire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
    }
    // SUPER CLASS: BlockData

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

///
/// This is a representation of an abstract class.
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
/// 'pickles' indicates the number of pickles in this block.
///
/// This is a representation of an abstract class.
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
    pub fn pickles(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPickles", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'pickles' property.
    pub fn set_pickles(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPickles",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = SeaPickle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = SeaPickle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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

///
/// This is a representation of an abstract class.
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
pub enum Height<'mc> {
    None { inner: HeightStruct<'mc> },
    Low { inner: HeightStruct<'mc> },
    Tall { inner: HeightStruct<'mc> },
}
impl<'mc> std::fmt::Display for Height<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Height::None { .. } => f.write_str("NONE"),
            Height::Low { .. } => f.write_str("LOW"),
            Height::Tall { .. } => f.write_str("TALL"),
        }
    }
}

impl<'mc> Height<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Height<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Height");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Height;",
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
            "NONE" => Ok(Height::None {
                inner: HeightStruct::from_raw(env, obj)?,
            }),
            "LOW" => Ok(Height::Low {
                inner: HeightStruct::from_raw(env, obj)?,
            }),
            "TALL" => Ok(Height::Tall {
                inner: HeightStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct HeightStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Height<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Height<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Height from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Height")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Height object, got {}",
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
                "NONE" => Ok(Height::None {
                    inner: HeightStruct::from_raw(env, obj)?,
                }),
                "LOW" => Ok(Height::Low {
                    inner: HeightStruct::from_raw(env, obj)?,
                }),
                "TALL" => Ok(Height::Tall {
                    inner: HeightStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for HeightStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HeightStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HeightStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Height")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HeightStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HeightStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Horizontal half of a bed.
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

impl<'mc> BedPart<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BedPart<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Bed$Part");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Bed$Part;",
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Bed$Part")?;
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Bed$Part")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::BedPart<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()[Lorg/bukkit/block/data/type/Bed$Part;");
        let cls = jni.find_class("org/bukkit/block/data/type/Bed$Part");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::BedPart::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
/// 'north', 'east', 'south', 'west' represent the types of connections this redstone wire has to adjacent blocks.
///
/// This is a representation of an abstract class.
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
    pub fn get_face(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<crate::block::data::mod_type::RedstoneWireConnection<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Lorg/bukkit/block/BlockFace;)Lorg/bukkit/block/data/type/RedstoneWire$Connection;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn set_face(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::data::mod_type::RedstoneWireConnection<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/data/type/RedstoneWire$Connection;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
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
    pub fn power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = RedstoneWire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::AnaloguePowerable = temp_clone.into();
        real.power()
    }
    pub fn set_power(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = RedstoneWire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::AnaloguePowerable = temp_clone.into();
        real.set_power(arg0)
    }
    pub fn maximum_power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = RedstoneWire::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::AnaloguePowerable = temp_clone.into();
        real.maximum_power()
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
/// 'has_record' is a quick flag to check whether this jukebox has a record inside it.
///
/// This is a representation of an abstract class.
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
    pub fn has_record(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasRecord", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Jukebox::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    // SUPER CLASS: Cloneable

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
pub enum Mode<'mc> {
    Compare { inner: ModeStruct<'mc> },
    Subtract { inner: ModeStruct<'mc> },
}
impl<'mc> std::fmt::Display for Mode<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Compare { .. } => f.write_str("COMPARE"),
            Mode::Subtract { .. } => f.write_str("SUBTRACT"),
        }
    }
}

impl<'mc> Mode<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Mode<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Mode");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Mode;",
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
            "COMPARE" => Ok(Mode::Compare {
                inner: ModeStruct::from_raw(env, obj)?,
            }),
            "SUBTRACT" => Ok(Mode::Subtract {
                inner: ModeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct ModeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Mode<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Mode<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Mode from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Mode")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Mode object, got {}",
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
                "COMPARE" => Ok(Mode::Compare {
                    inner: ModeStruct::from_raw(env, obj)?,
                }),
                "SUBTRACT" => Ok(Mode::Subtract {
                    inner: ModeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for ModeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ModeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ModeStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Mode")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ModeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ModeStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
/// 'conditional' denotes whether this command block is conditional or not, i.e. will only execute if the preceeding command block also executed successfully.
///
/// This is a representation of an abstract class.
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
    pub fn is_conditional(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isConditional", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'conditional' property.
    pub fn set_conditional(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setConditional",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CommandBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CommandBlock::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
/// 'hinge' indicates which hinge this door is attached to and will rotate around when opened.
///
/// This is a representation of an abstract class.
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
    pub fn hinge(
        &self,
    ) -> Result<crate::block::data::mod_type::DoorHinge<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/type/Door$Hinge;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHinge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::DoorHinge::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_hinge(
        &self,
        arg0: impl Into<crate::block::data::mod_type::DoorHinge<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/Door$Hinge;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn half(
        &self,
    ) -> Result<crate::block::data::BisectedHalf<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Door::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Bisected = temp_clone.into();
        real.half()
    }
    pub fn set_half(
        &self,
        arg0: impl Into<crate::block::data::BisectedHalf<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Door::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Bisected = temp_clone.into();
        real.set_half(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Door::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Door::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn set_open(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Door::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Openable = temp_clone.into();
        real.set_open(arg0)
    }
    pub fn is_open(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Door::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Openable = temp_clone.into();
        real.is_open()
    }
    // SUPER CLASS: BlockData
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Door::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.is_powered()
    }
    pub fn set_powered(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Door::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Powerable = temp_clone.into();
        real.set_powered(arg0)
    }
    // SUPER CLASS: BlockData

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
/// Interface to the 'slot_0_occupied', 'slow_1_occupied' ... 'slot_5_occupied' flags on a bookshelf which indicate which slots are occupied rendered on the outside.
///
/// Block may have 0, 1... <a href="#getMaximumOccupiedSlots()"><code>getMaximumOccupiedSlots()</code></a>-1 occupied slots.
///
/// This is a representation of an abstract class.
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
    pub fn is_slot_occupied(&self, arg0: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Z");
        let val_1 = jni::objects::JValueGen::Int(arg0);
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
        arg0: i32,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(IZ)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
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
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelf::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.facing()
    }
    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ChiseledBookshelf::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Directional = temp_clone.into();
        real.set_facing(arg0)
    }
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
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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

///
/// This is a representation of an abstract class.
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
/// 'candles' represents the number of candles which are present.
///
/// This is a representation of an abstract class.
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
    pub fn candles(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCandles", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'candles' property.
    pub fn set_candles(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCandles",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

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
    pub fn is_lit(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Candle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Lightable = temp_clone.into();
        real.is_lit()
    }
    pub fn set_lit(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Candle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Lightable = temp_clone.into();
        real.set_lit(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
    }
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Candle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Candle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
    }
    // SUPER CLASS: BlockData

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
/// 'thickness' represents the dripstone thickness.
///
/// 'vertical_direction' represents the dripstone orientation.
///
/// Some blocks may not be able to face in all directions, use <a href="#getVerticalDirections()"><code>getVerticalDirections()</code></a> to get all possible directions for this block.
///
/// This is a representation of an abstract class.
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

    pub fn set_vertical_direction(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn thickness(
        &self,
    ) -> Result<
        crate::block::data::mod_type::PointedDripstoneThickness<'mc>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/block/data/type/PointedDripstone$Thickness;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getThickness", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::mod_type::PointedDripstoneThickness::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_thickness(
        &self,
        arg0: impl Into<crate::block::data::mod_type::PointedDripstoneThickness<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/type/PointedDripstone$Thickness;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PointedDripstone::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PointedDripstone::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.set_waterlogged(arg0)
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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

///
/// This is a representation of an abstract class.
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
/// Different piston variants.
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

impl<'mc> TechnicalPistonType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<TechnicalPistonType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/TechnicalPiston$Type");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/TechnicalPiston$Type;",
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
            env.validate_name(&obj, "org/bukkit/block/data/type/TechnicalPiston$Type")?;
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
            env.validate_name(&obj, "org/bukkit/block/data/type/TechnicalPiston$Type")?;
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
    ) -> Result<
        Vec<crate::block::data::mod_type::TechnicalPistonType<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/block/data/type/TechnicalPiston$Type;");
        let cls = jni.find_class("org/bukkit/block/data/type/TechnicalPiston$Type");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::TechnicalPistonType::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
pub enum Tilt<'mc> {
    None { inner: TiltStruct<'mc> },
    Unstable { inner: TiltStruct<'mc> },
    Partial { inner: TiltStruct<'mc> },
    Full { inner: TiltStruct<'mc> },
}
impl<'mc> std::fmt::Display for Tilt<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tilt::None { .. } => f.write_str("NONE"),
            Tilt::Unstable { .. } => f.write_str("UNSTABLE"),
            Tilt::Partial { .. } => f.write_str("PARTIAL"),
            Tilt::Full { .. } => f.write_str("FULL"),
        }
    }
}

impl<'mc> Tilt<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Tilt<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Tilt");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Tilt;",
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
            "NONE" => Ok(Tilt::None {
                inner: TiltStruct::from_raw(env, obj)?,
            }),
            "UNSTABLE" => Ok(Tilt::Unstable {
                inner: TiltStruct::from_raw(env, obj)?,
            }),
            "PARTIAL" => Ok(Tilt::Partial {
                inner: TiltStruct::from_raw(env, obj)?,
            }),
            "FULL" => Ok(Tilt::Full {
                inner: TiltStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct TiltStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Tilt<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for Tilt<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Tilt from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Tilt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Tilt object, got {}",
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
                "NONE" => Ok(Tilt::None {
                    inner: TiltStruct::from_raw(env, obj)?,
                }),
                "UNSTABLE" => Ok(Tilt::Unstable {
                    inner: TiltStruct::from_raw(env, obj)?,
                }),
                "PARTIAL" => Ok(Tilt::Partial {
                    inner: TiltStruct::from_raw(env, obj)?,
                }),
                "FULL" => Ok(Tilt::Full {
                    inner: TiltStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for TiltStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TiltStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate TiltStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Tilt")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TiltStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TiltStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Bamboo leaf size.
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

impl<'mc> BambooLeaves<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BambooLeaves<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/type/Bamboo$Leaves");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/type/Bamboo$Leaves;",
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Bamboo$Leaves")?;
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/type/Bamboo$Leaves")?;
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
    ) -> Result<Vec<crate::block::data::mod_type::BambooLeaves<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()[Lorg/bukkit/block/data/type/Bamboo$Leaves;");
        let cls = jni.find_class("org/bukkit/block/data/type/Bamboo$Leaves");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::mod_type::BambooLeaves::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

///
/// This is a representation of an abstract class.
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
/// 'eggs' is the number of eggs which appear in this block.
///
/// This is a representation of an abstract class.
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
    pub fn eggs(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getEggs", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'eggs' property.
    pub fn set_eggs(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEggs",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn minimum_eggs(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinimumEggs", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn maximum_eggs(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumEggs", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn hatch(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = TurtleEgg::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Hatchable = temp_clone.into();
        real.hatch()
    }
    pub fn set_hatch(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = TurtleEgg::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Hatchable = temp_clone.into();
        real.set_hatch(arg0)
    }
    pub fn maximum_hatch(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = TurtleEgg::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Hatchable = temp_clone.into();
        real.maximum_hatch()
    }
    // SUPER CLASS: BlockData
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.rotate(arg0)
    }
    pub fn is_supported_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSupported", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_occluding()
    }
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
    pub fn is_preferred_tool(
        &self,
        arg0: impl Into<crate::inventory::ItemStack<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.clone()
    }
    pub fn matches(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/BlockData;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn merge(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::block::data::BlockData::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.merge(arg0)
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
