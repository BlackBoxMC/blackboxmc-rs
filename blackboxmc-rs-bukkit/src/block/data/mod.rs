#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// 'hatch' is the number of entities which may hatch from these eggs.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Hatchable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Hatchable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Hatchable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Hatchable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Hatchable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Hatchable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Hatchable<'mc> {
    pub fn hatch(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHatch", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'hatch' property.
    pub fn set_hatch(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHatch",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn maximum_hatch(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumHatch", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
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
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
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
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
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
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
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
        let temp_clone = Hatchable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Hatchable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Hatchable into crate::block::data::BlockData")
    }
}
/// 'lit' denotes whether this block (either a redstone torch or furnace) is currently lit - that is not burned out.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Lightable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Lightable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Lightable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Lightable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Lightable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Lightable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Lightable<'mc> {
    pub fn is_lit(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLit", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'lit' property.
    pub fn set_lit(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLit",
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
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
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
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
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
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
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
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
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
        let temp_clone = Lightable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Lightable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Lightable into crate::block::data::BlockData")
    }
}
/// 'face' represents the face to which a lever or button is stuck.
///
/// This is used in conjunction with <a title="interface in org.bukkit.block.data" href="Directional.html"><code>Directional</code></a> to compute the orientation of these blocks.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct FaceAttachable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FaceAttachable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FaceAttachable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FaceAttachable from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/FaceAttachable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FaceAttachable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FaceAttachable<'mc> {
    pub fn attached_face(
        &self,
    ) -> Result<crate::block::data::FaceAttachableAttachedFace<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/FaceAttachable$AttachedFace;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachedFace", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::FaceAttachableAttachedFace::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_attached_face(
        &self,
        arg0: impl Into<crate::block::data::FaceAttachableAttachedFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/FaceAttachable$AttachedFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
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
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
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
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
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
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
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
        let temp_clone = FaceAttachable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for FaceAttachable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting FaceAttachable into crate::block::data::BlockData")
    }
}
pub enum AttachedFace<'mc> {
    Floor { inner: AttachedFaceStruct<'mc> },
    Wall { inner: AttachedFaceStruct<'mc> },
    Ceiling { inner: AttachedFaceStruct<'mc> },
}
impl<'mc> std::fmt::Display for AttachedFace<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttachedFace::Floor { .. } => f.write_str("FLOOR"),
            AttachedFace::Wall { .. } => f.write_str("WALL"),
            AttachedFace::Ceiling { .. } => f.write_str("CEILING"),
        }
    }
}

