#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents a Minecart with TNT inside it that can explode when triggered.
///
/// This is a representation of an abstract class.
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
    /// Set the fuse ticks of this minecart. If the fuse ticks are set to a non-zero value, this will ignite the explosive.
    pub fn set_fuse_ticks(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuseTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn fuse_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFuseTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn ignite(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ignite", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_ignited(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isIgnited", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Immediately explode this minecart with the given power.
    pub fn explode_with_double(
        &self,
        arg0: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
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
    pub fn set_damage(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_damage(arg0)
    }
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.damage()
    }
    pub fn max_speed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.max_speed()
    }
    pub fn set_max_speed(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_max_speed(arg0)
    }
    pub fn is_slow_when_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.is_slow_when_empty()
    }
    pub fn set_slow_when_empty(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_slow_when_empty(arg0)
    }
    pub fn flying_velocity_mod(
        &self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.flying_velocity_mod()
    }
    pub fn set_flying_velocity_mod(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_flying_velocity_mod(arg0)
    }
    pub fn derailed_velocity_mod(
        &self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.derailed_velocity_mod()
    }
    pub fn set_derailed_velocity_mod(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_derailed_velocity_mod(arg0)
    }
    pub fn set_display_block(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block(arg0)
    }
    pub fn display_block(
        &self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block()
    }
    pub fn set_display_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block_data(arg0)
    }
    pub fn display_block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block_data()
    }
    pub fn set_display_block_offset(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block_offset(arg0)
    }
    pub fn display_block_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = ExplosiveMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block_offset()
    }
    // SUPER CLASS: Vehicle
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.facing()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.world()
    }
    pub fn is_silent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_silent()
    }
    pub fn portal_cooldown(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.portal_cooldown()
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_valid()
    }
    pub fn get_nearby_entities(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let val_2 = jni::objects::JValueGen::Double(arg1);
        let val_3 = jni::objects::JValueGen::Double(arg2);
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
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn set_rotation(&self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_rotation(arg0, arg1)
    }
    pub fn set_velocity(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_velocity(arg0)
    }
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.velocity()
    }
    pub fn height(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.height()
    }
    pub fn width(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.width()
    }
    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.bounding_box()
    }
    pub fn is_on_ground(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_on_ground()
    }
    pub fn is_in_water(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_in_water()
    }
    pub fn teleport_with_entity(
        &self,
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
    pub fn teleport_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
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
    pub fn entity_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.entity_id()
    }
    pub fn fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn max_fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn set_fire_ticks(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_visual_fire(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_visual_fire(arg0)
    }
    pub fn is_visual_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_visual_fire()
    }
    pub fn freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn max_freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
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
    pub fn set_freeze_ticks(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_dead(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_dead()
    }
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.server()
    }
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_persistent()
    }
    pub fn set_persistent(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_persistent(arg0)
    }

    pub fn passenger(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.passenger()
    }
    pub fn set_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_passenger(arg0)
    }
    pub fn passengers(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), vec![]);
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
    pub fn add_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.add_passenger(arg0)
    }
    pub fn remove_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove_passenger(arg0)
    }
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.eject()
    }
    pub fn fall_distance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.fall_distance()
    }
    pub fn set_fall_distance(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_fall_distance(arg0)
    }
    pub fn set_last_damage_cause(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_last_damage_cause(arg0)
    }
    pub fn last_damage_cause(
        &self,
    ) -> Result<Option<crate::event::entity::EntityDamageEvent<'mc>>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.last_damage_cause()
    }
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.unique_id()
    }
    pub fn ticks_lived(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.ticks_lived()
    }
    pub fn set_ticks_lived(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_ticks_lived(arg0)
    }
    pub fn play_effect(
        &self,
        arg0: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.play_effect(arg0)
    }
    pub fn swim_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_sound()
    }
    pub fn swim_splash_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_splash_sound()
    }
    pub fn swim_high_speed_splash_sound(
        &self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_high_speed_splash_sound()
    }
    pub fn is_inside_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_inside_vehicle()
    }
    pub fn leave_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.leave_vehicle()
    }
    pub fn vehicle(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.vehicle()
    }
    pub fn set_custom_name_visible(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_custom_name_visible(arg0)
    }
    pub fn is_custom_name_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_custom_name_visible()
    }
    pub fn set_visible_by_default(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_visible_by_default(arg0)
    }
    pub fn is_visible_by_default(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_visible_by_default()
    }
    pub fn set_glowing(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_glowing(arg0)
    }
    pub fn is_glowing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_glowing()
    }
    pub fn set_invulnerable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_invulnerable(arg0)
    }
    pub fn is_invulnerable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_invulnerable()
    }
    pub fn set_silent(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_silent(arg0)
    }
    pub fn has_gravity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.has_gravity()
    }
    pub fn set_gravity(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_gravity(arg0)
    }
    pub fn set_portal_cooldown(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_portal_cooldown(arg0)
    }
    pub fn scoreboard_tags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboardTags",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_scoreboard_tag(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.add_scoreboard_tag(arg0)
    }
    pub fn remove_scoreboard_tag(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove_scoreboard_tag(arg0)
    }
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.pose()
    }
    pub fn spawn_category(
        &self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.spawn_category()
    }
    pub fn spigot(
        &self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.spigot()
    }
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove()
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_empty()
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.get_type()
    }
    pub fn is_frozen(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_frozen()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    // SUPER CLASS: CommandSender
    pub fn is_permission_set_with_permission(
        &self,
        arg0: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_permission_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn add_attachment_with_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
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
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
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
    pub fn remove_attachment(
        &self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::command::CommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::Permissible = temp_clone.into();
        real.remove_attachment(arg0)
    }
    pub fn recalculate_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn effective_permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: ServerOperator
    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::ServerOperator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.is_op()
    }
    pub fn set_op(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::ServerOperator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.set_op(arg0)
    }
    pub fn send_message_with_strings(
        &self,
        arg0: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Ljava/lang/String;";
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg0.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
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
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::CommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::CommandSender = temp_clone.into();
        real.name()
    }
    // SUPER CLASS: Nameable
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
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
/// Represents a Minecart with a Hopper inside it
///
/// This is a representation of an abstract class.
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
    pub fn is_enabled(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets whether this Minecart will pick up items.
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
    pub fn set_damage(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_damage(arg0)
    }
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.damage()
    }
    pub fn max_speed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.max_speed()
    }
    pub fn set_max_speed(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_max_speed(arg0)
    }
    pub fn is_slow_when_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.is_slow_when_empty()
    }
    pub fn set_slow_when_empty(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_slow_when_empty(arg0)
    }
    pub fn flying_velocity_mod(
        &self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.flying_velocity_mod()
    }
    pub fn set_flying_velocity_mod(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_flying_velocity_mod(arg0)
    }
    pub fn derailed_velocity_mod(
        &self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.derailed_velocity_mod()
    }
    pub fn set_derailed_velocity_mod(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_derailed_velocity_mod(arg0)
    }
    pub fn set_display_block(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block(arg0)
    }
    pub fn display_block(
        &self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block()
    }
    pub fn set_display_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block_data(arg0)
    }
    pub fn display_block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block_data()
    }
    pub fn set_display_block_offset(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block_offset(arg0)
    }
    pub fn display_block_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block_offset()
    }
    // SUPER CLASS: Vehicle
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.facing()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.world()
    }
    pub fn is_silent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_silent()
    }
    pub fn portal_cooldown(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.portal_cooldown()
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_valid()
    }
    pub fn get_nearby_entities(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let val_2 = jni::objects::JValueGen::Double(arg1);
        let val_3 = jni::objects::JValueGen::Double(arg2);
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
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn set_rotation(&self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_rotation(arg0, arg1)
    }
    pub fn set_velocity(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_velocity(arg0)
    }
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.velocity()
    }
    pub fn height(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.height()
    }
    pub fn width(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.width()
    }
    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.bounding_box()
    }
    pub fn is_on_ground(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_on_ground()
    }
    pub fn is_in_water(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_in_water()
    }
    pub fn teleport_with_entity(
        &self,
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
    pub fn teleport_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
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
    pub fn entity_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.entity_id()
    }
    pub fn fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn max_fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn set_fire_ticks(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_visual_fire(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_visual_fire(arg0)
    }
    pub fn is_visual_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_visual_fire()
    }
    pub fn freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn max_freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
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
    pub fn set_freeze_ticks(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_dead(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_dead()
    }
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.server()
    }
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_persistent()
    }
    pub fn set_persistent(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_persistent(arg0)
    }

    pub fn passenger(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.passenger()
    }
    pub fn set_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_passenger(arg0)
    }
    pub fn passengers(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), vec![]);
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
    pub fn add_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.add_passenger(arg0)
    }
    pub fn remove_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove_passenger(arg0)
    }
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.eject()
    }
    pub fn fall_distance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.fall_distance()
    }
    pub fn set_fall_distance(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_fall_distance(arg0)
    }
    pub fn set_last_damage_cause(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_last_damage_cause(arg0)
    }
    pub fn last_damage_cause(
        &self,
    ) -> Result<Option<crate::event::entity::EntityDamageEvent<'mc>>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.last_damage_cause()
    }
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.unique_id()
    }
    pub fn ticks_lived(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.ticks_lived()
    }
    pub fn set_ticks_lived(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_ticks_lived(arg0)
    }
    pub fn play_effect(
        &self,
        arg0: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.play_effect(arg0)
    }
    pub fn swim_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_sound()
    }
    pub fn swim_splash_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_splash_sound()
    }
    pub fn swim_high_speed_splash_sound(
        &self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_high_speed_splash_sound()
    }
    pub fn is_inside_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_inside_vehicle()
    }
    pub fn leave_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.leave_vehicle()
    }
    pub fn vehicle(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.vehicle()
    }
    pub fn set_custom_name_visible(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_custom_name_visible(arg0)
    }
    pub fn is_custom_name_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_custom_name_visible()
    }
    pub fn set_visible_by_default(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_visible_by_default(arg0)
    }
    pub fn is_visible_by_default(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_visible_by_default()
    }
    pub fn set_glowing(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_glowing(arg0)
    }
    pub fn is_glowing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_glowing()
    }
    pub fn set_invulnerable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_invulnerable(arg0)
    }
    pub fn is_invulnerable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_invulnerable()
    }
    pub fn set_silent(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_silent(arg0)
    }
    pub fn has_gravity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.has_gravity()
    }
    pub fn set_gravity(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_gravity(arg0)
    }
    pub fn set_portal_cooldown(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_portal_cooldown(arg0)
    }
    pub fn scoreboard_tags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboardTags",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_scoreboard_tag(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.add_scoreboard_tag(arg0)
    }
    pub fn remove_scoreboard_tag(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove_scoreboard_tag(arg0)
    }
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.pose()
    }
    pub fn spawn_category(
        &self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.spawn_category()
    }
    pub fn spigot(
        &self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.spigot()
    }
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove()
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_empty()
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.get_type()
    }
    pub fn is_frozen(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_frozen()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    // SUPER CLASS: CommandSender
    pub fn is_permission_set_with_permission(
        &self,
        arg0: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_permission_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn add_attachment_with_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
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
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
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
    pub fn remove_attachment(
        &self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::command::CommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::Permissible = temp_clone.into();
        real.remove_attachment(arg0)
    }
    pub fn recalculate_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn effective_permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: ServerOperator
    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::ServerOperator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.is_op()
    }
    pub fn set_op(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::ServerOperator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.set_op(arg0)
    }
    pub fn send_message_with_strings(
        &self,
        arg0: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Ljava/lang/String;";
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg0.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
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
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::CommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::CommandSender = temp_clone.into();
        real.name()
    }
    // SUPER CLASS: Nameable
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
    }
    pub fn inventory(
        &self,
    ) -> Result<crate::inventory::Inventory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::inventory::InventoryHolder = temp_clone.into();
        real.inventory()
    }
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.seed()
    }
    pub fn set_seed(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_seed(arg0)
    }
    pub fn set_loot_table(
        &self,
        arg0: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.set_loot_table(arg0)
    }
    pub fn loot_table(
        &self,
    ) -> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = HopperMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::loot::Lootable = temp_clone.into();
        real.loot_table()
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
/// Represents a minecart with a chest. These types of <a title="interface in org.bukkit.entity" href="../Minecart.html"><code>minecarts</code></a> have their own inventory that can be accessed using methods from the <a href="../../inventory/InventoryHolder.html" title="interface in org.bukkit.inventory"><code>InventoryHolder</code></a> interface.
///
/// This is a representation of an abstract class.
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
/// Represents a Minecart with an <a title="interface in org.bukkit.block" href="../../block/CreatureSpawner.html"><code>entity spawner</code></a> inside it.
///
/// This is a representation of an abstract class.
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
/// Represents a powered minecart. A powered minecart moves on its own when a player deposits <a href="../../Material.html#COAL"><code>fuel</code></a>.
///
/// This is a representation of an abstract class.
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
    /// Set the number of ticks until the minecart runs out of fuel.
    pub fn set_fuel(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFuel",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn fuel(&self) -> Result<Option<i32>, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFuel", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(Some(res.i()?))
    }
    pub fn set_damage(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_damage(arg0)
    }
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.damage()
    }
    pub fn max_speed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.max_speed()
    }
    pub fn set_max_speed(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_max_speed(arg0)
    }
    pub fn is_slow_when_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.is_slow_when_empty()
    }
    pub fn set_slow_when_empty(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_slow_when_empty(arg0)
    }
    pub fn flying_velocity_mod(
        &self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.flying_velocity_mod()
    }
    pub fn set_flying_velocity_mod(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_flying_velocity_mod(arg0)
    }
    pub fn derailed_velocity_mod(
        &self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.derailed_velocity_mod()
    }
    pub fn set_derailed_velocity_mod(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_derailed_velocity_mod(arg0)
    }
    pub fn set_display_block(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block(arg0)
    }
    pub fn display_block(
        &self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block()
    }
    pub fn set_display_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block_data(arg0)
    }
    pub fn display_block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block_data()
    }
    pub fn set_display_block_offset(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block_offset(arg0)
    }
    pub fn display_block_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = PoweredMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block_offset()
    }
    // SUPER CLASS: Vehicle
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.facing()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.world()
    }
    pub fn is_silent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_silent()
    }
    pub fn portal_cooldown(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.portal_cooldown()
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_valid()
    }
    pub fn get_nearby_entities(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let val_2 = jni::objects::JValueGen::Double(arg1);
        let val_3 = jni::objects::JValueGen::Double(arg2);
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
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn set_rotation(&self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_rotation(arg0, arg1)
    }
    pub fn set_velocity(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_velocity(arg0)
    }
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.velocity()
    }
    pub fn height(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.height()
    }
    pub fn width(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.width()
    }
    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.bounding_box()
    }
    pub fn is_on_ground(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_on_ground()
    }
    pub fn is_in_water(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_in_water()
    }
    pub fn teleport_with_entity(
        &self,
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
    pub fn teleport_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
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
    pub fn entity_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.entity_id()
    }
    pub fn fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn max_fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn set_fire_ticks(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_visual_fire(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_visual_fire(arg0)
    }
    pub fn is_visual_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_visual_fire()
    }
    pub fn freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn max_freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
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
    pub fn set_freeze_ticks(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_dead(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_dead()
    }
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.server()
    }
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_persistent()
    }
    pub fn set_persistent(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_persistent(arg0)
    }

    pub fn passenger(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.passenger()
    }
    pub fn set_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_passenger(arg0)
    }
    pub fn passengers(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), vec![]);
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
    pub fn add_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.add_passenger(arg0)
    }
    pub fn remove_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove_passenger(arg0)
    }
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.eject()
    }
    pub fn fall_distance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.fall_distance()
    }
    pub fn set_fall_distance(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_fall_distance(arg0)
    }
    pub fn set_last_damage_cause(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_last_damage_cause(arg0)
    }
    pub fn last_damage_cause(
        &self,
    ) -> Result<Option<crate::event::entity::EntityDamageEvent<'mc>>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.last_damage_cause()
    }
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.unique_id()
    }
    pub fn ticks_lived(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.ticks_lived()
    }
    pub fn set_ticks_lived(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_ticks_lived(arg0)
    }
    pub fn play_effect(
        &self,
        arg0: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.play_effect(arg0)
    }
    pub fn swim_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_sound()
    }
    pub fn swim_splash_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_splash_sound()
    }
    pub fn swim_high_speed_splash_sound(
        &self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_high_speed_splash_sound()
    }
    pub fn is_inside_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_inside_vehicle()
    }
    pub fn leave_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.leave_vehicle()
    }
    pub fn vehicle(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.vehicle()
    }
    pub fn set_custom_name_visible(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_custom_name_visible(arg0)
    }
    pub fn is_custom_name_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_custom_name_visible()
    }
    pub fn set_visible_by_default(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_visible_by_default(arg0)
    }
    pub fn is_visible_by_default(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_visible_by_default()
    }
    pub fn set_glowing(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_glowing(arg0)
    }
    pub fn is_glowing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_glowing()
    }
    pub fn set_invulnerable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_invulnerable(arg0)
    }
    pub fn is_invulnerable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_invulnerable()
    }
    pub fn set_silent(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_silent(arg0)
    }
    pub fn has_gravity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.has_gravity()
    }
    pub fn set_gravity(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_gravity(arg0)
    }
    pub fn set_portal_cooldown(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_portal_cooldown(arg0)
    }
    pub fn scoreboard_tags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboardTags",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_scoreboard_tag(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.add_scoreboard_tag(arg0)
    }
    pub fn remove_scoreboard_tag(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove_scoreboard_tag(arg0)
    }
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.pose()
    }
    pub fn spawn_category(
        &self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.spawn_category()
    }
    pub fn spigot(
        &self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.spigot()
    }
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove()
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_empty()
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.get_type()
    }
    pub fn is_frozen(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_frozen()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    // SUPER CLASS: CommandSender
    pub fn is_permission_set_with_permission(
        &self,
        arg0: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_permission_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn add_attachment_with_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
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
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
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
    pub fn remove_attachment(
        &self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::command::CommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::Permissible = temp_clone.into();
        real.remove_attachment(arg0)
    }
    pub fn recalculate_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn effective_permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: ServerOperator
    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::ServerOperator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.is_op()
    }
    pub fn set_op(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::ServerOperator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.set_op(arg0)
    }
    pub fn send_message_with_strings(
        &self,
        arg0: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Ljava/lang/String;";
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg0.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
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
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::CommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::CommandSender = temp_clone.into();
        real.name()
    }
    // SUPER CLASS: Nameable
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
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
/// Represents a minecart that can have certain <a title="interface in org.bukkit.entity" href="../Entity.html"><code>entities</code></a> as passengers. Normal passengers include all <a title="interface in org.bukkit.entity" href="../LivingEntity.html"><code>living entities</code></a> with the exception of <a href="../IronGolem.html" title="interface in org.bukkit.entity"><code>iron golems</code></a>. Non-player entities that meet normal passenger criteria automatically mount these minecarts when close enough.
///
/// This is a representation of an abstract class.
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

///
/// This is a representation of an abstract class.
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

    pub fn set_command(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
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

    pub fn set_name(&self, arg0: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn set_damage(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_damage(arg0)
    }
    pub fn damage(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.damage()
    }
    pub fn max_speed(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.max_speed()
    }
    pub fn set_max_speed(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_max_speed(arg0)
    }
    pub fn is_slow_when_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.is_slow_when_empty()
    }
    pub fn set_slow_when_empty(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_slow_when_empty(arg0)
    }
    pub fn flying_velocity_mod(
        &self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.flying_velocity_mod()
    }
    pub fn set_flying_velocity_mod(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_flying_velocity_mod(arg0)
    }
    pub fn derailed_velocity_mod(
        &self,
    ) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.derailed_velocity_mod()
    }
    pub fn set_derailed_velocity_mod(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_derailed_velocity_mod(arg0)
    }
    pub fn set_display_block(
        &self,
        arg0: impl Into<crate::material::MaterialData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block(arg0)
    }
    pub fn display_block(
        &self,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block()
    }
    pub fn set_display_block_data(
        &self,
        arg0: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block_data(arg0)
    }
    pub fn display_block_data(
        &self,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block_data()
    }
    pub fn set_display_block_offset(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.set_display_block_offset(arg0)
    }
    pub fn display_block_offset(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = CommandMinecart::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Minecart = temp_clone.into();
        real.display_block_offset()
    }
    // SUPER CLASS: Vehicle
    pub fn facing(&self) -> Result<crate::block::BlockFace<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.facing()
    }
    pub fn world(&self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.world()
    }
    pub fn is_silent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_silent()
    }
    pub fn portal_cooldown(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.portal_cooldown()
    }
    pub fn piston_move_reaction(
        &self,
    ) -> Result<crate::block::PistonMoveReaction<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.piston_move_reaction()
    }
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_valid()
    }
    pub fn get_nearby_entities(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let val_2 = jni::objects::JValueGen::Double(arg1);
        let val_3 = jni::objects::JValueGen::Double(arg2);
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
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    pub fn set_rotation(&self, arg0: f32, arg1: f32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_rotation(arg0, arg1)
    }
    pub fn set_velocity(
        &self,
        arg0: impl Into<crate::util::Vector<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_velocity(arg0)
    }
    pub fn velocity(&self) -> Result<crate::util::Vector<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.velocity()
    }
    pub fn height(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.height()
    }
    pub fn width(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.width()
    }
    pub fn bounding_box(
        &self,
    ) -> Result<crate::util::BoundingBox<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.bounding_box()
    }
    pub fn is_on_ground(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_on_ground()
    }
    pub fn is_in_water(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_in_water()
    }
    pub fn teleport_with_entity(
        &self,
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
    pub fn teleport_with_location(
        &self,
        arg0: impl Into<crate::Location<'mc>>,
        arg1: std::option::Option<
            impl Into<crate::event::player::PlayerTeleportEventTeleportCause<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
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
    pub fn entity_id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.entity_id()
    }
    pub fn fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn max_fire_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxFireTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn set_fire_ticks(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFireTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_visual_fire(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_visual_fire(arg0)
    }
    pub fn is_visual_fire(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_visual_fire()
    }
    pub fn freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getFreezeTicks", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn max_freeze_ticks(&self) -> Result<i32, Box<dyn std::error::Error>> {
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
    pub fn set_freeze_ticks(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFreezeTicks",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_dead(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_dead()
    }
    pub fn server(&self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.server()
    }
    pub fn is_persistent(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_persistent()
    }
    pub fn set_persistent(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_persistent(arg0)
    }

    pub fn passenger(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.passenger()
    }
    pub fn set_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_passenger(arg0)
    }
    pub fn passengers(
        &self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPassengers", sig.as_str(), vec![]);
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
    pub fn add_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.add_passenger(arg0)
    }
    pub fn remove_passenger(
        &self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove_passenger(arg0)
    }
    pub fn eject(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.eject()
    }
    pub fn fall_distance(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.fall_distance()
    }
    pub fn set_fall_distance(&self, arg0: f32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_fall_distance(arg0)
    }
    pub fn set_last_damage_cause(
        &self,
        arg0: impl Into<crate::event::entity::EntityDamageEvent<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_last_damage_cause(arg0)
    }
    pub fn last_damage_cause(
        &self,
    ) -> Result<Option<crate::event::entity::EntityDamageEvent<'mc>>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.last_damage_cause()
    }
    pub fn unique_id(
        &self,
    ) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.unique_id()
    }
    pub fn ticks_lived(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.ticks_lived()
    }
    pub fn set_ticks_lived(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_ticks_lived(arg0)
    }
    pub fn play_effect(
        &self,
        arg0: impl Into<crate::EntityEffect<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.play_effect(arg0)
    }
    pub fn swim_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_sound()
    }
    pub fn swim_splash_sound(&self) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_splash_sound()
    }
    pub fn swim_high_speed_splash_sound(
        &self,
    ) -> Result<crate::Sound<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.swim_high_speed_splash_sound()
    }
    pub fn is_inside_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_inside_vehicle()
    }
    pub fn leave_vehicle(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.leave_vehicle()
    }
    pub fn vehicle(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.vehicle()
    }
    pub fn set_custom_name_visible(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_custom_name_visible(arg0)
    }
    pub fn is_custom_name_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_custom_name_visible()
    }
    pub fn set_visible_by_default(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_visible_by_default(arg0)
    }
    pub fn is_visible_by_default(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_visible_by_default()
    }
    pub fn set_glowing(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_glowing(arg0)
    }
    pub fn is_glowing(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_glowing()
    }
    pub fn set_invulnerable(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_invulnerable(arg0)
    }
    pub fn is_invulnerable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_invulnerable()
    }
    pub fn set_silent(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_silent(arg0)
    }
    pub fn has_gravity(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.has_gravity()
    }
    pub fn set_gravity(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_gravity(arg0)
    }
    pub fn set_portal_cooldown(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.set_portal_cooldown(arg0)
    }
    pub fn scoreboard_tags(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScoreboardTags",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn add_scoreboard_tag(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.add_scoreboard_tag(arg0)
    }
    pub fn remove_scoreboard_tag(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove_scoreboard_tag(arg0)
    }
    pub fn pose(&self) -> Result<crate::entity::Pose<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.pose()
    }
    pub fn spawn_category(
        &self,
    ) -> Result<crate::entity::SpawnCategory<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.spawn_category()
    }
    pub fn spigot(
        &self,
    ) -> Result<crate::command::CommandSenderSpigot<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.spigot()
    }
    pub fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.remove()
    }
    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_empty()
    }
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.location()
    }
    pub fn get_type(&self) -> Result<crate::entity::EntityType<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.get_type()
    }
    pub fn is_frozen(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::entity::Vehicle::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::entity::Entity = temp_clone.into();
        real.is_frozen()
    }
    // SUPER CLASS: Metadatable
    pub fn set_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::metadata::MetadataValue<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.set_metadata(arg0, arg1)
    }
    pub fn get_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<Vec<crate::metadata::MetadataValue<'mc>>, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.get_metadata(arg0)
    }
    pub fn has_metadata(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.has_metadata(arg0)
    }
    pub fn remove_metadata(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::metadata::Metadatable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::metadata::Metadatable = temp_clone.into();
        real.remove_metadata(arg0, arg1)
    }
    // SUPER CLASS: CommandSender
    pub fn is_permission_set_with_permission(
        &self,
        arg0: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn has_permission_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn add_attachment_with_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
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
        if let Some(a) = arg2 {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
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
    pub fn remove_attachment(
        &self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::command::CommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::Permissible = temp_clone.into();
        real.remove_attachment(arg0)
    }
    pub fn recalculate_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn effective_permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: ServerOperator
    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::ServerOperator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.is_op()
    }
    pub fn set_op(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::permissions::ServerOperator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::permissions::ServerOperator = temp_clone.into();
        real.set_op(arg0)
    }
    pub fn send_message_with_strings(
        &self,
        arg0: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[Ljava/lang/String;";
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "java/lang/String",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(arg0.get(i).unwrap().clone())?,
            ));
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "sendMessage", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn send_message_with_uuid(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaUUID<'mc>>,
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
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let temp_clone = crate::command::CommandSender::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::command::CommandSender = temp_clone.into();
        real.name()
    }
    // SUPER CLASS: Nameable
    pub fn custom_name(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.custom_name()
    }
    pub fn set_custom_name(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::Nameable::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::Nameable = temp_clone.into();
        real.set_custom_name(arg0)
    }
    // SUPER CLASS: PersistentDataHolder
    pub fn persistent_data_container(
        &self,
    ) -> Result<crate::persistence::PersistentDataContainer<'mc>, Box<dyn std::error::Error>> {
        let temp_clone = crate::persistence::PersistentDataHolder::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::persistence::PersistentDataHolder = temp_clone.into();
        real.persistent_data_container()
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
