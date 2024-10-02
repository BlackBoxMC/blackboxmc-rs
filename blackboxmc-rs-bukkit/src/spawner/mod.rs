#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct BaseSpawner<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BaseSpawner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BaseSpawner<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BaseSpawner from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/spawner/BaseSpawner")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BaseSpawner object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BaseSpawner<'mc> {
    /// Get the spawner's creature type.
    pub fn spawned_type(
        &self,
    ) -> Result<Option<crate::entity::EntityType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnedType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Set the spawner's creature type.
    ///
    /// This will override any entities that have been added with {@link #addPotentialSpawn}
    pub fn set_spawned_type(
        &self,
        creature_type: impl Into<crate::entity::EntityType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/EntityType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(creature_type.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnedType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the spawner's delay.
    ///
    /// This is the delay, in ticks, until the spawner will spawn its next mob.
    pub fn delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDelay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the spawner's delay.
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
    /// Get the maximum distance(squared) a player can be in order for this
    /// spawner to be active.
    ///
    /// If this value is less than or equal to 0, this spawner is always active
    /// (given that there are players online).
    ///
    /// Default value is 16.
    pub fn required_player_range(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRequiredPlayerRange",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the maximum distance (squared) a player can be in order for this
    /// spawner to be active.
    ///
    /// Setting this value to less than or equal to 0 will make this spawner
    /// always active (given that there are players online).
    pub fn set_required_player_range(
        &self,
        required_player_range: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(required_player_range);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRequiredPlayerRange",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the radius around which the spawner will attempt to spawn mobs in.
    ///
    /// This area is square, includes the block the spawner is in, and is
    /// centered on the spawner's x,z coordinates - not the spawner itself.
    ///
    /// It is 2 blocks high, centered on the spawner's y-coordinate (its bottom);
    /// thus allowing mobs to spawn as high as its top surface and as low
    /// as 1 block below its bottom surface.
    ///
    /// Default value is 4.
    pub fn spawn_range(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnRange", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the new spawn range.
    ///
    pub fn set_spawn_range(&self, spawn_range: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(spawn_range);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnRange",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the {@link EntitySnapshot} that will be spawned by this spawner or null
    /// if no entities have been assigned to this spawner.
    ///
    ///
    /// All applicable data from the spawner will be copied, such as custom name,
    /// health, and velocity.
    ///
    pub fn spawned_entity(
        &self,
    ) -> Result<Option<crate::entity::EntitySnapshot<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntitySnapshot;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnedEntity",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntitySnapshot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the {@link SpawnerEntry} that will be spawned by this spawner.
    ///
    /// This will override any previous entries that have been added with
    /// {@link #addPotentialSpawn}
    pub fn set_spawned_entity(
        &self,
        spawner_entry: impl Into<crate::block::spawner::SpawnerEntry<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/spawner/SpawnerEntry;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawner_entry.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setSpawnedEntity", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds a new {@link EntitySnapshot} to the list of entities this spawner can
    /// spawn.
    ///
    /// The weight will determine how often this entry is chosen to spawn, higher
    /// weighted entries will spawn more often than lower weighted ones.
    ///
    /// The {@link SpawnRule} will determine under what conditions this entry can
    /// spawn, passing null will use the default conditions for the given entity.
    pub fn add_potential_spawn(
        &self,
        snapshot: impl Into<crate::entity::EntitySnapshot<'mc>>,
        weight: std::option::Option<i32>,
        spawn_rule: std::option::Option<impl Into<crate::block::spawner::SpawnRule<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/EntitySnapshot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(snapshot.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = weight {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = spawn_rule {
            sig += "Lorg/bukkit/block/spawner/SpawnRule;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addPotentialSpawn", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a list of potential spawns from this spawner or an empty list if no
    /// entities have been assigned to this spawner.
    ///
    /// Changes made to the returned list will not be reflected in the spawner unless
    /// applied with {@link #setPotentialSpawns}
    pub fn potential_spawns(
        &self,
    ) -> Result<Vec<crate::block::spawner::SpawnerEntry<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPotentialSpawns",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::block::spawner::SpawnerEntry::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct TrialSpawnerConfiguration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for TrialSpawnerConfiguration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for TrialSpawnerConfiguration<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TrialSpawnerConfiguration from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/spawner/TrialSpawnerConfiguration")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TrialSpawnerConfiguration object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> TrialSpawnerConfiguration<'mc> {
    /// Gets the base number of entities the spawner will spawn before going into
    /// cooldown.
    pub fn base_spawns_before_cooldown(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBaseSpawnsBeforeCooldown",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the base number of entities the spawner will spawn before going into
    /// cooldown.
    pub fn set_base_spawns_before_cooldown(
        &self,
        amount: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(amount);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBaseSpawnsBeforeCooldown",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the base number of entities this spawner can track at once.
    ///
    /// If the limit is reached the spawner will not be able to spawn any more
    /// entities until the existing entities are killed or move too far away.
    pub fn base_simultaneous_entities(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBaseSimultaneousEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the base number of entities this spawner can track at once.
    ///
    /// If the limit is reached the spawner will not be able to spawn any more
    /// entities until the existing entities are killed or move too far away.
    pub fn set_base_simultaneous_entities(
        &self,
        amount: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(amount);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBaseSimultaneousEntities",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the additional number of entities the spawner will spawn per tracked player
    /// before going into cooldown.
    pub fn additional_spawns_before_cooldown(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAdditionalSpawnsBeforeCooldown",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the additional number of entities the spawner will spawn per tracked player
    /// before going into cooldown.
    pub fn set_additional_spawns_before_cooldown(
        &self,
        amount: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(amount);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAdditionalSpawnsBeforeCooldown",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the additional number of entities this spawner can track at once per
    /// tracked player.
    ///
    /// If the limit is reached the spawner will not be able to spawn any more
    /// entities until the existing entities are killed or move too far away.
    pub fn additional_simultaneous_entities(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAdditionalSimultaneousEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    /// Sets the additional number of entities this spawner can track at once per
    /// tracked player.
    ///
    /// If the limit is reached the spawner will not be able to spawn any more
    /// entities until the existing entities are killed or move too far away.
    pub fn set_additional_simultaneous_entities(
        &self,
        amount: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(F)V");
        let val_1 = jni::objects::JValueGen::Float(amount);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setAdditionalSimultaneousEntities",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a list of {@link LootTable}s this spawner can pick a reward from as
    /// well as their associated weight to be chosen.
    pub fn possible_rewards(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPossibleRewards",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Add a {@link LootTable} to the list of tables this spawner can pick a reward
    /// from with a given weight.
    pub fn add_possible_reward(
        &self,
        table: impl Into<crate::loot::LootTable<'mc>>,
        weight: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/loot/LootTable;I)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(table.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(weight);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPossibleReward",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Removes the provided {@link LootTable} from the list of tables this spawner
    /// can pick a reward from.
    pub fn remove_possible_reward(
        &self,
        table: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/loot/LootTable;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(table.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePossibleReward",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the list of {@link LootTable}s and their weights this spawner can pick a
    /// reward from.
    ///
    /// All loot tables in the map must be non-null and all weights must be at least
    /// 1.
    pub fn set_possible_rewards(
        &self,
        rewards: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(rewards.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPossibleRewards",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the spawner's creature type.
    pub fn spawned_type(
        &self,
    ) -> Result<Option<crate::entity::EntityType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnedType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Set the spawner's creature type.
    ///
    /// This will override any entities that have been added with {@link #addPotentialSpawn}
    pub fn set_spawned_type(
        &self,
        creature_type: impl Into<crate::entity::EntityType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/EntityType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(creature_type.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnedType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the spawner's delay.
    ///
    /// This is the delay, in ticks, until the spawner will spawn its next mob.
    pub fn delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDelay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the spawner's delay.
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
    /// Get the maximum distance(squared) a player can be in order for this
    /// spawner to be active.
    ///
    /// If this value is less than or equal to 0, this spawner is always active
    /// (given that there are players online).
    ///
    /// Default value is 16.
    pub fn required_player_range(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRequiredPlayerRange",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the maximum distance (squared) a player can be in order for this
    /// spawner to be active.
    ///
    /// Setting this value to less than or equal to 0 will make this spawner
    /// always active (given that there are players online).
    pub fn set_required_player_range(
        &self,
        required_player_range: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(required_player_range);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRequiredPlayerRange",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the radius around which the spawner will attempt to spawn mobs in.
    ///
    /// This area is square, includes the block the spawner is in, and is
    /// centered on the spawner's x,z coordinates - not the spawner itself.
    ///
    /// It is 2 blocks high, centered on the spawner's y-coordinate (its bottom);
    /// thus allowing mobs to spawn as high as its top surface and as low
    /// as 1 block below its bottom surface.
    ///
    /// Default value is 4.
    pub fn spawn_range(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnRange", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the new spawn range.
    ///
    pub fn set_spawn_range(&self, spawn_range: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(spawn_range);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnRange",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the {@link EntitySnapshot} that will be spawned by this spawner or null
    /// if no entities have been assigned to this spawner.
    ///
    ///
    /// All applicable data from the spawner will be copied, such as custom name,
    /// health, and velocity.
    ///
    pub fn spawned_entity(
        &self,
    ) -> Result<Option<crate::entity::EntitySnapshot<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntitySnapshot;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnedEntity",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntitySnapshot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the {@link SpawnerEntry} that will be spawned by this spawner.
    ///
    /// This will override any previous entries that have been added with
    /// {@link #addPotentialSpawn}
    pub fn set_spawned_entity(
        &self,
        spawner_entry: impl Into<crate::block::spawner::SpawnerEntry<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/spawner/SpawnerEntry;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawner_entry.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setSpawnedEntity", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds a new {@link EntitySnapshot} to the list of entities this spawner can
    /// spawn.
    ///
    /// The weight will determine how often this entry is chosen to spawn, higher
    /// weighted entries will spawn more often than lower weighted ones.
    ///
    /// The {@link SpawnRule} will determine under what conditions this entry can
    /// spawn, passing null will use the default conditions for the given entity.
    pub fn add_potential_spawn(
        &self,
        snapshot: impl Into<crate::entity::EntitySnapshot<'mc>>,
        weight: std::option::Option<i32>,
        spawn_rule: std::option::Option<impl Into<crate::block::spawner::SpawnRule<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/EntitySnapshot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(snapshot.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = weight {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = spawn_rule {
            sig += "Lorg/bukkit/block/spawner/SpawnRule;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addPotentialSpawn", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a list of potential spawns from this spawner or an empty list if no
    /// entities have been assigned to this spawner.
    ///
    /// Changes made to the returned list will not be reflected in the spawner unless
    /// applied with {@link #setPotentialSpawns}
    pub fn potential_spawns(
        &self,
    ) -> Result<Vec<crate::block::spawner::SpawnerEntry<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPotentialSpawns",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::block::spawner::SpawnerEntry::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::spawner::BaseSpawner<'mc>> for TrialSpawnerConfiguration<'mc> {
    fn into(self) -> crate::spawner::BaseSpawner<'mc> {
        crate::spawner::BaseSpawner::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting TrialSpawnerConfiguration into crate::spawner::BaseSpawner")
    }
}
#[repr(C)]
pub struct Spawner<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Spawner<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Spawner<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Spawner from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/spawner/Spawner")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Spawner object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Spawner<'mc> {
    /// {@inheritDoc}
    ///
    /// If set to -1, the spawn delay will be reset to a random value between
    /// {@link #getMinSpawnDelay} and {@link #getMaxSpawnDelay()}.
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
    /// The minimum spawn delay amount (in ticks).
    ///
    /// This value is used when the spawner resets its delay (for any reason).
    /// It will choose a random number between {@link #getMinSpawnDelay()}
    /// and {@link #getMaxSpawnDelay()} for its next {@link #getDelay()}.
    ///
    /// Default value is 200 ticks.
    pub fn min_spawn_delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMinSpawnDelay",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the minimum spawn delay amount (in ticks).
    pub fn set_min_spawn_delay(&self, delay: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(delay);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMinSpawnDelay",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// The maximum spawn delay amount (in ticks).
    ///
    /// This value is used when the spawner resets its delay (for any reason).
    /// It will choose a random number between {@link #getMinSpawnDelay()}
    /// and {@link #getMaxSpawnDelay()} for its next {@link #getDelay()}.
    ///
    /// This value <b>must</b> be greater than 0 and less than or equal to
    /// {@link #getMaxSpawnDelay()}.
    ///
    /// Default value is 800 ticks.
    pub fn max_spawn_delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxSpawnDelay",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the maximum spawn delay amount (in ticks).
    ///
    /// This value <b>must</b> be greater than 0, as well as greater than or
    /// equal to {@link #getMinSpawnDelay()}
    pub fn set_max_spawn_delay(&self, delay: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(delay);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxSpawnDelay",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get how many mobs attempt to spawn.
    ///
    /// Default value is 4.
    pub fn spawn_count(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnCount", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set how many mobs attempt to spawn.
    pub fn set_spawn_count(&self, spawn_count: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(spawn_count);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnCount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Set the new maximum amount of similar entities that are allowed to be
    /// within spawning range of this spawner.
    ///
    /// If more than the maximum number of entities are within range, the spawner
    /// will not spawn and try again with a new {@link #getDelay()}.
    ///
    /// Default value is 16.
    pub fn max_nearby_entities(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxNearbyEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the maximum number of similar entities that are allowed to be within
    /// spawning range of this spawner.
    ///
    /// Similar entities are entities that are of the same {@link EntityType}
    pub fn set_max_nearby_entities(
        &self,
        max_nearby_entities: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(max_nearby_entities);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setMaxNearbyEntities",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the spawner's creature type.
    pub fn spawned_type(
        &self,
    ) -> Result<Option<crate::entity::EntityType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntityType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnedType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntityType::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Set the spawner's creature type.
    ///
    /// This will override any entities that have been added with {@link #addPotentialSpawn}
    pub fn set_spawned_type(
        &self,
        creature_type: impl Into<crate::entity::EntityType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/EntityType;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(creature_type.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnedType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the spawner's delay.
    ///
    /// This is the delay, in ticks, until the spawner will spawn its next mob.
    pub fn delay(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDelay", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the maximum distance(squared) a player can be in order for this
    /// spawner to be active.
    ///
    /// If this value is less than or equal to 0, this spawner is always active
    /// (given that there are players online).
    ///
    /// Default value is 16.
    pub fn required_player_range(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRequiredPlayerRange",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the maximum distance (squared) a player can be in order for this
    /// spawner to be active.
    ///
    /// Setting this value to less than or equal to 0 will make this spawner
    /// always active (given that there are players online).
    pub fn set_required_player_range(
        &self,
        required_player_range: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(required_player_range);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRequiredPlayerRange",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the radius around which the spawner will attempt to spawn mobs in.
    ///
    /// This area is square, includes the block the spawner is in, and is
    /// centered on the spawner's x,z coordinates - not the spawner itself.
    ///
    /// It is 2 blocks high, centered on the spawner's y-coordinate (its bottom);
    /// thus allowing mobs to spawn as high as its top surface and as low
    /// as 1 block below its bottom surface.
    ///
    /// Default value is 4.
    pub fn spawn_range(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getSpawnRange", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Set the new spawn range.
    ///
    pub fn set_spawn_range(&self, spawn_range: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(spawn_range);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSpawnRange",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the {@link EntitySnapshot} that will be spawned by this spawner or null
    /// if no entities have been assigned to this spawner.
    ///
    ///
    /// All applicable data from the spawner will be copied, such as custom name,
    /// health, and velocity.
    ///
    pub fn spawned_entity(
        &self,
    ) -> Result<Option<crate::entity::EntitySnapshot<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/EntitySnapshot;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSpawnedEntity",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::EntitySnapshot::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Sets the {@link SpawnerEntry} that will be spawned by this spawner.
    ///
    /// This will override any previous entries that have been added with
    /// {@link #addPotentialSpawn}
    pub fn set_spawned_entity(
        &self,
        spawner_entry: impl Into<crate::block::spawner::SpawnerEntry<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/block/spawner/SpawnerEntry;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(spawner_entry.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setSpawnedEntity", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds a new {@link EntitySnapshot} to the list of entities this spawner can
    /// spawn.
    ///
    /// The weight will determine how often this entry is chosen to spawn, higher
    /// weighted entries will spawn more often than lower weighted ones.
    ///
    /// The {@link SpawnRule} will determine under what conditions this entry can
    /// spawn, passing null will use the default conditions for the given entity.
    pub fn add_potential_spawn(
        &self,
        snapshot: impl Into<crate::entity::EntitySnapshot<'mc>>,
        weight: std::option::Option<i32>,
        spawn_rule: std::option::Option<impl Into<crate::block::spawner::SpawnRule<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/entity/EntitySnapshot;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(snapshot.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = weight {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = spawn_rule {
            sig += "Lorg/bukkit/block/spawner/SpawnRule;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addPotentialSpawn", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a list of potential spawns from this spawner or an empty list if no
    /// entities have been assigned to this spawner.
    ///
    /// Changes made to the returned list will not be reflected in the spawner unless
    /// applied with {@link #setPotentialSpawns}
    pub fn potential_spawns(
        &self,
    ) -> Result<Vec<crate::block::spawner::SpawnerEntry<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPotentialSpawns",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::block::spawner::SpawnerEntry::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::spawner::BaseSpawner<'mc>> for Spawner<'mc> {
    fn into(self) -> crate::spawner::BaseSpawner<'mc> {
        crate::spawner::BaseSpawner::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Spawner into crate::spawner::BaseSpawner")
    }
}