impl<'mc> AttachedFace<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<AttachedFace<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/AttachedFace");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/AttachedFace;",
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
            "FLOOR" => Ok(AttachedFace::Floor {
                inner: AttachedFaceStruct::from_raw(env, obj)?,
            }),
            "WALL" => Ok(AttachedFace::Wall {
                inner: AttachedFaceStruct::from_raw(env, obj)?,
            }),
            "CEILING" => Ok(AttachedFace::Ceiling {
                inner: AttachedFaceStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct AttachedFaceStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AttachedFace<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for AttachedFace<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate AttachedFace from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/AttachedFace")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AttachedFace object, got {}",
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
                "FLOOR" => Ok(AttachedFace::Floor {
                    inner: AttachedFaceStruct::from_raw(env, obj)?,
                }),
                "WALL" => Ok(AttachedFace::Wall {
                    inner: AttachedFaceStruct::from_raw(env, obj)?,
                }),
                "CEILING" => Ok(AttachedFace::Ceiling {
                    inner: AttachedFaceStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for AttachedFaceStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AttachedFaceStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AttachedFaceStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/AttachedFace")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AttachedFaceStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AttachedFaceStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'age' represents the different growth stages that a crop-like block can go through.
///
/// A value of 0 indicates that the crop was freshly planted, whilst a value equal to <a href="#getMaximumAge()"><code>getMaximumAge()</code></a> indicates that the crop is ripe and ready to be harvested.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Ageable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Ageable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Ageable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Ageable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Ageable")?;
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

impl<'mc> Ageable<'mc> {
    pub fn age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'age' property.
    pub fn set_age(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAge",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn maximum_age(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumAge", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
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
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
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
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
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
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
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
        let temp_clone = Ageable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Ageable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Ageable into crate::block::data::BlockData")
    }
}
/// 'open' denotes whether this block is currently opened.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Openable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Openable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Openable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Openable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Openable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Openable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Openable<'mc> {
    /// Sets the value of the 'open' property.
    pub fn set_open(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOpen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_open(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOpen", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Openable::from_raw(&self.0, unsafe {
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
        let temp_clone = Openable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Openable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Openable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Openable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Openable::from_raw(&self.0, unsafe {
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
        let temp_clone = Openable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Openable::from_raw(&self.0, unsafe {
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
        let temp_clone = Openable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Openable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Openable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Openable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Openable::from_raw(&self.0, unsafe {
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
        let temp_clone = Openable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Openable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Openable into crate::block::data::BlockData")
    }
}
/// 'shape' represents the current layout of a minecart rail.
///
/// Some types of rail may not be able to be laid out in all shapes, use <a href="#getShapes()"><code>getShapes()</code></a> to get those applicable to this block.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Rail<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Rail<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Rail<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Rail from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Rail")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Rail object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Rail<'mc> {
    pub fn shape(&self) -> Result<crate::block::data::RailShape<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/Rail$Shape;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getShape", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::RailShape::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_shape(
        &self,
        arg0: impl Into<crate::block::data::RailShape<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/Rail$Shape;)V");
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
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Rail::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::Waterlogged = temp_clone.into();
        real.is_waterlogged()
    }
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Rail::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::Waterlogged<'mc>> for Rail<'mc> {
    fn into(self) -> crate::block::data::Waterlogged<'mc> {
        crate::block::data::Waterlogged::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Rail into crate::block::data::Waterlogged")
    }
}

///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct BlockData<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockData<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockData<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BlockData from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/BlockData")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockData object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockData<'mc> {
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/StructureRotation;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
        let sig = String::from("()Lorg/bukkit/Material;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaterial", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAsString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

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

    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isOccluding", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
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
        let sig = String::from("(Lorg/bukkit/inventory/ItemStack;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn is_face_sturdy(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
        arg1: impl Into<crate::block::BlockSupport<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Lorg/bukkit/block/BlockSupport;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
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

    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/structure/Mirror;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/block/data/BlockData;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
        let sig =
            String::from("(Lorg/bukkit/block/data/BlockData;)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'axis' represents the axis along whilst this block is oriented.
///
/// Some blocks such as the portal block may not be able to be placed in all orientations, use <a href="#getAxes()"><code>getAxes()</code></a> to retrieve all possible such orientations.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Orientable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Orientable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Orientable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Orientable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Orientable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Orientable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Orientable<'mc> {
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

    pub fn set_axis(
        &self,
        arg0: impl Into<crate::Axis<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Axis;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
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
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
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
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
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
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
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
        let temp_clone = Orientable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Orientable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Orientable into crate::block::data::BlockData")
    }
}
/// 'powered' indicates whether this block is in the powered state or not, i.e. receiving a redstone current of power &gt; 0.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Powerable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Powerable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Powerable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Powerable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Powerable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Powerable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Powerable<'mc> {
    pub fn is_powered(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isPowered", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'powered' property.
    pub fn set_powered(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPowered",
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
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
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
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
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
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
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
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
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
        let temp_clone = Powerable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Powerable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Powerable into crate::block::data::BlockData")
    }
}
/// The face to which a switch type block is stuck.
pub enum FaceAttachableAttachedFace<'mc> {
    Floor {
        inner: FaceAttachableAttachedFaceStruct<'mc>,
    },
    Wall {
        inner: FaceAttachableAttachedFaceStruct<'mc>,
    },
    Ceiling {
        inner: FaceAttachableAttachedFaceStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for FaceAttachableAttachedFace<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FaceAttachableAttachedFace::Floor { .. } => f.write_str("FLOOR"),
            FaceAttachableAttachedFace::Wall { .. } => f.write_str("WALL"),
            FaceAttachableAttachedFace::Ceiling { .. } => f.write_str("CEILING"),
        }
    }
}

impl<'mc> FaceAttachableAttachedFace<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<FaceAttachableAttachedFace<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/FaceAttachable$AttachedFace");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/FaceAttachable$AttachedFace;",
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
            "FLOOR" => Ok(FaceAttachableAttachedFace::Floor {
                inner: FaceAttachableAttachedFaceStruct::from_raw(env, obj)?,
            }),
            "WALL" => Ok(FaceAttachableAttachedFace::Wall {
                inner: FaceAttachableAttachedFaceStruct::from_raw(env, obj)?,
            }),
            "CEILING" => Ok(FaceAttachableAttachedFace::Ceiling {
                inner: FaceAttachableAttachedFaceStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct FaceAttachableAttachedFaceStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FaceAttachableAttachedFace<'mc> {
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
impl<'mc> JNIInstantiatable<'mc> for FaceAttachableAttachedFace<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate FaceAttachableAttachedFace from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/FaceAttachable$AttachedFace")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FaceAttachableAttachedFace object, got {}",
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
                "FLOOR" => Ok(FaceAttachableAttachedFace::Floor {
                    inner: FaceAttachableAttachedFaceStruct::from_raw(env, obj)?,
                }),
                "WALL" => Ok(FaceAttachableAttachedFace::Wall {
                    inner: FaceAttachableAttachedFaceStruct::from_raw(env, obj)?,
                }),
                "CEILING" => Ok(FaceAttachableAttachedFace::Ceiling {
                    inner: FaceAttachableAttachedFaceStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for FaceAttachableAttachedFaceStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FaceAttachableAttachedFaceStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate FaceAttachableAttachedFaceStruct from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/data/FaceAttachable$AttachedFace")?;
        if !valid {
            Err(eyre::eyre!(
                    "Invalid argument passed. Expected a FaceAttachableAttachedFaceStruct object, got {}",
                    name
                )
                .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FaceAttachableAttachedFaceStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::block::data::FaceAttachableAttachedFace<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/data/FaceAttachable$AttachedFace;");
        let cls = jni.find_class("org/bukkit/block/data/FaceAttachable$AttachedFace");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::FaceAttachableAttachedFace::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'level' represents the amount of fluid contained within this block, either by itself or inside a cauldron.
///
/// In the case of water and lava blocks the levels have special meanings: a level of 0 corresponds to a source block, 1-7 regular fluid heights, and 8-15 to "falling" fluids. All falling fluids have the same behaviour, but the level corresponds to that of the block above them, equal to <code>this.level - 8</code> <b>Note that counterintuitively, an adjusted level of 1 is the highest level, whilst 7 is the lowest.</b>
///
/// May not be higher than <a href="#getMaximumLevel()"><code>getMaximumLevel()</code></a>.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Levelled<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Levelled<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Levelled<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Levelled from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Levelled")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Levelled object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Levelled<'mc> {
    /// Sets the value of the 'level' property.
    pub fn set_level(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLevel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn maximum_level(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumLevel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
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
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
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
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
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
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
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
        let temp_clone = Levelled::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Levelled<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Levelled into crate::block::data::BlockData")
    }
}
/// This class encompasses the 'north', 'east', 'south', 'west', 'up', 'down' boolean flags which are used to set which faces of the block textures are displayed on.
///
/// Some blocks may not be able to have faces on all directions, use <a href="#getAllowedFaces()"><code>getAllowedFaces()</code></a> to get all possible faces for this block. It is not valid to call any methods on non-allowed faces.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct MultipleFacing<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MultipleFacing<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MultipleFacing<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MultipleFacing from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/MultipleFacing")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MultipleFacing object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MultipleFacing<'mc> {
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
        let sig = String::from("(Lorg/bukkit/block/BlockFace;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
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

    pub fn has_face(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
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
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
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
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
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
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
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
        let temp_clone = MultipleFacing::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for MultipleFacing<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MultipleFacing into crate::block::data::BlockData")
    }
}
/// The different types of shapes a rail block can occupy.
pub enum RailShape<'mc> {
    NorthSouth { inner: RailShapeStruct<'mc> },
    EastWest { inner: RailShapeStruct<'mc> },
    AscendingEast { inner: RailShapeStruct<'mc> },
    AscendingWest { inner: RailShapeStruct<'mc> },
    AscendingNorth { inner: RailShapeStruct<'mc> },
    AscendingSouth { inner: RailShapeStruct<'mc> },
    SouthEast { inner: RailShapeStruct<'mc> },
    SouthWest { inner: RailShapeStruct<'mc> },
    NorthWest { inner: RailShapeStruct<'mc> },
    NorthEast { inner: RailShapeStruct<'mc> },
}
impl<'mc> std::fmt::Display for RailShape<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RailShape::NorthSouth { .. } => f.write_str("NORTH_SOUTH"),
            RailShape::EastWest { .. } => f.write_str("EAST_WEST"),
            RailShape::AscendingEast { .. } => f.write_str("ASCENDING_EAST"),
            RailShape::AscendingWest { .. } => f.write_str("ASCENDING_WEST"),
            RailShape::AscendingNorth { .. } => f.write_str("ASCENDING_NORTH"),
            RailShape::AscendingSouth { .. } => f.write_str("ASCENDING_SOUTH"),
            RailShape::SouthEast { .. } => f.write_str("SOUTH_EAST"),
            RailShape::SouthWest { .. } => f.write_str("SOUTH_WEST"),
            RailShape::NorthWest { .. } => f.write_str("NORTH_WEST"),
            RailShape::NorthEast { .. } => f.write_str("NORTH_EAST"),
        }
    }
}

impl<'mc> RailShape<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<RailShape<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/Rail$Shape");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/Rail$Shape;",
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
            "NORTH_SOUTH" => Ok(RailShape::NorthSouth {
                inner: RailShapeStruct::from_raw(env, obj)?,
            }),
            "EAST_WEST" => Ok(RailShape::EastWest {
                inner: RailShapeStruct::from_raw(env, obj)?,
            }),
            "ASCENDING_EAST" => Ok(RailShape::AscendingEast {
                inner: RailShapeStruct::from_raw(env, obj)?,
            }),
            "ASCENDING_WEST" => Ok(RailShape::AscendingWest {
                inner: RailShapeStruct::from_raw(env, obj)?,
            }),
            "ASCENDING_NORTH" => Ok(RailShape::AscendingNorth {
                inner: RailShapeStruct::from_raw(env, obj)?,
            }),
            "ASCENDING_SOUTH" => Ok(RailShape::AscendingSouth {
                inner: RailShapeStruct::from_raw(env, obj)?,
            }),
            "SOUTH_EAST" => Ok(RailShape::SouthEast {
                inner: RailShapeStruct::from_raw(env, obj)?,
            }),
            "SOUTH_WEST" => Ok(RailShape::SouthWest {
                inner: RailShapeStruct::from_raw(env, obj)?,
            }),
            "NORTH_WEST" => Ok(RailShape::NorthWest {
                inner: RailShapeStruct::from_raw(env, obj)?,
            }),
            "NORTH_EAST" => Ok(RailShape::NorthEast {
                inner: RailShapeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct RailShapeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for RailShape<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::NorthSouth { inner } => inner.0.clone(),
            Self::EastWest { inner } => inner.0.clone(),
            Self::AscendingEast { inner } => inner.0.clone(),
            Self::AscendingWest { inner } => inner.0.clone(),
            Self::AscendingNorth { inner } => inner.0.clone(),
            Self::AscendingSouth { inner } => inner.0.clone(),
            Self::SouthEast { inner } => inner.0.clone(),
            Self::SouthWest { inner } => inner.0.clone(),
            Self::NorthWest { inner } => inner.0.clone(),
            Self::NorthEast { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::NorthSouth { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EastWest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::AscendingEast { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AscendingWest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AscendingNorth { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AscendingSouth { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SouthEast { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SouthWest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NorthWest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NorthEast { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RailShape<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate RailShape from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Rail$Shape")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RailShape object, got {}",
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
                "NORTH_SOUTH" => Ok(RailShape::NorthSouth {
                    inner: RailShapeStruct::from_raw(env, obj)?,
                }),
                "EAST_WEST" => Ok(RailShape::EastWest {
                    inner: RailShapeStruct::from_raw(env, obj)?,
                }),
                "ASCENDING_EAST" => Ok(RailShape::AscendingEast {
                    inner: RailShapeStruct::from_raw(env, obj)?,
                }),
                "ASCENDING_WEST" => Ok(RailShape::AscendingWest {
                    inner: RailShapeStruct::from_raw(env, obj)?,
                }),
                "ASCENDING_NORTH" => Ok(RailShape::AscendingNorth {
                    inner: RailShapeStruct::from_raw(env, obj)?,
                }),
                "ASCENDING_SOUTH" => Ok(RailShape::AscendingSouth {
                    inner: RailShapeStruct::from_raw(env, obj)?,
                }),
                "SOUTH_EAST" => Ok(RailShape::SouthEast {
                    inner: RailShapeStruct::from_raw(env, obj)?,
                }),
                "SOUTH_WEST" => Ok(RailShape::SouthWest {
                    inner: RailShapeStruct::from_raw(env, obj)?,
                }),
                "NORTH_WEST" => Ok(RailShape::NorthWest {
                    inner: RailShapeStruct::from_raw(env, obj)?,
                }),
                "NORTH_EAST" => Ok(RailShape::NorthEast {
                    inner: RailShapeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for RailShapeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for RailShapeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate RailShapeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Rail$Shape")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RailShapeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> RailShapeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::block::data::RailShape<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/Rail$Shape;");
        let cls = jni.find_class("org/bukkit/block/data/Rail$Shape");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::RailShape::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'snowy' denotes whether this block has a snow covered side and top texture (normally because the block above is snow).
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Snowable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Snowable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Snowable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Snowable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Snowable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Snowable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Snowable<'mc> {
    pub fn is_snowy(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isSnowy", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'snowy' property.
    pub fn set_snowy(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSnowy",
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
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
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
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
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
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
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
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
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
        let temp_clone = Snowable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Snowable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Snowable into crate::block::data::BlockData")
    }
}
/// 'facing' represents the face towards which the block is pointing.
///
/// Some blocks may not be able to face in all directions, use <a href="#getFaces()"><code>getFaces()</code></a> to get all possible directions for this block.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Directional<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Directional<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Directional<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Directional from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Directional")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Directional object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Directional<'mc> {
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

    pub fn set_facing(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Directional::from_raw(&self.0, unsafe {
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
        let temp_clone = Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Directional::from_raw(&self.0, unsafe {
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
        let temp_clone = Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Directional::from_raw(&self.0, unsafe {
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
        let temp_clone = Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Directional::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Directional::from_raw(&self.0, unsafe {
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
        let temp_clone = Directional::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Directional<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Directional into crate::block::data::BlockData")
    }
}
pub enum Half<'mc> {
    Top { inner: HalfStruct<'mc> },
    Bottom { inner: HalfStruct<'mc> },
}
impl<'mc> std::fmt::Display for Half<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Half::Top { .. } => f.write_str("TOP"),
            Half::Bottom { .. } => f.write_str("BOTTOM"),
        }
    }
}

impl<'mc> Half<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Half<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/Half");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/Half;",
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
            "TOP" => Ok(Half::Top {
                inner: HalfStruct::from_raw(env, obj)?,
            }),
            "BOTTOM" => Ok(Half::Bottom {
                inner: HalfStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct HalfStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Half<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Top { inner } => inner.0.clone(),
            Self::Bottom { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Top { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Bottom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Half<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Half from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Half")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Half object, got {}",
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
                "TOP" => Ok(Half::Top {
                    inner: HalfStruct::from_raw(env, obj)?,
                }),
                "BOTTOM" => Ok(Half::Bottom {
                    inner: HalfStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for HalfStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for HalfStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate HalfStruct from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Half")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HalfStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> HalfStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// 'rotation' represents the current rotation of this block.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Rotatable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Rotatable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Rotatable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Rotatable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Rotatable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Rotatable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Rotatable<'mc> {
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

    pub fn set_rotation(
        &self,
        arg0: impl Into<crate::block::BlockFace<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/BlockFace;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
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
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
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
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
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
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
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
        let temp_clone = Rotatable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Rotatable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Rotatable into crate::block::data::BlockData")
    }
}
/// 'hanging' denotes whether the lantern is hanging from a block.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Hangable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Hangable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Hangable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Hangable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Hangable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Hangable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Hangable<'mc> {
    pub fn is_hanging(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isHanging", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'hanging' property.
    pub fn set_hanging(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHanging",
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
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
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
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
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
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
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
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
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
        let temp_clone = Hangable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Hangable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Hangable into crate::block::data::BlockData")
    }
}
/// 'waterlogged' denotes whether this block has fluid in it.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Waterlogged<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Waterlogged<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Waterlogged<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Waterlogged from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Waterlogged")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Waterlogged object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Waterlogged<'mc> {
    pub fn is_waterlogged(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isWaterlogged", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'waterlogged' property.
    pub fn set_waterlogged(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWaterlogged",
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
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
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
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
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
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
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
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
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
        let temp_clone = Waterlogged::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Waterlogged<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Waterlogged into crate::block::data::BlockData")
    }
}
/// 'power' represents the redstone power level currently being emitted or transmitted via this block.
///
/// May not be over 9000 or <a href="#getMaximumPower()"><code>getMaximumPower()</code></a> (usually 15).
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct AnaloguePowerable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for AnaloguePowerable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for AnaloguePowerable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate AnaloguePowerable from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/AnaloguePowerable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a AnaloguePowerable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> AnaloguePowerable<'mc> {
    pub fn power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'power' property.
    pub fn set_power(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPower",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn maximum_power(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaximumPower", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
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
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
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
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
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
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
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
        let temp_clone = AnaloguePowerable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for AnaloguePowerable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting AnaloguePowerable into crate::block::data::BlockData")
    }
}
/// 'dusted' represents how far uncovered by brush the block is.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Brushable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Brushable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Brushable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Brushable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Brushable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Brushable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Brushable<'mc> {
    pub fn dusted(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDusted", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the value of the 'dusted' property.
    pub fn set_dusted(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDusted",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn maximum_dusted(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaximumDusted",
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
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
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
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
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
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
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
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
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
        let temp_clone = Brushable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Brushable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Brushable into crate::block::data::BlockData")
    }
}
/// 'attached' denotes whether a tripwire hook or string forms a complete tripwire circuit and is ready to trigger.
///
/// Updating the property on a tripwire hook will change the texture to indicate a connected string, but will not have any effect when used on the tripwire string itself. It may however still be used to check whether the string forms a circuit.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Attachable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Attachable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Attachable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Attachable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Attachable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Attachable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Attachable<'mc> {
    pub fn is_attached(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isAttached", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the value of the 'attached' property.
    pub fn set_attached(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAttached",
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
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
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
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
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
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
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
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
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
        let temp_clone = Attachable::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Attachable<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Attachable into crate::block::data::BlockData")
    }
}
/// 'half' denotes which half of a two block tall material this block is.
///
/// In game it may be referred to as either (top, bottom) or (upper, lower).
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Bisected<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Bisected<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Bisected<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Bisected from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Bisected")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Bisected object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Bisected<'mc> {
    pub fn half(
        &self,
    ) -> Result<crate::block::data::BisectedHalf<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/Bisected$Half;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHalf", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BisectedHalf::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_half(
        &self,
        arg0: impl Into<crate::block::data::BisectedHalf<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/data/Bisected$Half;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
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
    pub fn rotate(
        &self,
        arg0: impl Into<crate::block::structure::StructureRotation<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
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
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.material()
    }
    pub fn as_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.as_string()
    }
    pub fn sound_group(&self) -> Result<crate::SoundGroup<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.sound_group()
    }
    pub fn light_emission(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.light_emission()
    }
    pub fn is_occluding(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
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
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_preferred_tool(arg0)
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
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
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.is_face_sturdy(arg0, arg1)
    }
    pub fn placement_material(&self) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.placement_material()
    }
    pub fn mirror(
        &self,
        arg0: impl Into<crate::block::structure::Mirror<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.mirror(arg0)
    }
    pub fn create_block_state(
        &self,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::block::data::BlockData = temp_clone.into();
        real.create_block_state()
    }
    pub fn clone(&self) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
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
        let temp_clone = Bisected::from_raw(&self.0, unsafe {
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
impl<'mc> Into<crate::block::data::BlockData<'mc>> for Bisected<'mc> {
    fn into(self) -> crate::block::data::BlockData<'mc> {
        crate::block::data::BlockData::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Bisected into crate::block::data::BlockData")
    }
}
/// The half of a vertically bisected block.
pub enum BisectedHalf<'mc> {
    Top { inner: BisectedHalfStruct<'mc> },
    Bottom { inner: BisectedHalfStruct<'mc> },
}
impl<'mc> std::fmt::Display for BisectedHalf<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BisectedHalf::Top { .. } => f.write_str("TOP"),
            BisectedHalf::Bottom { .. } => f.write_str("BOTTOM"),
        }
    }
}

impl<'mc> BisectedHalf<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<BisectedHalf<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/data/Bisected$Half");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/data/Bisected$Half;",
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
            "TOP" => Ok(BisectedHalf::Top {
                inner: BisectedHalfStruct::from_raw(env, obj)?,
            }),
            "BOTTOM" => Ok(BisectedHalf::Bottom {
                inner: BisectedHalfStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct BisectedHalfStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BisectedHalf<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Top { inner } => inner.0.clone(),
            Self::Bottom { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Top { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Bottom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BisectedHalf<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BisectedHalf from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Bisected$Half")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BisectedHalf object, got {}",
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
                "TOP" => Ok(BisectedHalf::Top {
                    inner: BisectedHalfStruct::from_raw(env, obj)?,
                }),
                "BOTTOM" => Ok(BisectedHalf::Bottom {
                    inner: BisectedHalfStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for BisectedHalfStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BisectedHalfStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BisectedHalfStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/data/Bisected$Half")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BisectedHalfStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BisectedHalfStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<Vec<crate::block::data::BisectedHalf<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/data/Bisected$Half;");
        let cls = jni.find_class("org/bukkit/block/data/Bisected$Half");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = jni.get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = jni.get_object_array_element(&arr, i)?;
            vec.push({ crate::block::data::BisectedHalf::from_raw(&jni, res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub mod mod_type;
