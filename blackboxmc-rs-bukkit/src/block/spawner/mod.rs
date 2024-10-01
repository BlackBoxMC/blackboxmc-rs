#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct SpawnRule<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SpawnRule<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpawnRule<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SpawnRule from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/spawner/SpawnRule")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnRule object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SpawnRule<'mc> {
    /// Constructs a new SpawnRule.
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        min_block_light: i32,
        max_block_light: i32,
        min_sky_light: i32,
        max_sky_light: i32,
    ) -> Result<crate::block::spawner::SpawnRule<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(IIII)V");
        let val_1 = jni::objects::JValueGen::Int(min_block_light);
        let val_2 = jni::objects::JValueGen::Int(max_block_light);
        let val_3 = jni::objects::JValueGen::Int(min_sky_light);
        let val_4 = jni::objects::JValueGen::Int(max_sky_light);
        let cls = jni.find_class("org/bukkit/block/spawner/SpawnRule");
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
        crate::block::spawner::SpawnRule::from_raw(&jni, res)
    }
    /// Gets the minimum (inclusive) block light required for spawning to
    /// succeed.
    pub fn min_block_light(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMinBlockLight",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the minimum (inclusive) block light required for spawning to
    /// succeed.
    pub fn set_min_block_light(
        &self,
        min_block_light: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(min_block_light);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMinBlockLight",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum (inclusive) block light required for spawning to
    /// succeed.
    pub fn max_block_light(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxBlockLight",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the maximum (inclusive) block light required for spawning to
    /// succeed.
    pub fn set_max_block_light(
        &self,
        max_block_light: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(max_block_light);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxBlockLight",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the minimum (inclusive) sky light required for spawning to succeed.
    pub fn min_sky_light(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinSkyLight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the minimum (inclusive) sky light required for spawning to succeed.
    pub fn set_min_sky_light(&self, min_sky_light: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(min_sky_light);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMinSkyLight",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the maximum (inclusive) sky light required for spawning to succeed.
    pub fn max_sky_light(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxSkyLight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the maximum (inclusive) sky light required for spawning to succeed.
    pub fn set_max_sky_light(&self, max_sky_light: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(max_sky_light);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSkyLight",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &self,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(obj);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn clone(
        &self,
    ) -> Result<crate::block::spawner::SpawnRule<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/spawner/SpawnRule;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clone", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::spawner::SpawnRule::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn deserialize(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        val_args: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<crate::block::spawner::SpawnRule<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)Lorg/bukkit/block/spawner/SpawnRule;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_args.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/block/spawner/SpawnRule");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "deserialize",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::spawner::SpawnRule::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for SpawnRule<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting SpawnRule into crate::configuration::serialization::ConfigurationSerializable")
    }
}
#[repr(C)]
pub struct SpawnerEntry<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SpawnerEntry<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpawnerEntry<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate SpawnerEntry from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/spawner/SpawnerEntry")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnerEntry object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SpawnerEntry<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        snapshot: impl Into<crate::entity::EntitySnapshot<'mc>>,
        spawn_weight: i32,
        spawn_rule: impl Into<crate::block::spawner::SpawnRule<'mc>>,
        equipment: std::option::Option<
            impl Into<crate::block::spawner::SpawnerEntryEquipment<'mc>>,
        >,
    ) -> Result<crate::block::spawner::SpawnerEntry<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/EntitySnapshot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(snapshot.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(spawn_weight);
        args.push(val_2);
        sig += "Lorg/bukkit/block/spawner/SpawnRule;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawn_rule.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = equipment {
            sig += "Lorg/bukkit/block/spawner/SpawnerEntry/Equipment;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/block/spawner/SpawnerEntry");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::block::spawner::SpawnerEntry::from_raw(&jni, res)
    }
    /// Gets the {@link EntitySnapshot} for this SpawnerEntry.
    pub fn snapshot(
        &self,
    ) -> Result<crate::entity::EntitySnapshot<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntitySnapshot;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSnapshot", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::EntitySnapshot::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the {@link EntitySnapshot} for this SpawnerEntry.
    pub fn set_snapshot(
        &self,
        snapshot: impl Into<crate::entity::EntitySnapshot<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/EntitySnapshot;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(snapshot.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSnapshot",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the weight for this SpawnerEntry, when added to a spawner entries
    /// with higher weight will spawn more often.
    pub fn spawn_weight(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnWeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Sets the weight for this SpawnerEntry, when added to a spawner entries
    /// with higher weight will spawn more often.
    pub fn set_spawn_weight(&self, spawn_weight: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(spawn_weight);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnWeight",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a copy of the {@link SpawnRule} for this SpawnerEntry, or null if
    /// none has been set.
    pub fn spawn_rule(
        &self,
    ) -> Result<Option<crate::block::spawner::SpawnRule<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/block/spawner/SpawnRule;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnRule", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::block::spawner::SpawnRule::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the {@link SpawnRule} for this SpawnerEntry, null may be used to
    /// clear the current spawn rule.
    pub fn set_spawn_rule(
        &self,
        spawn_rule: impl Into<crate::block::spawner::SpawnRule<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/spawner/SpawnRule;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawn_rule.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnRule",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the equipment which will be applied to the spawned entity.
    pub fn equipment(
        &self,
    ) -> Result<Option<crate::block::spawner::SpawnerEntryEquipment<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/block/spawner/SpawnerEntry/Equipment;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEquipment", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            crate::block::spawner::SpawnerEntryEquipment::from_raw(&self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })?,
        ))
    }
    /// Sets the equipment which will be applied to the spawned entity.
    pub fn set_equipment(
        &self,
        equipment: impl Into<crate::block::spawner::SpawnerEntryEquipment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/block/spawner/SpawnerEntry/Equipment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(equipment.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEquipment",
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
#[repr(C)]
pub struct SpawnerEntryEquipment<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SpawnerEntryEquipment<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SpawnerEntryEquipment<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate SpawnerEntryEquipment from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/block/spawner/SpawnerEntry/Equipment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SpawnerEntryEquipment object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> SpawnerEntryEquipment<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        equipment_loot_table: impl Into<crate::loot::LootTable<'mc>>,
        drop_chances: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<crate::block::spawner::SpawnerEntryEquipment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/loot/LootTable;Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(equipment_loot_table.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(drop_chances.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/block/spawner/SpawnerEntry/Equipment");
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
        crate::block::spawner::SpawnerEntryEquipment::from_raw(&jni, res)
    }
    /// Set the loot table for the entity.
    ///
    /// To remove a loot table use null. Do not use {@link LootTables#EMPTY}
    /// to clear a LootTable.
    pub fn set_equipment_loot_table(
        &self,
        table: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/loot/LootTable;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(table.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setEquipmentLootTable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the loot table for the entity.
    ///
    /// If an entity does not have a loot table, this will return null, NOT
    /// an empty loot table.
    pub fn equipment_loot_table(
        &self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootTable;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEquipmentLootTable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a mutable map of the drop chances for each slot of the entity.
    /// If non-null, the entity's drop chances will be overridden with the
    /// given value.
    pub fn drop_chances(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDropChances", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
