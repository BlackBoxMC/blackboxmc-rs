#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Data for a Chunk.
///
/// This is a representation of an abstract class.
pub struct ChunkGeneratorChunkData<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ChunkGeneratorChunkData<'mc> {
    pub fn from_raw(
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
            env.validate_name(&obj, "org/bukkit/generator/ChunkGeneratorChunkData")?;
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
    /// Set a region of this chunk from xMin, yMin, zMin (inclusive) to xMax, yMax, zMax (exclusive) to material. Setting blocks outside the chunk's bounds does nothing.
    /// Set a region of this chunk from xMin, yMin, zMin (inclusive) to xMax, yMax, zMax (exclusive) to material. Setting blocks outside the chunk's bounds does nothing.
    /// Set a region of this chunk from xMin, yMin, zMin (inclusive) to xMax, yMax, zMax (exclusive) to material. Setting blocks outside the chunk's bounds does nothing.
    pub fn set_region_with_int(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = jni::objects::JValueGen::Int(arg4.into());
        let val_6 = jni::objects::JValueGen::Int(arg5.into());
        let val_7 =
            unsafe { jni::objects::JObject::from_raw(arg6.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRegion",
            "(IIIIIILorg/bukkit/Material;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
                jni::objects::JValueGen::from(&val_7),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Uses magic values
    /// </div>
    /// Uses magic values
    ///
    /// Get the block data at x,y,z in the chunk data. Getting blocks outside the chunk's bounds returns 0.
    pub fn get_data(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getData",
            "(III)B",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b().unwrap())
    }
    /// Get the type and data of the block at x, y, z. Getting blocks outside the chunk's bounds returns air.
    pub fn get_block_data(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "(III)Lorg/bukkit/block/data/BlockData;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the biome at x, y, z within chunk being generated
    pub fn get_biome(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBiome",
            "(III)Lorg/bukkit/block/Biome;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::Biome::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::Biome::from_string(variant_str).unwrap(),
        )
    }
    /// Get the type and data of the block at x, y, z. Getting blocks outside the chunk's bounds returns air.
    pub fn get_type_and_data(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::material::MaterialData<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTypeAndData",
            "(III)Lorg/bukkit/material/MaterialData;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::material::MaterialData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the minimum height for this ChunkData.
    /// <p>It is not guaranteed that this method will return the same value as <a href="WorldInfo.html#getMinHeight()"><code>WorldInfo.getMinHeight()</code></a>.</p>
    /// <p>Setting blocks below this height will do nothing.</p>
    pub fn min_height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinHeight", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Get the maximum height for this ChunkData.
    /// <p>It is not guaranteed that this method will return the same value as <a href="WorldInfo.html#getMaxHeight()"><code>WorldInfo.getMaxHeight()</code></a>.</p>
    /// <p>Setting blocks at or above this height will do nothing.</p>
    pub fn max_height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxHeight", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Set the block at x,y,z in the chunk data to material. Note: setting blocks outside the chunk's bounds does nothing.
    /// Set the block at x,y,z in the chunk data to material. Setting blocks outside the chunk's bounds does nothing.
    /// Set the block at x,y,z in the chunk data to material. Setting blocks outside the chunk's bounds does nothing.
    pub fn set_block_with_int(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: std::option::Option<impl Into<&'mc crate::block::data::BlockData<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 =
            unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlock",
            "(IIILorg/bukkit/block/data/BlockData;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the type of the block at x, y, z. Getting blocks outside the chunk's bounds returns air.
    /// Get the type and data of the block at x, y, z. Getting blocks outside the chunk's bounds returns air.
    pub fn get_type(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "(III)Lorg/bukkit/Material;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }
}
impl<'mc> JNIRaw<'mc> for ChunkGeneratorChunkData<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Class for providing biomes.
pub struct BiomeProvider<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BiomeProvider<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
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
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::generator::BiomeProvider<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/generator/BiomeProvider")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::generator::BiomeProvider::from_raw(&jni, res)
    }
    /// Return the Biome which should be present at the provided location.
    /// <p>Notes:</p>
    /// <p>This method <b>must</b> be completely thread safe and able to handle multiple concurrent callers.</p>
    /// <p>This method should only return biomes which are present in the list returned by <a href="#getBiomes(org.bukkit.generator.WorldInfo)"><code>getBiomes(WorldInfo)</code></a></p>
    /// <p>This method should <b>never</b> return <a href="../block/Biome.html#CUSTOM"><code>Biome.CUSTOM</code></a>.</p>
    /// Return the Biome which should be present at the provided location.
    /// <p>Notes:</p>
    /// <p>This method <b>must</b> be completely thread safe and able to handle multiple concurrent callers.</p>
    /// <p>This method should only return biomes which are present in the list returned by <a href="#getBiomes(org.bukkit.generator.WorldInfo)"><code>getBiomes(WorldInfo)</code></a></p>
    /// <p>This method should <b>never</b> return <a href="../block/Biome.html#CUSTOM"><code>Biome.CUSTOM</code></a>. Only this method is called if both this and <a href="#getBiome(org.bukkit.generator.WorldInfo,int,int,int)"><code>getBiome(WorldInfo, int, int, int)</code></a> are overridden.</p>
    /// Returns a list with every biome the <a title="class in org.bukkit.generator" href="BiomeProvider.html"><code>BiomeProvider</code></a> will use for the given world.
    /// <p>Notes:</p>
    /// <p>This method only gets called once, when the world is loaded. Returning another list or modifying the values from the initial returned list later one, are not respected.</p>
    /// <p>This method should <b>never</b> return a list which contains <a href="../block/Biome.html#CUSTOM"><code>Biome.CUSTOM</code></a>.</p>
    pub fn get_biome_with_world_info(
        &mut self,
        arg0: impl Into<&'mc crate::generator::WorldInfo<'mc>>,
        arg1: i32,
        arg2: i32,
        arg3: std::option::Option<i32>,
        arg4: std::option::Option<impl Into<&'mc crate::generator::BiomeParameterPoint<'mc>>>,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let val_5 =
            unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"getBiome","(Lorg/bukkit/generator/WorldInfo;IIILorg/bukkit/generator/BiomeParameterPoint;)Lorg/bukkit/block/Biome;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::Biome::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::Biome::from_string(variant_str).unwrap(),
        )
    }
    /// Returns a list with every biome the <a title="class in org.bukkit.generator" href="BiomeProvider.html"><code>BiomeProvider</code></a> will use for the given world.
    /// <p>Notes:</p>
    /// <p>This method only gets called once, when the world is loaded. Returning another list or modifying the values from the initial returned list later one, are not respected.</p>
    /// <p>This method should <b>never</b> return a list which contains <a href="../block/Biome.html#CUSTOM"><code>Biome.CUSTOM</code></a>.</p>
    pub fn get_biomes(
        &mut self,
        arg0: impl Into<&'mc crate::generator::WorldInfo<'mc>>,
    ) -> Result<Vec<crate::block::Biome<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBiomes",
            "(Lorg/bukkit/generator/WorldInfo;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            let variant =
                self.0
                    .call_method(list.get(i)?, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            new_vec.push(crate::block::Biome::from_raw(
                &self.0,
                obj,
                crate::block::Biome::from_string(variant_str).unwrap(),
            )?);
        }
        Ok(new_vec)
    }

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Interface to biome section for chunk to be generated: initialized with default values for world type and seed.
/// <p>Custom generator is free to access and tailor values during generateBlockSections() or generateExtBlockSections().</p>
///
/// This is a representation of an abstract class.
pub struct ChunkGeneratorBiomeGrid<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ChunkGeneratorBiomeGrid<'mc> {
    pub fn from_raw(
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
            env.validate_name(&obj, "org/bukkit/generator/ChunkGeneratorBiomeGrid")?;
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
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// biomes are now 3-dimensional
    /// </div>
    /// biomes are now 3-dimensional
    ///
    /// Get biome at x, z within chunk being generated
    /// <span class="deprecated-label">Deprecated.</span>
    /// Get biome at x, z within chunk being generated
    pub fn get_biome_with_int(
        &mut self,
        arg0: i32,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBiome",
            "(III)Lorg/bukkit/block/Biome;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::Biome::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::Biome::from_string(variant_str).unwrap(),
        )
    }
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// biomes are now 3-dimensional
    /// </div>
    /// biomes are now 3-dimensional
    ///
    /// Set biome at x, z within chunk being generated
    /// <span class="deprecated-label">Deprecated.</span>
    /// Set biome at x, z within chunk being generated
    pub fn set_biome_with_int(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<impl Into<&'mc crate::block::Biome<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 =
            unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBiome",
            "(IIILorg/bukkit/block/Biome;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for ChunkGeneratorBiomeGrid<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents the biome noise parameters which may be passed to a world generator.
///
/// This is a representation of an abstract class.
pub struct BiomeParameterPoint<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> BiomeParameterPoint<'mc> {
    pub fn from_raw(
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
    /// Gets the minimum erosion that is possible.
    pub fn min_erosion(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinErosion", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the temperature of the biome at this point that is suggested by the NoiseGenerator.
    pub fn temperature(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTemperature", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the maximum temperature that is possible.
    pub fn max_temperature(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxTemperature", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the minimum temperature that is possible.
    pub fn min_temperature(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinTemperature", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the humidity of the biome at this point that is suggested by the NoiseGenerator.
    pub fn humidity(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHumidity", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the maximum humidity that is possible.
    pub fn max_humidity(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxHumidity", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the minimum humidity that is possible.
    pub fn min_humidity(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinHumidity", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the continentalness of the biome at this point that is suggested by the NoiseGenerator.
    pub fn continentalness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getContinentalness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the maximum continentalness that is possible.
    pub fn max_continentalness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMaxContinentalness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the minimum continentalness that is possible.
    pub fn min_continentalness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMinContinentalness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the erosion of the biome at this point that is suggested by the NoiseGenerator.
    pub fn erosion(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getErosion", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the maximum erosion that is possible.
    pub fn max_erosion(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxErosion", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the depth of the biome at this point that is suggested by the NoiseGenerator.
    pub fn depth(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDepth", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the maximum depth that is possible.
    pub fn max_depth(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxDepth", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the minimum depth that is possible.
    pub fn min_depth(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinDepth", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the weirdness of the biome at this point that is suggested by the NoiseGenerator.
    pub fn weirdness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWeirdness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the maximum weirdness that is possible.
    pub fn max_weirdness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxWeirdness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    /// Gets the minimum weirdness that is possible.
    pub fn min_weirdness(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinWeirdness", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for BiomeParameterPoint<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// A block populator is responsible for generating a small area of blocks.
/// <p>For example, generating glowstone inside the nether or generating dungeons full of treasure</p>
/// <p>A BlockPopulator can be used in combination with a custom <a title="class in org.bukkit.generator" href="ChunkGenerator.html"><code>ChunkGenerator</code></a> by returning it in the method <a href="ChunkGenerator.html#getDefaultPopulators(org.bukkit.World)"><code>ChunkGenerator.getDefaultPopulators(World)</code></a> or by adding it manually to the worlds populator list returned by <a href="../World.html#getPopulators()"><code>World.getPopulators()</code></a>.</p>
/// <p>When adding a BlockPopulator manually to a world it is recommended to do so during the <a href="../event/world/WorldInitEvent.html" title="class in org.bukkit.event.world"><code>WorldInitEvent</code></a>.</p>
pub struct BlockPopulator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BlockPopulator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
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
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::generator::BlockPopulator<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/generator/BlockPopulator")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::generator::BlockPopulator::from_raw(&jni, res)
    }
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Use <a href="#populate(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.LimitedRegion)"><code>populate(WorldInfo, Random, int, int, LimitedRegion)</code></a>
    /// </div>
    /// Use <a href="#populate(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.LimitedRegion)"><code>populate(WorldInfo, Random, int, int, LimitedRegion)</code></a>
    ///
    /// Populates an area of blocks at or around the given chunk.
    /// <p>The chunks on each side of the specified chunk must already exist; that is, there must be one north, east, south and west of the specified chunk. The "corner" chunks may not exist, in which scenario the populator should record any changes required for those chunks and perform the changes when they are ready.</p>
    /// Populates an area of blocks at or around the given chunk.
    /// <p>Notes:</p>
    /// <p>This method should <b>never</b> attempt to get the Chunk at the passed coordinates, as doing so may cause an infinite loop</p>
    /// <p>This method should <b>never</b> modify a <a title="interface in org.bukkit.generator" href="LimitedRegion.html"><code>LimitedRegion</code></a> at a later point of time.</p>
    /// <p>This method <b>must</b> be completely thread safe and able to handle multiple concurrent callers.</p>
    /// <p>No physics are applied, whether or not it is set to true in <a href="../block/BlockState.html#update(boolean,boolean)"><code>BlockState.update(boolean, boolean)</code></a></p>
    /// <p><b>Only</b> use the <a title="interface in org.bukkit.block" href="../block/BlockState.html"><code>BlockState</code></a> returned by <a href="LimitedRegion.html" title="interface in org.bukkit.generator"><code>LimitedRegion</code></a>, <b>never</b> use methods from a <a title="interface in org.bukkit" href="../World.html"><code>World</code></a> to modify the chunk.</p>
    pub fn populate_with_world(
        &mut self,
        arg0: impl Into<&'mc crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
        arg4: std::option::Option<impl Into<&'mc crate::generator::LimitedRegion<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let val_5 =
            unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"populate","(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/LimitedRegion;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// A limited region is used in world generation for features which are going over a chunk. For example, trees or ores. Use <a href="#getBuffer()"><code>getBuffer()</code></a> to know how much you can go beyond the central chunk. The buffer zone may or may not be already populated. The coordinates are <b>absolute</b> from the world origin.
///
/// This is a representation of an abstract class.
pub struct LimitedRegion<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LimitedRegion<'mc> {
    pub fn from_raw(
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
    /// Gets the buffer around the central chunk which is accessible. The returned value is in normal world coordinate scale.
    /// <p>For example: If the method returns 16 you have a working area of 48x48.</p>
    pub fn buffer(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getBuffer", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets a list of all tile entities in the limited region including the buffer zone.
    pub fn tile_entities(
        &mut self,
    ) -> Result<Vec<crate::block::BlockState<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getTileEntities",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::block::BlockState::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Checks if the given <a title="class in org.bukkit" href="../Location.html"><code>Location</code></a> is in the region.
    /// Checks if the given coordinates are in the region.
    pub fn is_in_region_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isInRegion",
            "(III)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_type_with_location(
        &mut self,
        arg0: i32,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<impl Into<&'mc crate::Material<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 =
            unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(IIILorg/bukkit/Material;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_block_data_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::block::data::BlockData<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockData",
            "(III)Lorg/bukkit/block/data/BlockData;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::data::BlockData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_block_data_with_location(
        &mut self,
        arg0: i32,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<impl Into<&'mc crate::block::data::BlockData<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 =
            unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBlockData",
            "(IIILorg/bukkit/block/data/BlockData;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_highest_block_yat_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHighestBlockYAt",
            "(II)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn get_highest_block_yat_with_int(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: std::option::Option<impl Into<&'mc crate::HeightMap<'mc>>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getHighestBlockYAt",
            "(IILorg/bukkit/HeightMap;)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn get_biome_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::block::Biome<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBiome",
            "(III)Lorg/bukkit/block/Biome;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::Biome::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::Biome::from_string(variant_str).unwrap(),
        )
    }

    pub fn set_biome_with_location(
        &mut self,
        arg0: i32,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<impl Into<&'mc crate::block::Biome<'mc>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 =
            unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setBiome",
            "(IIILorg/bukkit/block/Biome;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_block_state_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::block::BlockState<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBlockState",
            "(III)Lorg/bukkit/block/BlockState;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::block::BlockState::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn spawn_entity_with_location(
        &mut self,
        arg0: impl Into<&'mc crate::Location<'mc>>,
        arg1: std::option::Option<impl Into<&'mc crate::entity::EntityType<'mc>>>,
        arg2: std::option::Option<bool>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        // 1
        let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "spawnEntity",
            "(Lorg/bukkit/Location;Lorg/bukkit/entity/EntityType;Z)Lorg/bukkit/entity/Entity;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn entities(
        &mut self,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntities",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn living_entities(
        &mut self,
    ) -> Result<Vec<crate::entity::LivingEntity<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLivingEntities",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::entity::LivingEntity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn get_entities_by_class(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntitiesByClass",
            "(Ljava/lang/Class;)Ljava/util/Collection;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }

    pub fn get_entities_by_classes(
        &mut self,
        arg0: Vec<jni::objects::JClass<'mc>>,
    ) -> Result<Vec<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEntitiesByClasses",
            "(Ljava/lang/Class;)Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::entity::Entity::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn spawn_with_location(
        &mut self,
        arg0: impl Into<&'mc crate::Location<'mc>>,
        arg1: std::option::Option<jni::objects::JClass<'mc>>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<impl Into<&'mc crate::util::Consumer<'mc>>>,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = arg1.unwrap();
        // 1
        let val_3 = jni::objects::JValueGen::Bool(arg2.unwrap().into());
        let val_4 =
            unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"spawn","(Lorg/bukkit/Location;Ljava/lang/Class;ZLorg/bukkit/util/Consumer;)Lorg/bukkit/entity/Entity;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type_with_location(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::Material<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "(III)Lorg/bukkit/Material;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant =
            self.jni_ref()
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::Material::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::Material::from_string(variant_str).unwrap(),
        )
    }
}
impl<'mc> JNIRaw<'mc> for LimitedRegion<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::RegionAccessor<'mc>> for LimitedRegion<'mc> {
    fn into(self) -> crate::RegionAccessor<'mc> {
        crate::RegionAccessor::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Holds various information of a World
///
/// This is a representation of an abstract class.
pub struct WorldInfo<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> WorldInfo<'mc> {
    pub fn from_raw(
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
    /// Gets the Seed for this world.
    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    /// Gets the <a title="enum in org.bukkit" href="../World.Environment.html"><code>World.Environment</code></a> type of this world
    pub fn environment(
        &mut self,
    ) -> Result<crate::WorldEnvironment<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEnvironment",
            "()Lorg/bukkit/World$Environment;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::WorldEnvironment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the minimum height of this world.
    /// <p>If the min height is 0, there are only blocks from y=0.</p>
    pub fn min_height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMinHeight", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the maximum height of this world.
    /// <p>If the max height is 100, there are only blocks from y=0 to y=99.</p>
    pub fn max_height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getMaxHeight", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// Gets the unique name of this world
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
}
impl<'mc> JNIRaw<'mc> for WorldInfo<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// A chunk generator is responsible for the initial shaping of an entire chunk. For example, the nether chunk generator should shape netherrack and soulsand. A chunk is generated in multiple steps, those steps are always in the same order. Between those steps however an unlimited time may pass. This means, a chunk may generated until the surface step and continue with the bedrock step after one or multiple server restarts or even after multiple Minecraft versions. The order of generation is as follows
/// <ol>
/// <li><a href="#generateNoise(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateNoise(WorldInfo, Random, int, int, ChunkData)</code></a></li>
/// <li><a href="#generateSurface(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateSurface(WorldInfo, Random, int, int, ChunkData)</code></a></li>
/// <li><a href="#generateBedrock(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateBedrock(WorldInfo, Random, int, int, ChunkData)</code></a></li>
/// <li><a href="#generateCaves(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateCaves(WorldInfo, Random, int, int, ChunkData)</code></a></li>
/// </ol> Every method listed above as well as <a href="#getBaseHeight(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.HeightMap)"><code>getBaseHeight(WorldInfo, Random, int, int, HeightMap)</code></a> <b>must</b> be completely thread safe and able to handle multiple concurrent callers. Some aspects of world generation can be delegated to the Vanilla generator. The following methods can be overridden to enable this:
/// <ul>
/// <li><a href="#shouldGenerateNoise()"><code>shouldGenerateNoise()</code></a> or <a href="#shouldGenerateNoise(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateNoise(WorldInfo, Random, int, int)</code></a></li>
/// <li><a href="#shouldGenerateSurface()"><code>shouldGenerateSurface()</code></a> or <a href="#shouldGenerateSurface(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateSurface(WorldInfo, Random, int, int)</code></a></li>
/// <li><a href="#shouldGenerateCaves()"><code>shouldGenerateCaves()</code></a> or <a href="#shouldGenerateCaves(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateCaves(WorldInfo, Random, int, int)</code></a></li>
/// <li><a href="#shouldGenerateDecorations()"><code>shouldGenerateDecorations()</code></a> or <a href="#shouldGenerateDecorations(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateDecorations(WorldInfo, Random, int, int)</code></a></li>
/// <li><a href="#shouldGenerateMobs()"><code>shouldGenerateMobs()</code></a> or <a href="#shouldGenerateMobs(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateMobs(WorldInfo, Random, int, int)</code></a></li>
/// <li><a href="#shouldGenerateStructures()"><code>shouldGenerateStructures()</code></a> or <a href="#shouldGenerateStructures(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateStructures(WorldInfo, Random, int, int)</code></a></li>
/// </ul>
pub struct ChunkGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ChunkGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
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
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::generator::ChunkGenerator<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/generator/ChunkGenerator")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::generator::ChunkGenerator::from_raw(&jni, res)
    }
    /// Shapes the Chunk noise for the given coordinates.
    /// <p>Notes:</p>
    /// <p>This method should <b>never</b> attempt to get the Chunk at the passed coordinates, as doing so may cause an infinite loop.</p>
    /// <p>This method should <b>never</b> modify the <a href="ChunkGenerator.ChunkData.html" title="interface in org.bukkit.generator"><code>ChunkGenerator.ChunkData</code></a> at a later point of time.</p>
    /// <p>The Y-coordinate range should <b>never</b> be hardcoded, to get the Y-coordinate range use the methods <a href="ChunkGenerator.ChunkData.html#getMinHeight()"><code>ChunkGenerator.ChunkData.getMinHeight()</code></a> and <a href="ChunkGenerator.ChunkData.html#getMaxHeight()"><code>ChunkGenerator.ChunkData.getMaxHeight()</code></a>.</p>
    /// <p>If <a href="#shouldGenerateNoise()"><code>shouldGenerateNoise()</code></a> is set to true, the given <a title="interface in org.bukkit.generator" href="ChunkGenerator.ChunkData.html"><code>ChunkGenerator.ChunkData</code></a> contains already the Vanilla noise generation.</p>
    pub fn generate_noise(
        &mut self,
        arg0: impl Into<&'mc crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<&'mc crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"generateNoise","(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator$ChunkData;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Shapes the Chunk surface for the given coordinates.
    /// <p>Notes:</p>
    /// <p>This method should <b>never</b> attempt to get the Chunk at the passed coordinates, as doing so may cause an infinite loop.</p>
    /// <p>This method should <b>never</b> modify the <a title="interface in org.bukkit.generator" href="ChunkGenerator.ChunkData.html"><code>ChunkGenerator.ChunkData</code></a> at a later point of time.</p>
    /// <p>The Y-coordinate range should <b>never</b> be hardcoded, to get the Y-coordinate range use the methods <a href="ChunkGenerator.ChunkData.html#getMinHeight()"><code>ChunkGenerator.ChunkData.getMinHeight()</code></a> and <a href="ChunkGenerator.ChunkData.html#getMaxHeight()"><code>ChunkGenerator.ChunkData.getMaxHeight()</code></a>.</p>
    /// <p>If <a href="#shouldGenerateSurface()"><code>shouldGenerateSurface()</code></a> is set to true, the given <a href="ChunkGenerator.ChunkData.html" title="interface in org.bukkit.generator"><code>ChunkGenerator.ChunkData</code></a> contains already the Vanilla surface generation.</p>
    pub fn generate_surface(
        &mut self,
        arg0: impl Into<&'mc crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<&'mc crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"generateSurface","(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator$ChunkData;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Shapes the Chunk bedrock layer for the given coordinates.
    /// <p>Notes:</p>
    /// <p>This method should <b>never</b> attempt to get the Chunk at the passed coordinates, as doing so may cause an infinite loop.</p>
    /// <p>This method should <b>never</b> modify the <a title="interface in org.bukkit.generator" href="ChunkGenerator.ChunkData.html"><code>ChunkGenerator.ChunkData</code></a> at a later point of time.</p>
    /// <p>The Y-coordinate range should <b>never</b> be hardcoded, to get the Y-coordinate range use the methods <a href="ChunkGenerator.ChunkData.html#getMinHeight()"><code>ChunkGenerator.ChunkData.getMinHeight()</code></a> and <a href="ChunkGenerator.ChunkData.html#getMaxHeight()"><code>ChunkGenerator.ChunkData.getMaxHeight()</code></a>.</p>
    /// <p></p>
    pub fn generate_bedrock(
        &mut self,
        arg0: impl Into<&'mc crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<&'mc crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"generateBedrock","(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator$ChunkData;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Shapes the Chunk caves for the given coordinates.
    /// <p>Notes:</p>
    /// <p>This method should <b>never</b> attempt to get the Chunk at the passed coordinates, as doing so may cause an infinite loop.</p>
    /// <p>This method should <b>never</b> modify the <a title="interface in org.bukkit.generator" href="ChunkGenerator.ChunkData.html"><code>ChunkGenerator.ChunkData</code></a> at a later point of time.</p>
    /// <p>The Y-coordinate range should <b>never</b> be hardcoded, to get the Y-coordinate range use the methods <a href="ChunkGenerator.ChunkData.html#getMinHeight()"><code>ChunkGenerator.ChunkData.getMinHeight()</code></a> and <a href="ChunkGenerator.ChunkData.html#getMaxHeight()"><code>ChunkGenerator.ChunkData.getMaxHeight()</code></a>.</p>
    /// <p>If <a href="#shouldGenerateCaves()"><code>shouldGenerateCaves()</code></a> is set to true, the given <a href="ChunkGenerator.ChunkData.html" title="interface in org.bukkit.generator"><code>ChunkGenerator.ChunkData</code></a> contains already the Vanilla cave generation.</p>
    pub fn generate_caves(
        &mut self,
        arg0: impl Into<&'mc crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<&'mc crate::generator::ChunkGeneratorChunkData<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"generateCaves","(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator$ChunkData;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets called when no <a href="BiomeProvider.html" title="class in org.bukkit.generator"><code>BiomeProvider</code></a> is set in <a href="../WorldCreator.html" title="class in org.bukkit"><code>WorldCreator</code></a> or via the server configuration files. It is therefore possible that one plugin can provide the Biomes and another one the generation.
    /// <p>Notes:</p>
    /// <p>If <code>null</code> is returned, than Vanilla biomes are used.</p>
    /// <p>This method only gets called once when the world is loaded. Returning another <a title="class in org.bukkit.generator" href="BiomeProvider.html"><code>BiomeProvider</code></a> later one is not respected.</p>
    pub fn get_default_biome_provider(
        &mut self,
        arg0: impl Into<&'mc crate::generator::WorldInfo<'mc>>,
    ) -> Result<crate::generator::BiomeProvider<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultBiomeProvider",
            "(Lorg/bukkit/generator/WorldInfo;)Lorg/bukkit/generator/BiomeProvider;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::BiomeProvider::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// This method is similar to <a href="../World.html#getHighestBlockAt(int,int,org.bukkit.HeightMap)"><code>World.getHighestBlockAt(int, int, HeightMap)</code></a>. With the difference being, that the highest y coordinate should be the block before any surface, bedrock, caves or decoration is applied. Or in other words the highest block when only the noise is present at the chunk.
    /// <p>Notes:</p>
    /// <p>When this method is not overridden, the Vanilla base height is used.</p>
    /// <p>This method should <b>never</b> attempt to get the Chunk at the passed coordinates, or use the method <a href="../World.html#getHighestBlockAt(int,int,org.bukkit.HeightMap)"><code>World.getHighestBlockAt(int, int, HeightMap)</code></a>, as doing so may cause an infinite loop.</p>
    pub fn get_base_height(
        &mut self,
        arg0: impl Into<&'mc crate::generator::WorldInfo<'mc>>,
        arg1: impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<&'mc crate::HeightMap<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBaseHeight",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;IILorg/bukkit/HeightMap;)I",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// The generation is now split up and the new methods should be used, see <a href="ChunkGenerator.html" title="class in org.bukkit.generator"><code>ChunkGenerator</code></a>
    /// </div>
    /// The generation is now split up and the new methods should be used, see <a href="ChunkGenerator.html" title="class in org.bukkit.generator"><code>ChunkGenerator</code></a>
    ///
    /// Shapes the chunk for the given coordinates. This method must return a ChunkData.
    /// <p>Notes:</p>
    /// <p>This method should <b>never</b> attempt to get the Chunk at the passed coordinates, as doing so may cause an infinite loop</p>
    /// <p>This method should <b>never</b> modify a ChunkData after it has been returned.</p>
    /// <p>This method <b>must</b> return a ChunkData returned by <a href="#createChunkData(org.bukkit.World)"><code>createChunkData(org.bukkit.World)</code></a></p>
    pub fn generate_chunk_data(
        &mut self,
        arg0: impl Into<&'mc crate::World<'mc>>,
        arg1: impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>,
        arg2: i32,
        arg3: i32,
        arg4: impl Into<&'mc crate::generator::ChunkGeneratorBiomeGrid<'mc>>,
    ) -> Result<crate::generator::ChunkGeneratorChunkData<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = unsafe { jni::objects::JObject::from_raw(arg4.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"generateChunkData","(Lorg/bukkit/World;Ljava/util/Random;IILorg/bukkit/generator/ChunkGenerator$BiomeGrid;)Lorg/bukkit/generator/ChunkGenerator$ChunkData;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::ChunkGeneratorChunkData::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Tests if the specified location is valid for a natural spawn position
    pub fn can_spawn(
        &mut self,
        arg0: impl Into<&'mc crate::World<'mc>>,
        arg1: i32,
        arg2: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Int(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "canSpawn",
            "(Lorg/bukkit/World;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets a list of default <a title="class in org.bukkit.generator" href="BlockPopulator.html"><code>BlockPopulator</code></a>s to apply to a given world
    pub fn get_default_populators(
        &mut self,
        arg0: impl Into<&'mc crate::World<'mc>>,
    ) -> Result<Vec<crate::generator::BlockPopulator<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultPopulators",
            "(Lorg/bukkit/World;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::generator::BlockPopulator::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    /// Gets a fixed spawn location to use for a given world.
    /// <p>A null value is returned if a world should not use a fixed spawn point, and will instead attempt to find one randomly.</p>
    pub fn get_fixed_spawn_location(
        &mut self,
        arg0: impl Into<&'mc crate::World<'mc>>,
        arg1: impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>,
    ) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFixedSpawnLocation",
            "(Lorg/bukkit/World;Ljava/util/Random;)Lorg/bukkit/Location;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// the chunk generation code should be thread safe
    /// </div>
    /// the chunk generation code should be thread safe
    ///
    /// Gets if this ChunkGenerator is parallel capable. See <a href="ChunkGenerator.html" title="class in org.bukkit.generator"><code>ChunkGenerator</code></a> for more information.
    pub fn is_parallel_capable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isParallelCapable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets if the server should generate Vanilla noise.
    /// <p>The Vanilla noise is generated <b>before</b> <a href="#generateNoise(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateNoise(WorldInfo, Random, int, int, ChunkData)</code></a> is called.</p>
    /// <p>This is method is not called (and has therefore no effect), if <a href="#shouldGenerateNoise(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateNoise(WorldInfo, Random, int, int)</code></a> is overridden.</p>
    /// Gets if the server should generate Vanilla noise.
    /// <p>The Vanilla noise is generated <b>before</b> <a href="#generateNoise(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateNoise(WorldInfo, Random, int, int, ChunkData)</code></a> is called.</p>
    /// <p>Only this method is called if both this and <a href="#shouldGenerateNoise()"><code>shouldGenerateNoise()</code></a> are overridden.</p>
    pub fn should_generate_noise(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateNoise",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets if the server should generate Vanilla surface.
    /// <p>The Vanilla surface is generated <b>before</b> <a href="#generateSurface(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateSurface(WorldInfo, Random, int, int, ChunkData)</code></a> is called.</p>
    /// <p>This is method is not called (and has therefore no effect), if <a href="#shouldGenerateSurface(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateSurface(WorldInfo, Random, int, int)</code></a> is overridden.</p>
    /// Gets if the server should generate Vanilla surface.
    /// <p>The Vanilla surface is generated <b>before</b> <a href="#generateSurface(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateSurface(WorldInfo, Random, int, int, ChunkData)</code></a> is called.</p>
    /// <p>Only this method is called if both this and <a href="#shouldGenerateSurface()"><code>shouldGenerateSurface()</code></a> are overridden.</p>
    pub fn should_generate_surface(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateSurface",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// has no effect, bedrock generation is part of the surface step, see <a href="#shouldGenerateSurface()"><code>shouldGenerateSurface()</code></a>
    /// </div>
    /// has no effect, bedrock generation is part of the surface step, see <a href="#shouldGenerateSurface()"><code>shouldGenerateSurface()</code></a>
    ///
    /// Gets if the server should generate Vanilla bedrock.
    /// <p>The Vanilla bedrock is generated <b>before</b> <a href="#generateBedrock(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateBedrock(WorldInfo, Random, int, int, ChunkData)</code></a> is called.</p>
    pub fn should_generate_bedrock(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "shouldGenerateBedrock", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets if the server should generate Vanilla caves.
    /// <p>The Vanilla caves are generated <b>before</b> <a href="#generateCaves(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateCaves(WorldInfo, Random, int, int, ChunkData)</code></a> is called.</p>
    /// <p>This is method is not called (and has therefore no effect), if <a href="#shouldGenerateCaves(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateCaves(WorldInfo, Random, int, int)</code></a> is overridden.</p>
    /// Gets if the server should generate Vanilla caves.
    /// <p>The Vanilla caves are generated <b>before</b> <a href="#generateCaves(org.bukkit.generator.WorldInfo,java.util.Random,int,int,org.bukkit.generator.ChunkGenerator.ChunkData)"><code>generateCaves(WorldInfo, Random, int, int, ChunkData)</code></a> is called.</p>
    /// <p>Only this method is called if both this and <a href="#shouldGenerateCaves()"><code>shouldGenerateCaves()</code></a> are overridden.</p>
    pub fn should_generate_caves(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateCaves",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets if the server should generate Vanilla decorations after this ChunkGenerator.
    /// <p>The Vanilla decoration are generated <b>before</b> any <a href="BlockPopulator.html" title="class in org.bukkit.generator"><code>BlockPopulator</code></a> are called.</p>
    /// <p>This is method is not called (and has therefore no effect), if <a href="#shouldGenerateDecorations(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateDecorations(WorldInfo, Random, int, int)</code></a> is overridden.</p>
    /// Gets if the server should generate Vanilla decorations after this ChunkGenerator.
    /// <p>The Vanilla decoration are generated <b>before</b> any <a title="class in org.bukkit.generator" href="BlockPopulator.html"><code>BlockPopulator</code></a> are called.</p>
    /// <p>Only this method is called if both this and <a href="#shouldGenerateDecorations()"><code>shouldGenerateDecorations()</code></a> are overridden.</p>
    pub fn should_generate_decorations(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateDecorations",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets if the server should generate Vanilla mobs after this ChunkGenerator.
    /// <p>This is method is not called (and has therefore no effect), if <a href="#shouldGenerateMobs(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateMobs(WorldInfo, Random, int, int)</code></a> is overridden.</p>
    /// Gets if the server should generate Vanilla mobs after this ChunkGenerator.
    /// <p>Only this method is called if both this and <a href="#shouldGenerateMobs()"><code>shouldGenerateMobs()</code></a> are overridden.</p>
    pub fn should_generate_mobs(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateMobs",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets if the server should generate Vanilla structures after this ChunkGenerator.
    /// <p>This is method is not called (and has therefore no effect), if <a href="#shouldGenerateStructures(org.bukkit.generator.WorldInfo,java.util.Random,int,int)"><code>shouldGenerateStructures(WorldInfo, Random, int, int)</code></a> is overridden.</p>
    /// Gets if the server should generate Vanilla structures after this ChunkGenerator.
    /// <p>Only this method is called if both this and <a href="#shouldGenerateStructures()"><code>shouldGenerateStructures()</code></a> are overridden.</p>
    pub fn should_generate_structures(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::generator::WorldInfo<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let val_3 = jni::objects::JValueGen::Int(arg2.unwrap().into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "shouldGenerateStructures",
            "(Lorg/bukkit/generator/WorldInfo;Ljava/util/Random;II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
pub mod structure;
