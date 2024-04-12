#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct ChunkGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ChunkGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ChunkGenerator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ChunkGenerator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/ChunkGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChunkGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ChunkGenerator<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "ChunkGenerator", name, lib_name) }?;
        Self::from_raw(env, obj)
    }

    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::generator::ChunkGenerator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/generator/ChunkGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::generator::ChunkGenerator::from_raw(&jni, res)
    }
    /// Shapes the Chunk noise for the given coordinates.
    ///
    /// Notes:
    ///
    /// This method should <b>never</b> attempt to get the Chunk at the passed
    /// coordinates, as doing so may cause an infinite loop.
    ///
    /// This method should <b>never</b> modify the {@link ChunkData} at a later
    /// point of time.
    ///
    /// The Y-coordinate range should <b>never</b> be hardcoded, to get the
    /// Y-coordinate range use the methods {@link ChunkData#getMinHeight()} and
    /// {@link ChunkData#getMaxHeight()}.
    ///
    /// If {@link #shouldGenerateNoise()} is set to true, the given
    /// {@link ChunkData} contains already the Vanilla noise generation.
    pub fn generate_noise(
        &self,
        world_info: impl Into<crate::generator::WorldInfo<'mc>>,
        random: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        chunk_x: i32,
        chunk_z: i32,
        chunk_data: impl Into<crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator/ChunkData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world_info.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(random.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(chunk_x);
        let val_4 = jni::objects::JValueGen::Int(chunk_z);
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(chunk_data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "generateNoise",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Shapes the Chunk surface for the given coordinates.
    ///
    /// Notes:
    ///
    /// This method should <b>never</b> attempt to get the Chunk at the passed
    /// coordinates, as doing so may cause an infinite loop.
    ///
    /// This method should <b>never</b> modify the {@link ChunkData} at a later
    /// point of time.
    ///
    /// The Y-coordinate range should <b>never</b> be hardcoded, to get the
    /// Y-coordinate range use the methods {@link ChunkData#getMinHeight()} and
    /// {@link ChunkData#getMaxHeight()}.
    ///
    /// If {@link #shouldGenerateSurface()} is set to true, the given
    /// {@link ChunkData} contains already the Vanilla surface generation.
    pub fn generate_surface(
        &self,
        world_info: impl Into<crate::generator::WorldInfo<'mc>>,
        random: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        chunk_x: i32,
        chunk_z: i32,
        chunk_data: impl Into<crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator/ChunkData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world_info.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(random.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(chunk_x);
        let val_4 = jni::objects::JValueGen::Int(chunk_z);
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(chunk_data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "generateSurface",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Shapes the Chunk bedrock layer for the given coordinates.
    ///
    /// Notes:
    ///
    /// This method should <b>never</b> attempt to get the Chunk at the passed
    /// coordinates, as doing so may cause an infinite loop.
    ///
    /// This method should <b>never</b> modify the {@link ChunkData} at a later
    /// point of time.
    ///
    /// The Y-coordinate range should <b>never</b> be hardcoded, to get the
    /// Y-coordinate range use the methods {@link ChunkData#getMinHeight()} and
    /// {@link ChunkData#getMaxHeight()}.
    ///
    pub fn generate_bedrock(
        &self,
        world_info: impl Into<crate::generator::WorldInfo<'mc>>,
        random: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        chunk_x: i32,
        chunk_z: i32,
        chunk_data: impl Into<crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator/ChunkData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world_info.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(random.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(chunk_x);
        let val_4 = jni::objects::JValueGen::Int(chunk_z);
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(chunk_data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "generateBedrock",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Shapes the Chunk caves for the given coordinates.
    ///
    /// Notes:
    ///
    /// This method should <b>never</b> attempt to get the Chunk at the passed
    /// coordinates, as doing so may cause an infinite loop.
    ///
    /// This method should <b>never</b> modify the {@link ChunkData} at a later
    /// point of time.
    ///
    /// The Y-coordinate range should <b>never</b> be hardcoded, to get the
    /// Y-coordinate range use the methods {@link ChunkData#getMinHeight()} and
    /// {@link ChunkData#getMaxHeight()}.
    ///
    /// If {@link #shouldGenerateCaves()} is set to true, the given
    /// {@link ChunkData} contains already the Vanilla cave generation.
    pub fn generate_caves(
        &self,
        world_info: impl Into<crate::generator::WorldInfo<'mc>>,
        random: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        chunk_x: i32,
        chunk_z: i32,
        chunk_data: impl Into<crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator/ChunkData;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world_info.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(random.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(chunk_x);
        let val_4 = jni::objects::JValueGen::Int(chunk_z);
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(chunk_data.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "generateCaves",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets called when no {@link BiomeProvider} is set in
    /// {@link org.bukkit.WorldCreator} or via the server configuration files. It
    /// is therefore possible that one plugin can provide the Biomes and another
    /// one the generation.
    ///
    /// Notes:
    ///
    /// If <code>null</code> is returned, than Vanilla biomes are used.
    ///
    /// This method only gets called once when the world is loaded. Returning
    /// another {@link BiomeProvider} later one is not respected.
    pub fn get_default_biome_provider(
        &self,
        world_info: impl Into<crate::generator::WorldInfo<'mc>>,
    ) -> Result<Option<crate::generator::BiomeProvider<'mc>>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/generator/WorldInfo;)Lorg/bukkit/generator/BiomeProvider;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world_info.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultBiomeProvider",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::generator::BiomeProvider::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// This method is similar to
    /// {@link World#getHighestBlockAt(int, int, HeightMap)}. With the difference
    /// being, that the highest y coordinate should be the block before any
    /// surface, bedrock, caves or decoration is applied. Or in other words the
    /// highest block when only the noise is present at the chunk.
    ///
    /// Notes:
    ///
    /// When this method is not overridden, the Vanilla base height is used.
    ///
    /// This method should <b>never</b> attempt to get the Chunk at the passed
    /// coordinates, or use the method
    /// {@link World#getHighestBlockAt(int, int, HeightMap)}, as doing so may
    /// cause an infinite loop.
    pub fn get_base_height(
        &self,
        world_info: impl Into<crate::generator::WorldInfo<'mc>>,
        random: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        x: i32,
        z: i32,
        height_map: impl Into<crate::HeightMap<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/HeightMap;)I",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world_info.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(random.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(x);
        let val_4 = jni::objects::JValueGen::Int(z);
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(height_map.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBaseHeight",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    #[deprecated]
    /// Shapes the chunk for the given coordinates. This method must return a ChunkData.Notes:This method should <b>never</b> attempt to get the Chunk at the passed coordinates, as doing so may cause an infinite loopThis method should <b>never</b> modify a ChunkData after it has been returned.This method <b>must</b> return a ChunkData returned by {@link ChunkGenerator#createChunkData(org.bukkit.World)}
    pub fn generate_chunk_data(
        &self,
        world: impl Into<crate::World<'mc>>,
        random: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        x: i32,
        z: i32,
        biome: impl Into<crate::generator::ChunkGeneratorBiomeGrid<'mc>>,
    ) -> Result<crate::generator::ChunkGeneratorChunkData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator/BiomeGrid;)Lorg/bukkit/generator/ChunkGenerator/ChunkData;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(random.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Int(x);
        let val_4 = jni::objects::JValueGen::Int(z);
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(biome.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "generateChunkData",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::ChunkGeneratorChunkData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Tests if the specified location is valid for a natural spawn position
    pub fn can_spawn(
        &self,
        world: impl Into<crate::World<'mc>>,
        x: i32,
        z: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;II)Z");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(x);
        let val_3 = jni::objects::JValueGen::Int(z);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canSpawn",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets a list of default {@link BlockPopulator}s to apply to a given
    /// world
    pub fn get_default_populators(
        &self,
        world: impl Into<crate::World<'mc>>,
    ) -> Result<Vec<crate::generator::BlockPopulator<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultPopulators",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::generator::BlockPopulator::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    /// Gets a fixed spawn location to use for a given world.
    ///
    /// A null value is returned if a world should not use a fixed spawn point,
    /// and will instead attempt to find one randomly.
    pub fn get_fixed_spawn_location(
        &self,
        world: impl Into<crate::World<'mc>>,
        random: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
    ) -> Result<Option<crate::Location<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;Ljava/util/Random;)Lorg/bukkit/Location;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(random.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFixedSpawnLocation",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    #[deprecated]
    /// Gets if this ChunkGenerator is parallel capable. See {@link ChunkGenerator} for more information.
    pub fn is_parallel_capable(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isParallelCapable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets if the server should generate Vanilla noise.
    ///
    /// The Vanilla noise is generated <b>before</b>
    /// {@link #generateNoise(WorldInfo, Random, int, int, ChunkData)} is called.
    ///
    /// Only this method is called if both this and
    /// {@link #shouldGenerateNoise()} are overridden.
    pub fn should_generate_noise(
        &self,
        world_info: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        random: std::option::Option<impl Into<blackboxmc_java::util::JavaRandom<'mc>>>,
        chunk_x: std::option::Option<i32>,
        chunk_z: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = world_info {
            sig += "Lorg/bukkit/generator/WorldInfo;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = random {
            sig += "Ljava/util/Random;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = chunk_x {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = chunk_z {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateNoise",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets if the server should generate Vanilla surface.
    ///
    /// The Vanilla surface is generated <b>before</b>
    /// {@link #generateSurface(WorldInfo, Random, int, int, ChunkData)} is
    /// called.
    ///
    /// Only this method is called if both this and
    /// {@link #shouldGenerateSurface()} are overridden.
    pub fn should_generate_surface(
        &self,
        world_info: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        random: std::option::Option<impl Into<blackboxmc_java::util::JavaRandom<'mc>>>,
        chunk_x: std::option::Option<i32>,
        chunk_z: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = world_info {
            sig += "Lorg/bukkit/generator/WorldInfo;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = random {
            sig += "Ljava/util/Random;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = chunk_x {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = chunk_z {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateSurface",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]
    /// Gets if the server should generate Vanilla bedrock.The Vanilla bedrock is generated <b>before</b> {@link #generateBedrock(WorldInfo, Random, int, int, ChunkData)} is called.
    pub fn should_generate_bedrock(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateBedrock",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets if the server should generate Vanilla caves.
    ///
    /// The Vanilla caves are generated <b>before</b>
    /// {@link #generateCaves(WorldInfo, Random, int, int, ChunkData)} is called.
    ///
    /// Only this method is called if both this and
    /// {@link #shouldGenerateCaves()} are overridden.
    pub fn should_generate_caves(
        &self,
        world_info: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        random: std::option::Option<impl Into<blackboxmc_java::util::JavaRandom<'mc>>>,
        chunk_x: std::option::Option<i32>,
        chunk_z: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = world_info {
            sig += "Lorg/bukkit/generator/WorldInfo;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = random {
            sig += "Ljava/util/Random;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = chunk_x {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = chunk_z {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateCaves",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets if the server should generate Vanilla decorations after this
    /// ChunkGenerator.
    ///
    /// The Vanilla decoration are generated <b>before</b> any
    /// {@link BlockPopulator} are called.
    ///
    /// Only this method is called if both this and
    /// {@link #shouldGenerateDecorations()} are overridden.
    pub fn should_generate_decorations(
        &self,
        world_info: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        random: std::option::Option<impl Into<blackboxmc_java::util::JavaRandom<'mc>>>,
        chunk_x: std::option::Option<i32>,
        chunk_z: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = world_info {
            sig += "Lorg/bukkit/generator/WorldInfo;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = random {
            sig += "Ljava/util/Random;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = chunk_x {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = chunk_z {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateDecorations",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets if the server should generate Vanilla mobs after this
    /// ChunkGenerator.
    ///
    /// Only this method is called if both this and
    /// {@link #shouldGenerateMobs()} are overridden.
    pub fn should_generate_mobs(
        &self,
        world_info: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        random: std::option::Option<impl Into<blackboxmc_java::util::JavaRandom<'mc>>>,
        chunk_x: std::option::Option<i32>,
        chunk_z: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = world_info {
            sig += "Lorg/bukkit/generator/WorldInfo;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = random {
            sig += "Ljava/util/Random;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = chunk_x {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = chunk_z {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateMobs",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets if the server should generate Vanilla structures after this
    /// ChunkGenerator.
    ///
    /// Only this method is called if both this and
    /// {@link #shouldGenerateStructures()} are overridden.
    pub fn should_generate_structures(
        &self,
        world_info: std::option::Option<impl Into<crate::generator::WorldInfo<'mc>>>,
        random: std::option::Option<impl Into<blackboxmc_java::util::JavaRandom<'mc>>>,
        chunk_x: std::option::Option<i32>,
        chunk_z: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = world_info {
            sig += "Lorg/bukkit/generator/WorldInfo;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        if let Some(a) = random {
            sig += "Ljava/util/Random;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        if let Some(a) = chunk_x {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = chunk_z {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")Z";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateStructures",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct ChunkGeneratorChunkData<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ChunkGeneratorChunkData<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ChunkGeneratorChunkData<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ChunkGeneratorChunkData from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/generator/ChunkGenerator/ChunkData")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChunkGeneratorChunkData object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ChunkGeneratorChunkData<'mc> {
    /// Get the minimum height for this ChunkData.
    ///
    /// It is not guaranteed that this method will return the same value as
    /// {@link World#getMinHeight()}.
    ///
    /// Setting blocks below this height will do nothing.
    pub fn min_height(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the maximum height for this ChunkData.
    ///
    /// It is not guaranteed that this method will return the same value as
    /// {@link World#getMaxHeight()}.
    ///
    /// Setting blocks at or above this height will do nothing.
    pub fn max_height(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the biome at x, y, z within chunk being generated
    pub fn get_biome(
        &self,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(III)Lorg/bukkit/block/Biome;");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let val_3 = jni::objects::JValueGen::Int(z);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBiome",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Biome::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the block at x,y,z in the chunk data to material.
    /// Setting blocks outside the chunk's bounds does nothing.
    pub fn set_block(
        &self,
        x: i32,
        y: i32,
        z: i32,
        block_data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x);
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(y);
        args.push(val_2);
        sig += "I";
        let val_3 = jni::objects::JValueGen::Int(z);
        args.push(val_3);
        sig += "Lorg/bukkit/block/data/BlockData;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_data.into().jni_object().clone())
        });
        args.push(val_4);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setBlock", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Set a region of this chunk from xMin, yMin, zMin (inclusive) to xMax,
    /// yMax, zMax (exclusive) to material.
    /// Setting blocks outside the chunk's bounds does nothing.
    pub fn set_region(
        &self,
        x_min: i32,
        y_min: i32,
        z_min: i32,
        x_max: i32,
        y_max: i32,
        z_max: i32,
        block_data: impl Into<crate::block::data::BlockData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x_min);
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(y_min);
        args.push(val_2);
        sig += "I";
        let val_3 = jni::objects::JValueGen::Int(z_min);
        args.push(val_3);
        sig += "I";
        let val_4 = jni::objects::JValueGen::Int(x_max);
        args.push(val_4);
        sig += "I";
        let val_5 = jni::objects::JValueGen::Int(y_max);
        args.push(val_5);
        sig += "I";
        let val_6 = jni::objects::JValueGen::Int(z_max);
        args.push(val_6);
        sig += "Lorg/bukkit/block/data/BlockData;";
        let val_7 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(block_data.into().jni_object().clone())
        });
        args.push(val_7);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setRegion", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the type of the block at x, y, z.
    /// Getting blocks outside the chunk's bounds returns air.
    pub fn get_type(
        &self,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(III)Lorg/bukkit/Material;");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let val_3 = jni::objects::JValueGen::Int(z);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the type and data of the block at x, y, z.
    /// Getting blocks outside the chunk's bounds returns air.
    pub fn get_type_and_data(
        &self,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(III)Lorg/bukkit/material/MaterialData;");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let val_3 = jni::objects::JValueGen::Int(z);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTypeAndData",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the type and data of the block at x, y, z.
    /// Getting blocks outside the chunk's bounds returns air.
    pub fn get_block_data(
        &self,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(III)Lorg/bukkit/block/data/BlockData;");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let val_3 = jni::objects::JValueGen::Int(z);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Get the block data at x,y,z in the chunk data. Getting blocks outside the chunk's bounds returns 0.
    pub fn get_data(&self, x: i32, y: i32, z: i32) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("(III)B");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let val_3 = jni::objects::JValueGen::Int(z);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct LimitedRegion<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LimitedRegion<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LimitedRegion<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LimitedRegion from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/LimitedRegion")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LimitedRegion object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LimitedRegion<'mc> {
    /// Gets the buffer around the central chunk which is accessible.
    /// The returned value is in normal world coordinate scale.
    ///
    /// For example: If the method returns 16 you have a working area of 48x48.
    pub fn buffer(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBuffer", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Checks if the given coordinates are in the region.
    pub fn is_in_region(
        &self,
        x: i32,
        y: std::option::Option<i32>,
        z: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x);
        args.push(val_1);
        if let Some(a) = y {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = z {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isInRegion", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets a list of all tile entities in the limited region including the
    /// buffer zone.
    pub fn tile_entities(
        &self,
    ) -> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTileEntities", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::block::BlockState::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Gets the {@link Biome} at the given coordinates.
    pub fn get_biome(
        &self,
        x: i32,
        y: std::option::Option<i32>,
        z: std::option::Option<i32>,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x);
        args.push(val_1);
        if let Some(a) = y {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = z {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/block/Biome;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBiome", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Biome::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the {@link Biome} for the given block coordinates
    pub fn set_biome(
        &self,
        x: i32,
        y: i32,
        z: std::option::Option<i32>,
        biome: std::option::Option<impl Into<crate::block::Biome<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x);
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(y);
        args.push(val_2);
        if let Some(a) = z {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = biome {
            sig += "Lorg/bukkit/block/Biome;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setBiome", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the {@link BlockState} at the given coordinates.
    pub fn get_block_state(
        &self,
        x: i32,
        y: std::option::Option<i32>,
        z: std::option::Option<i32>,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x);
        args.push(val_1);
        if let Some(a) = y {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = z {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/block/BlockState;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockState", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the {@link BlockData} at the given coordinates.
    pub fn get_block_data(
        &self,
        x: i32,
        y: std::option::Option<i32>,
        z: std::option::Option<i32>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x);
        args.push(val_1);
        if let Some(a) = y {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = z {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/block/data/BlockData;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getBlockData", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the type of the block at the given coordinates.
    pub fn get_type(
        &self,
        x: i32,
        y: std::option::Option<i32>,
        z: std::option::Option<i32>,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x);
        args.push(val_1);
        if let Some(a) = y {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = z {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/Material;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::Material::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the {@link BlockData} at the given coordinates.
    pub fn set_block_data(
        &self,
        x: i32,
        y: i32,
        z: std::option::Option<i32>,
        block_data: std::option::Option<impl Into<crate::block::data::BlockData<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x);
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(y);
        args.push(val_2);
        if let Some(a) = z {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = block_data {
            sig += "Lorg/bukkit/block/data/BlockData;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setBlockData", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the {@link Material} at the given coordinates.
    pub fn set_type(
        &self,
        x: i32,
        y: i32,
        z: std::option::Option<i32>,
        material: std::option::Option<impl Into<crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x);
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(y);
        args.push(val_2);
        if let Some(a) = z {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = material {
            sig += "Lorg/bukkit/Material;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setType", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Creates a tree at the given {@link Location}
    ///
    /// The provided predicate gets called for every block which gets changed
    /// as a result of the tree generation. When the predicate gets called no
    /// modifications to the world are done yet. Which means, that calling
    /// {@link #getBlockState(Location)} in the predicate will return the state
    /// of the block before the generation.
    ///
    /// If the predicate returns {@code true} the block gets set in the world.
    /// If it returns {@code false} the block won't get set in the world.
    pub fn generate_tree(
        &self,
        location: impl Into<crate::Location<'mc>>,
        random: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        val_type: impl Into<crate::TreeType<'mc>>,
        state_predicate: std::option::Option<
            impl Into<blackboxmc_java::util::function::JavaPredicate<'mc>>,
        >,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/util/Random;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(random.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "Lorg/bukkit/TreeType;";
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_3);
        if let Some(a) = state_predicate {
            sig += "Ljava/util/function/Predicate;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "generateTree", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Creates a new entity at the given {@link Location}.
    pub fn spawn_entity(
        &self,
        loc: impl Into<crate::Location<'mc>>,
        val_type: impl Into<crate::entity::EntityType<'mc>>,
        randomize_data: std::option::Option<bool>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(loc.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Lorg/bukkit/entity/EntityType;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = randomize_data {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/entity/Entity;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "spawnEntity", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get a list of all entities in this RegionAccessor
    pub fn entities(&self) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEntities", sig.as_str(), vec![]);
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
    /// Get a list of all living entities in this RegionAccessor
    pub fn living_entities(
        &self,
    ) -> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLivingEntities",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::LivingEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Get a collection of all entities in this RegionAccessor matching the given
    /// class/interface
    pub fn get_entities_by_class(
        &self,
        cls: jni::objects::JClass<'mc>,
    ) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Class;)Ljava/util/Collection;");
        let val_1 = jni::objects::JValueGen::Object(cls.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntitiesByClass",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }
    /// Get a collection of all entities in this RegionAccessor matching any of the
    /// given classes/interfaces
    pub fn get_entities_by_classes(
        &self,
        classes: jni::objects::JClass<'mc>,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Class;)Ljava/util/Collection;");
        let val_1 = jni::objects::JValueGen::Object(classes.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntitiesByClasses",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Creates an entity of a specific class at the given {@link Location} but
    /// does not spawn it in the world.
    ///
    /// <b>Note:</b> The created entity keeps a reference to the world it was
    /// created in, care should be taken that the entity does not outlive the
    /// world instance as this will lead to memory leaks.
    pub fn create_entity(
        &self,
        location: impl Into<crate::Location<'mc>>,
        clazz: jni::objects::JClass<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;Ljava/lang/Class;)LT;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(clazz.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createEntity",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    /// Creates a new entity at the given {@link Location} with the supplied
    /// function run before the entity is added to the world.
    ///
    /// Note that when the function is run, the entity will not be actually in
    /// the world. Any operation involving such as teleporting the entity is undefined
    /// until after this function returns.
    /// The passed function however is run after the potential entity's spawn
    /// randomization and hence already allows access to the values of the mob,
    /// whether or not those were randomized, such as attributes or the entity
    /// equipment.
    pub fn spawn(
        &self,
        location: impl Into<crate::Location<'mc>>,
        clazz: jni::objects::JClass<'mc>,
        randomize_data: std::option::Option<bool>,
        function: std::option::Option<
            impl Into<blackboxmc_java::util::function::JavaConsumer<'mc>>,
        >,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/Class;";
        let val_2 = jni::objects::JValueGen::Object(clazz.into());
        args.push(val_2);
        if let Some(a) = randomize_data {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = function {
            sig += "Ljava/util/function/Consumer;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")LT;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "spawn", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    /// Gets the highest coordinate corresponding to the {@link HeightMap} at the
    /// given {@link Location}.
    pub fn get_highest_block_yat(
        &self,
        location: impl Into<crate::Location<'mc>>,
        height_map: std::option::Option<impl Into<crate::HeightMap<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/Location;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = height_map {
            sig += "Lorg/bukkit/HeightMap;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")I";
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHighestBlockYAt",
            sig.as_str(),
            args,
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Spawns a previously created entity in the world.
    ///
    /// The provided entity must not have already been spawned in a world.
    pub fn add_entity(
        &self,
        entity: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(LT;)LT;");
        let val_1 = jni::objects::JValueGen::Object(entity);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addEntity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::RegionAccessor<'mc>> for LimitedRegion<'mc> {
    fn into(self) -> crate::RegionAccessor<'mc> {
        crate::RegionAccessor::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LimitedRegion into crate::RegionAccessor")
    }
}
#[repr(C)]
pub struct BiomeProvider<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BiomeProvider<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BiomeProvider<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BiomeProvider from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/BiomeProvider")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BiomeProvider object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BiomeProvider<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "BiomeProvider", name, lib_name) }?;
        Self::from_raw(env, obj)
    }

    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::generator::BiomeProvider<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/generator/BiomeProvider");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::generator::BiomeProvider::from_raw(&jni, res)
    }
    /// Return the Biome which should be present at the provided location.
    ///
    /// Notes:
    ///
    /// This method <b>must</b> be completely thread safe and able to handle
    /// multiple concurrent callers.
    ///
    /// This method should only return biomes which are present in the list
    /// returned by {@link #getBiomes(WorldInfo)}
    ///
    /// This method should <b>never</b> return {@link Biome#CUSTOM}.
    /// Only this method is called if both this and
    /// {@link #getBiome(WorldInfo, int, int, int)} are overridden.
    pub fn get_biome(
        &self,
        world_info: impl Into<crate::generator::WorldInfo<'mc>>,
        x: i32,
        y: i32,
        z: i32,
        biome_parameter_point: impl Into<crate::generator::BiomeParameterPoint<'mc>>,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/generator/WorldInfo;IIILorg/bukkit/generator/BiomeParameterPoint;)Lorg/bukkit/block/Biome;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world_info.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Int(x);
        let val_3 = jni::objects::JValueGen::Int(y);
        let val_4 = jni::objects::JValueGen::Int(z);
        let val_5 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(biome_parameter_point.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBiome",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
                jni::objects::JValueGen::from(val_5),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Biome::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct WorldInfo<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for WorldInfo<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for WorldInfo<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate WorldInfo from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/WorldInfo")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a WorldInfo object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> WorldInfo<'mc> {
    /// Gets the unique name of this world
    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
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
    /// Gets the Unique ID of this world
    pub fn uid(&self) -> Result<blackboxmc_java::util::JavaUUID<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/UUID;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getUID", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaUUID::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the {@link World.Environment} type of this world
    pub fn environment(&self) -> Result<crate::WorldEnvironment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/World/Environment;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getEnvironment", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::WorldEnvironment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the Seed for this world.
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    /// Gets the minimum height of this world.
    ///
    /// If the min height is 0, there are only blocks from y=0.
    pub fn min_height(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Gets the maximum height of this world.
    ///
    /// If the max height is 100, there are only blocks from y=0 to y=99.
    pub fn max_height(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct ChunkGeneratorBiomeGrid<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ChunkGeneratorBiomeGrid<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ChunkGeneratorBiomeGrid<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate ChunkGeneratorBiomeGrid from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/generator/ChunkGenerator/BiomeGrid")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ChunkGeneratorBiomeGrid object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ChunkGeneratorBiomeGrid<'mc> {
    /// Get biome at x, z within chunk being generated
    pub fn get_biome(
        &self,
        x: i32,
        y: i32,
        z: std::option::Option<i32>,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x);
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(y);
        args.push(val_2);
        if let Some(a) = z {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")Lorg/bukkit/block/Biome;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBiome", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::Biome::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set biome at x, z within chunk being generated
    pub fn set_biome(
        &self,
        x: i32,
        y: i32,
        z: i32,
        bio: std::option::Option<impl Into<crate::block::Biome<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(x);
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(y);
        args.push(val_2);
        sig += "I";
        let val_3 = jni::objects::JValueGen::Int(z);
        args.push(val_3);
        if let Some(a) = bio {
            sig += "Lorg/bukkit/block/Biome;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "setBiome", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct BlockPopulator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BlockPopulator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BlockPopulator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BlockPopulator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/BlockPopulator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BlockPopulator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BlockPopulator<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "BlockPopulator", name, lib_name) }?;
        Self::from_raw(env, obj)
    }

    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::generator::BlockPopulator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/generator/BlockPopulator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::generator::BlockPopulator::from_raw(&jni, res)
    }
    /// Populates an area of blocks at or around the given chunk.
    ///
    /// Notes:
    ///
    /// This method should <b>never</b> attempt to get the Chunk at the passed
    /// coordinates, as doing so may cause an infinite loop
    ///
    /// This method should <b>never</b> modify a {@link LimitedRegion} at a later
    /// point of time.
    ///
    /// This method <b>must</b> be completely thread safe and able to handle
    /// multiple concurrent callers.
    ///
    /// No physics are applied, whether or not it is set to true in
    /// {@link org.bukkit.block.BlockState#update(boolean, boolean)}
    ///
    /// <b>Only</b> use the {@link org.bukkit.block.BlockState} returned by
    /// {@link LimitedRegion},
    /// <b>never</b> use methods from a {@link World} to modify the chunk.
    pub fn populate(
        &self,
        world_info: impl Into<crate::generator::WorldInfo<'mc>>,
        random: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        chunk_x: i32,
        chunk_z: std::option::Option<i32>,
        limited_region: std::option::Option<impl Into<crate::generator::LimitedRegion<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/generator/WorldInfo;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world_info.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/util/Random;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(random.into().jni_object().clone())
        });
        args.push(val_2);
        sig += "I";
        let val_3 = jni::objects::JValueGen::Int(chunk_x);
        args.push(val_3);
        if let Some(a) = chunk_z {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        if let Some(a) = limited_region {
            sig += "Lorg/bukkit/generator/LimitedRegion;";
            let val_5 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_5);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "populate", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct BiomeParameterPoint<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for BiomeParameterPoint<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for BiomeParameterPoint<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate BiomeParameterPoint from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/generator/BiomeParameterPoint")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BiomeParameterPoint object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> BiomeParameterPoint<'mc> {
    /// Gets the temperature of the biome at this point that is suggested by the
    /// NoiseGenerator.
    pub fn temperature(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getTemperature", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum temperature that is possible.
    pub fn max_temperature(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxTemperature",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the minimum temperature that is possible.
    pub fn min_temperature(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMinTemperature",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the humidity of the biome at this point that is suggested by the
    /// NoiseGenerator.
    pub fn humidity(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getHumidity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum humidity that is possible.
    pub fn max_humidity(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxHumidity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the minimum humidity that is possible.
    pub fn min_humidity(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinHumidity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the continentalness of the biome at this point that is suggested by
    /// the NoiseGenerator.
    pub fn continentalness(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getContinentalness",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum continentalness that is possible.
    pub fn max_continentalness(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMaxContinentalness",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the minimum continentalness that is possible.
    pub fn min_continentalness(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMinContinentalness",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the erosion of the biome at this point that is suggested by the
    /// NoiseGenerator.
    pub fn erosion(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getErosion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum erosion that is possible.
    pub fn max_erosion(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxErosion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the minimum erosion that is possible.
    pub fn min_erosion(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinErosion", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the depth of the biome at this point that is suggested by the
    /// NoiseGenerator.
    pub fn depth(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDepth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum depth that is possible.
    pub fn max_depth(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxDepth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the minimum depth that is possible.
    pub fn min_depth(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinDepth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the weirdness of the biome at this point that is suggested by the
    /// NoiseGenerator.
    pub fn weirdness(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWeirdness", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the maximum weirdness that is possible.
    pub fn max_weirdness(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxWeirdness", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Gets the minimum weirdness that is possible.
    pub fn min_weirdness(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinWeirdness", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub mod structure;
