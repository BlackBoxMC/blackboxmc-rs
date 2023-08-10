#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents all the map cursors on a <a href="MapCanvas.html" title="interface in org.bukkit.map"><code>MapCanvas</code></a>. Like MapCanvas, a MapCursorCollection is linked to a specific <a title="class in org.bukkit.map" href="MapRenderer.html"><code>MapRenderer</code></a>.
pub struct MapCursorCollection<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MapCursorCollection<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapCursorCollection<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MapCursorCollection from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapCursorCollection")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapCursorCollection object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::map::MapCursorCollection<'mc>, Box<dyn std::error::Error>> {
        let cls = jni.find_class("org/bukkit/map/MapCursorCollection");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, "()V", &[]);
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MapCursorCollection::from_raw(&jni, res)
    }
    //@NotNull

    /// Get a cursor from this collection.
    pub fn get_cursor(
        &mut self,
        arg0: i32,
    ) -> Result<crate::map::MapCursor<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCursor",
            "(I)Lorg/bukkit/map/MapCursor;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapCursor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove_cursor(
        &mut self,
        arg0: impl Into<crate::map::MapCursor<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeCursor",
            "(Lorg/bukkit/map/MapCursor;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //@NotNull

    /// Add a cursor to the collection.
    pub fn add_cursor_with_map_cursor(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i8>,
    ) -> Result<crate::map::MapCursor<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Byte(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addCursor",
            "(IIB)Lorg/bukkit/map/MapCursor;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapCursor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_cursor_with_int(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i8,
        arg3: i8,
        arg4: bool,
        arg5: std::option::Option<impl Into<String>>,
    ) -> Result<crate::map::MapCursor<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Byte(arg2.into());
        let val_4 = jni::objects::JValueGen::Byte(arg3.into());
        // 5
        let val_5 = jni::objects::JValueGen::Bool(arg4.into());
        let val_6 = jni::objects::JObject::from(
            self.jni_ref().new_string(
                arg5.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addCursor",
            "(IIBBZLjava/lang/String;)Lorg/bukkit/map/MapCursor;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapCursor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

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
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// An enum representing all possible scales a map can be set to.
pub enum MapViewScaleEnum {
    Closest,
    Close,
    Normal,
    Far,
    Farthest,
}
impl std::fmt::Display for MapViewScaleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapViewScaleEnum::Closest => f.write_str("CLOSEST"),
            MapViewScaleEnum::Close => f.write_str("CLOSE"),
            MapViewScaleEnum::Normal => f.write_str("NORMAL"),
            MapViewScaleEnum::Far => f.write_str("FAR"),
            MapViewScaleEnum::Farthest => f.write_str("FARTHEST"),
        }
    }
}
pub struct MapViewScale<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub MapViewScaleEnum,
);
impl<'mc> std::ops::Deref for MapViewScale<'mc> {
    type Target = MapViewScaleEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for MapViewScale<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapViewScale<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: MapViewScaleEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapViewScale from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapView$Scale")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapViewScale object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const CLOSEST: MapViewScaleEnum = MapViewScaleEnum::Closest;
    pub const CLOSE: MapViewScaleEnum = MapViewScaleEnum::Close;
    pub const NORMAL: MapViewScaleEnum = MapViewScaleEnum::Normal;
    pub const FAR: MapViewScaleEnum = MapViewScaleEnum::Far;
    pub const FARTHEST: MapViewScaleEnum = MapViewScaleEnum::Farthest;
    pub fn from_string(str: String) -> std::option::Option<MapViewScaleEnum> {
        match str.as_str() {
            "CLOSEST" => Some(MapViewScaleEnum::Closest),
            "CLOSE" => Some(MapViewScaleEnum::Close),
            "NORMAL" => Some(MapViewScaleEnum::Normal),
            "FAR" => Some(MapViewScaleEnum::Far),
            "FARTHEST" => Some(MapViewScaleEnum::Farthest),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<MapViewScale<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/map/MapView$Scale");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/map/MapView$Scale;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        MapViewScale::from_raw(
            &jni,
            raw_obj,
            MapViewScale::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    //

    //

    pub fn value(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
}
pub enum ScaleEnum {
    Closest,
    Close,
    Normal,
    Far,
    Farthest,
}
impl std::fmt::Display for ScaleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScaleEnum::Closest => f.write_str("CLOSEST"),
            ScaleEnum::Close => f.write_str("CLOSE"),
            ScaleEnum::Normal => f.write_str("NORMAL"),
            ScaleEnum::Far => f.write_str("FAR"),
            ScaleEnum::Farthest => f.write_str("FARTHEST"),
        }
    }
}
pub struct Scale<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub ScaleEnum,
);
impl<'mc> std::ops::Deref for Scale<'mc> {
    type Target = ScaleEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for Scale<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Scale<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: ScaleEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Scale from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/Scale")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Scale object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const CLOSEST: ScaleEnum = ScaleEnum::Closest;
    pub const CLOSE: ScaleEnum = ScaleEnum::Close;
    pub const NORMAL: ScaleEnum = ScaleEnum::Normal;
    pub const FAR: ScaleEnum = ScaleEnum::Far;
    pub const FARTHEST: ScaleEnum = ScaleEnum::Farthest;
    pub fn from_string(str: String) -> std::option::Option<ScaleEnum> {
        match str.as_str() {
            "CLOSEST" => Some(ScaleEnum::Closest),
            "CLOSE" => Some(ScaleEnum::Close),
            "NORMAL" => Some(ScaleEnum::Normal),
            "FAR" => Some(ScaleEnum::Far),
            "FARTHEST" => Some(ScaleEnum::Farthest),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<Scale<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/map/Scale");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/map/Scale;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        Scale::from_raw(
            &jni,
            raw_obj,
            Scale::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Represents the built-in Minecraft font.
pub struct MinecraftFont<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MinecraftFont<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MinecraftFont<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MinecraftFont from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MinecraftFont")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MinecraftFont object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::map::MinecraftFont<'mc>, Box<dyn std::error::Error>> {
        let cls = jni.find_class("org/bukkit/map/MinecraftFont");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, "()V", &[]);
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MinecraftFont::from_raw(&jni, res)
    }
    //

    pub fn is_valid(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isValid",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn get_width(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWidth",
            "(Ljava/lang/String;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //@Nullable

    pub fn get_char(
        &mut self,
        arg0: u16,
    ) -> Result<Option<crate::map::MapFontCharacterSprite<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Char(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChar",
            "(C)Lorg/bukkit/map/MapFont$CharacterSprite;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::map::MapFontCharacterSprite::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    //

    pub fn set_char(
        &mut self,
        arg0: u16,
        arg1: impl Into<crate::map::MapFontCharacterSprite<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Char(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setChar",
            "(CLorg/bukkit/map/MapFont$CharacterSprite;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

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
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::map::MapFont<'mc>> for MinecraftFont<'mc> {
    fn into(self) -> crate::map::MapFont<'mc> {
        crate::map::MapFont::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MinecraftFont into crate::map::MapFont")
    }
}
/// Represents a cursor on a map.
pub struct MapCursor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub enum MapCursorTypeEnum {
    WhitePointer,
    GreenPointer,
    RedPointer,
    BluePointer,
    WhiteCross,
    RedMarker,
    WhiteCircle,
    SmallWhiteCircle,
    Mansion,
    Temple,
    BannerWhite,
    BannerOrange,
    BannerMagenta,
    BannerLightBlue,
    BannerYellow,
    BannerLime,
    BannerPink,
    BannerGray,
    BannerLightGray,
    BannerCyan,
    BannerPurple,
    BannerBlue,
    BannerBrown,
    BannerGreen,
    BannerRed,
    BannerBlack,
    RedX,
}
impl std::fmt::Display for MapCursorTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapCursorTypeEnum::WhitePointer => f.write_str("WHITE_POINTER"),
            MapCursorTypeEnum::GreenPointer => f.write_str("GREEN_POINTER"),
            MapCursorTypeEnum::RedPointer => f.write_str("RED_POINTER"),
            MapCursorTypeEnum::BluePointer => f.write_str("BLUE_POINTER"),
            MapCursorTypeEnum::WhiteCross => f.write_str("WHITE_CROSS"),
            MapCursorTypeEnum::RedMarker => f.write_str("RED_MARKER"),
            MapCursorTypeEnum::WhiteCircle => f.write_str("WHITE_CIRCLE"),
            MapCursorTypeEnum::SmallWhiteCircle => f.write_str("SMALL_WHITE_CIRCLE"),
            MapCursorTypeEnum::Mansion => f.write_str("MANSION"),
            MapCursorTypeEnum::Temple => f.write_str("TEMPLE"),
            MapCursorTypeEnum::BannerWhite => f.write_str("BANNER_WHITE"),
            MapCursorTypeEnum::BannerOrange => f.write_str("BANNER_ORANGE"),
            MapCursorTypeEnum::BannerMagenta => f.write_str("BANNER_MAGENTA"),
            MapCursorTypeEnum::BannerLightBlue => f.write_str("BANNER_LIGHT_BLUE"),
            MapCursorTypeEnum::BannerYellow => f.write_str("BANNER_YELLOW"),
            MapCursorTypeEnum::BannerLime => f.write_str("BANNER_LIME"),
            MapCursorTypeEnum::BannerPink => f.write_str("BANNER_PINK"),
            MapCursorTypeEnum::BannerGray => f.write_str("BANNER_GRAY"),
            MapCursorTypeEnum::BannerLightGray => f.write_str("BANNER_LIGHT_GRAY"),
            MapCursorTypeEnum::BannerCyan => f.write_str("BANNER_CYAN"),
            MapCursorTypeEnum::BannerPurple => f.write_str("BANNER_PURPLE"),
            MapCursorTypeEnum::BannerBlue => f.write_str("BANNER_BLUE"),
            MapCursorTypeEnum::BannerBrown => f.write_str("BANNER_BROWN"),
            MapCursorTypeEnum::BannerGreen => f.write_str("BANNER_GREEN"),
            MapCursorTypeEnum::BannerRed => f.write_str("BANNER_RED"),
            MapCursorTypeEnum::BannerBlack => f.write_str("BANNER_BLACK"),
            MapCursorTypeEnum::RedX => f.write_str("RED_X"),
        }
    }
}
pub struct MapCursorType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub MapCursorTypeEnum,
);
impl<'mc> std::ops::Deref for MapCursorType<'mc> {
    type Target = MapCursorTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for MapCursorType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapCursorType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: MapCursorTypeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapCursorType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapCursor$Type")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapCursorType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const WHITE_POINTER: MapCursorTypeEnum = MapCursorTypeEnum::WhitePointer;
    pub const GREEN_POINTER: MapCursorTypeEnum = MapCursorTypeEnum::GreenPointer;
    pub const RED_POINTER: MapCursorTypeEnum = MapCursorTypeEnum::RedPointer;
    pub const BLUE_POINTER: MapCursorTypeEnum = MapCursorTypeEnum::BluePointer;
    pub const WHITE_CROSS: MapCursorTypeEnum = MapCursorTypeEnum::WhiteCross;
    pub const RED_MARKER: MapCursorTypeEnum = MapCursorTypeEnum::RedMarker;
    pub const WHITE_CIRCLE: MapCursorTypeEnum = MapCursorTypeEnum::WhiteCircle;
    pub const SMALL_WHITE_CIRCLE: MapCursorTypeEnum = MapCursorTypeEnum::SmallWhiteCircle;
    pub const MANSION: MapCursorTypeEnum = MapCursorTypeEnum::Mansion;
    pub const TEMPLE: MapCursorTypeEnum = MapCursorTypeEnum::Temple;
    pub const BANNER_WHITE: MapCursorTypeEnum = MapCursorTypeEnum::BannerWhite;
    pub const BANNER_ORANGE: MapCursorTypeEnum = MapCursorTypeEnum::BannerOrange;
    pub const BANNER_MAGENTA: MapCursorTypeEnum = MapCursorTypeEnum::BannerMagenta;
    pub const BANNER_LIGHT_BLUE: MapCursorTypeEnum = MapCursorTypeEnum::BannerLightBlue;
    pub const BANNER_YELLOW: MapCursorTypeEnum = MapCursorTypeEnum::BannerYellow;
    pub const BANNER_LIME: MapCursorTypeEnum = MapCursorTypeEnum::BannerLime;
    pub const BANNER_PINK: MapCursorTypeEnum = MapCursorTypeEnum::BannerPink;
    pub const BANNER_GRAY: MapCursorTypeEnum = MapCursorTypeEnum::BannerGray;
    pub const BANNER_LIGHT_GRAY: MapCursorTypeEnum = MapCursorTypeEnum::BannerLightGray;
    pub const BANNER_CYAN: MapCursorTypeEnum = MapCursorTypeEnum::BannerCyan;
    pub const BANNER_PURPLE: MapCursorTypeEnum = MapCursorTypeEnum::BannerPurple;
    pub const BANNER_BLUE: MapCursorTypeEnum = MapCursorTypeEnum::BannerBlue;
    pub const BANNER_BROWN: MapCursorTypeEnum = MapCursorTypeEnum::BannerBrown;
    pub const BANNER_GREEN: MapCursorTypeEnum = MapCursorTypeEnum::BannerGreen;
    pub const BANNER_RED: MapCursorTypeEnum = MapCursorTypeEnum::BannerRed;
    pub const BANNER_BLACK: MapCursorTypeEnum = MapCursorTypeEnum::BannerBlack;
    pub const RED_X: MapCursorTypeEnum = MapCursorTypeEnum::RedX;
    pub fn from_string(str: String) -> std::option::Option<MapCursorTypeEnum> {
        match str.as_str() {
            "WHITE_POINTER" => Some(MapCursorTypeEnum::WhitePointer),
            "GREEN_POINTER" => Some(MapCursorTypeEnum::GreenPointer),
            "RED_POINTER" => Some(MapCursorTypeEnum::RedPointer),
            "BLUE_POINTER" => Some(MapCursorTypeEnum::BluePointer),
            "WHITE_CROSS" => Some(MapCursorTypeEnum::WhiteCross),
            "RED_MARKER" => Some(MapCursorTypeEnum::RedMarker),
            "WHITE_CIRCLE" => Some(MapCursorTypeEnum::WhiteCircle),
            "SMALL_WHITE_CIRCLE" => Some(MapCursorTypeEnum::SmallWhiteCircle),
            "MANSION" => Some(MapCursorTypeEnum::Mansion),
            "TEMPLE" => Some(MapCursorTypeEnum::Temple),
            "BANNER_WHITE" => Some(MapCursorTypeEnum::BannerWhite),
            "BANNER_ORANGE" => Some(MapCursorTypeEnum::BannerOrange),
            "BANNER_MAGENTA" => Some(MapCursorTypeEnum::BannerMagenta),
            "BANNER_LIGHT_BLUE" => Some(MapCursorTypeEnum::BannerLightBlue),
            "BANNER_YELLOW" => Some(MapCursorTypeEnum::BannerYellow),
            "BANNER_LIME" => Some(MapCursorTypeEnum::BannerLime),
            "BANNER_PINK" => Some(MapCursorTypeEnum::BannerPink),
            "BANNER_GRAY" => Some(MapCursorTypeEnum::BannerGray),
            "BANNER_LIGHT_GRAY" => Some(MapCursorTypeEnum::BannerLightGray),
            "BANNER_CYAN" => Some(MapCursorTypeEnum::BannerCyan),
            "BANNER_PURPLE" => Some(MapCursorTypeEnum::BannerPurple),
            "BANNER_BLUE" => Some(MapCursorTypeEnum::BannerBlue),
            "BANNER_BROWN" => Some(MapCursorTypeEnum::BannerBrown),
            "BANNER_GREEN" => Some(MapCursorTypeEnum::BannerGreen),
            "BANNER_RED" => Some(MapCursorTypeEnum::BannerRed),
            "BANNER_BLACK" => Some(MapCursorTypeEnum::BannerBlack),
            "RED_X" => Some(MapCursorTypeEnum::RedX),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<MapCursorType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/map/MapCursor$Type");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/map/MapCursor$Type;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        MapCursorType::from_raw(
            &jni,
            raw_obj,
            MapCursorType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }

    //@Deprecated

    #[deprecated]
    //@Nullable

    pub fn by_value(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i8,
    ) -> Result<Option<crate::map::MapCursorType<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let cls = jni.find_class("org/bukkit/map/MapCursor$Type");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "byValue",
            "(B)Lorg/bukkit/map/MapCursor$Type;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        Ok(Some(crate::map::MapCursorType::from_raw(
            &jni,
            raw_obj,
            crate::map::MapCursorType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )?))
    }
    //

    //

    pub fn value(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MapCursor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapCursor<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapCursor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapCursor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapCursor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_byte(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i8,
        arg1: i8,
        arg2: i8,
        arg3: impl Into<crate::map::MapCursorType<'mc>>,
        arg4: bool,
        arg5: std::option::Option<impl Into<String>>,
    ) -> Result<crate::map::MapCursor<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let val_2 = jni::objects::JValueGen::Byte(arg1.into());
        let val_3 = jni::objects::JValueGen::Byte(arg2.into());
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        // 5
        let val_5 = jni::objects::JValueGen::Bool(arg4.into());
        let val_6 = jni::objects::JObject::from(
            jni.new_string(
                arg5.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                    .into(),
            )?,
        );
        let cls = jni.find_class("org/bukkit/map/MapCursor");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            "(BBBLorg/bukkit/map/MapCursor$Type;ZLjava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MapCursor::from_raw(&jni, res)
    }
    //

    pub fn set_type(
        &mut self,
        arg0: impl Into<crate::map::MapCursorType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/map/MapCursor$Type;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisible", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// Set the visibility status of this cursor.
    pub fn set_visible(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn x(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //

    pub fn y(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //

    /// Set the X position of this cursor.
    pub fn set_x(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setX",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    /// Set the Y position of this cursor.
    pub fn set_y(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setY",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn direction(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDirection", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //

    /// Set the direction of this cursor.
    pub fn set_direction(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@Deprecated

    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Set the type of this cursor.
    pub fn set_raw_type(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawType",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn caption(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCaption",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn set_caption(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCaption",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn get_type(
        &mut self,
    ) -> Result<crate::map::MapCursorType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/map/MapCursor$Type;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::map::MapCursorType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::map::MapCursorType::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn raw_type(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawType", "()B", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

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
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents the graphics for a single character in a MapFont.
pub struct MapFontCharacterSprite<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MapFontCharacterSprite<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapFontCharacterSprite<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate MapFontCharacterSprite from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapFont$CharacterSprite")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapFontCharacterSprite object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
        arg2: Vec<bool>,
    ) -> Result<crate::map::MapFontCharacterSprite<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let cls = jni.find_class("org/bukkit/map/MapFont$CharacterSprite");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            "(IIZ)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MapFontCharacterSprite::from_raw(&jni, res)
    }
    //

    pub fn height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn width(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    /// Get the value of a pixel of the character.
    pub fn get(&mut self, arg0: i32, arg1: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(II)Z",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

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
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents a canvas for drawing to a map. Each canvas is associated with a specific <a href="MapRenderer.html" title="class in org.bukkit.map"><code>MapRenderer</code></a> and represents that renderer's layer on the map.
///
/// This is a representation of an abstract class.
pub struct MapCanvas<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MapCanvas<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapCanvas from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapCanvas")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapCanvas object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn map_view(&mut self) -> Result<crate::map::MapView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMapView",
            "()Lorg/bukkit/map/MapView;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn cursors(
        &mut self,
    ) -> Result<crate::map::MapCursorCollection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCursors",
            "()Lorg/bukkit/map/MapCursorCollection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapCursorCollection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_cursors(
        &mut self,
        arg0: impl Into<crate::map::MapCursorCollection<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursors",
            "(Lorg/bukkit/map/MapCursorCollection;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@Nullable

    /// Get a pixel from the canvas. If no color is set at the given position for this canvas, then null is returned and the color returned by <a href="#getBasePixelColor(int,int)"><code>getBasePixelColor(int, int)</code></a> is shown on the map.
    pub fn get_pixel_color(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<Option<(u8, u8, u8)>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPixelColor",
            "(II)Ljava/awt/Color;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let r = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getRed",
            "(V)I",
            &[],
        );
        let r = self.jni_ref().translate_error(r)?.i()? as u8;
        let g = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getGreen",
            "(V)I",
            &[],
        );
        let g = self.jni_ref().translate_error(g)?.i()? as u8;
        let b = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getBlue",
            "(V)I",
            &[],
        );
        let b = self.jni_ref().translate_error(b)?.i()? as u8;
        Ok(Some((r, g, b)))
    }
    //@NotNull

    /// Get a pixel from the layers below this canvas.
    pub fn get_base_pixel_color(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBasePixelColor",
            "(II)Ljava/awt/Color;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let r = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getRed",
            "(V)I",
            &[],
        );
        let r = self.jni_ref().translate_error(r)?.i()? as u8;
        let g = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getGreen",
            "(V)I",
            &[],
        );
        let g = self.jni_ref().translate_error(g)?.i()? as u8;
        let b = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getBlue",
            "(V)I",
            &[],
        );
        let b = self.jni_ref().translate_error(b)?.i()? as u8;
        Ok((r, g, b))
    }
    //

    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value, use <a href="#setPixelColor(int,int,java.awt.Color)"><code>setPixelColor(int, int, Color)</code></a>
    /// </div>
    /// Magic value, use <a href="#setPixelColor(int,int,java.awt.Color)"><code>setPixelColor(int, int, Color)</code></a>
    ///
    /// Draw a pixel to the canvas.
    pub fn set_pixel(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i8,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = jni::objects::JValueGen::Byte(arg2.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPixel",
            "(IIB)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@Nullable

    //@Deprecated

    #[deprecated]
    /// Get a pixel from the canvas. If no color is set at the given position for this canvas, then null is returned and the color returned by <a href="#getBasePixelColor(int,int)"><code>getBasePixelColor(int, int)</code></a> is shown on the map.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value, use <a href="#getPixelColor(int,int)"><code>getPixelColor(int, int)</code></a>
    /// </div>
    /// Magic value, use <a href="#getPixelColor(int,int)"><code>getPixelColor(int, int)</code></a>
    ///
    /// Get a pixel from the canvas.
    pub fn get_pixel(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<Option<i8>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPixel",
            "(II)B",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(Some(res.b()?))
    }
    //@NotNull

    //@Deprecated

    #[deprecated]
    /// Get a pixel from the layers below this canvas.
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value, use <a href="#getBasePixelColor(int,int)"><code>getBasePixelColor(int, int)</code></a>
    /// </div>
    /// Magic value, use <a href="#getBasePixelColor(int,int)"><code>getBasePixelColor(int, int)</code></a>
    ///
    /// Get a pixel from the layers below this canvas.
    pub fn get_base_pixel(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBasePixel",
            "(II)B",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    //

    pub fn draw_image(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = arg2;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "drawImage",
            "(IILjava/awt/Image;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn draw_text(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: impl Into<crate::map::MapFont<'mc>>,
        arg3: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = jni::objects::JObject::from(self.jni_ref().new_string(arg3.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "drawText",
            "(IILorg/bukkit/map/MapFont;Ljava/lang/String;)V",
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
impl<'mc> JNIRaw<'mc> for MapCanvas<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a renderer for a map.
pub struct MapRenderer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MapRenderer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapRenderer<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "MapRenderer", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapRenderer from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapRenderer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapRenderer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::map::MapRenderer<'mc>, Box<dyn std::error::Error>> {
        // -1
        let val_1 = jni::objects::JValueGen::Bool(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let cls = jni.find_class("org/bukkit/map/MapRenderer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, "(Z)V", &[jni::objects::JValueGen::from(&val_1)]);
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MapRenderer::from_raw(&jni, res)
    }
    //

    pub fn is_contextual(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isContextual", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn render(
        &mut self,
        arg0: impl Into<crate::map::MapView<'mc>>,
        arg1: impl Into<crate::map::MapCanvas<'mc>>,
        arg2: impl Into<crate::entity::Player<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "render",
            "(Lorg/bukkit/map/MapView;Lorg/bukkit/map/MapCanvas;Lorg/bukkit/entity/Player;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn initialize(
        &mut self,
        arg0: impl Into<crate::map::MapView<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "initialize",
            "(Lorg/bukkit/map/MapView;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

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
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents a map item.
///
/// This is a representation of an abstract class.
pub struct MapView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MapView<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapView from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapView")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn set_scale(
        &mut self,
        arg0: impl Into<crate::map::MapViewScale<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            "(Lorg/bukkit/map/MapView$Scale;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn world(&mut self) -> Result<crate::World<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getWorld", "()Lorg/bukkit/World;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn scale(&mut self) -> Result<crate::map::MapViewScale<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScale",
            "()Lorg/bukkit/map/MapView$Scale;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .jni_ref()
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::map::MapViewScale::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::map::MapViewScale::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //

    pub fn set_world(
        &mut self,
        arg0: impl Into<crate::World<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWorld",
            "(Lorg/bukkit/World;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn center_x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCenterX", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn center_z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCenterZ", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_virtual(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVirtual", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// Set the center X position of this map.
    pub fn set_center_x(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCenterX",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    /// Set the center Z position of this map.
    pub fn set_center_z(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCenterZ",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn renderers(
        &mut self,
    ) -> Result<Vec<crate::map::MapRenderer<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRenderers",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::map::MapRenderer::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn add_renderer(
        &mut self,
        arg0: impl Into<crate::map::MapRenderer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addRenderer",
            "(Lorg/bukkit/map/MapRenderer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn remove_renderer(
        &mut self,
        arg0: impl Into<crate::map::MapRenderer<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeRenderer",
            "(Lorg/bukkit/map/MapRenderer;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_tracking_position(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isTrackingPosition", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// Sets whether a position cursor should be shown when the map is near its center.
    pub fn set_tracking_position(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTrackingPosition",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn is_unlimited_tracking(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isUnlimitedTracking", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// Whether the map will show a smaller position cursor (true), or no position cursor (false) when cursor is outside of map's range.
    pub fn set_unlimited_tracking(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnlimitedTracking",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    /// Gets whether the map is locked or not. A locked map may not be explored further.
    pub fn set_locked(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLocked",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getId", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
}
impl<'mc> JNIRaw<'mc> for MapView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a bitmap font drawable to a map.
pub struct MapFont<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MapFont<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapFont<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapFont from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapFont")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapFont object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::map::MapFont<'mc>, Box<dyn std::error::Error>> {
        let cls = jni.find_class("org/bukkit/map/MapFont");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, "()V", &[]);
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MapFont::from_raw(&jni, res)
    }
    //

    pub fn is_valid(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isValid",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn get_width(
        &mut self,
        arg0: impl Into<String>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into())?);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWidth",
            "(Ljava/lang/String;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //@Nullable

    /// Get the sprite for a given character.
    pub fn get_char(
        &mut self,
        arg0: u16,
    ) -> Result<Option<crate::map::MapFontCharacterSprite<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Char(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChar",
            "(C)Lorg/bukkit/map/MapFont$CharacterSprite;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::map::MapFontCharacterSprite::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    //

    pub fn set_char(
        &mut self,
        arg0: u16,
        arg1: impl Into<crate::map::MapFontCharacterSprite<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Char(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setChar",
            "(CLorg/bukkit/map/MapFont$CharacterSprite;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

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
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents the palette that map items use.
/// <p>These fields are hee base color ranges. Each entry corresponds to four colors of varying shades with values entry to entry + 3.</p>
pub struct MapPalette<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
///
/// This is a representation of an abstract class.
pub struct MapPaletteMapColorCache<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MapPaletteMapColorCache<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate MapPaletteMapColorCache from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapPalette$MapColorCache")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapPaletteMapColorCache object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //

    pub fn is_cached(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCached", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
impl<'mc> JNIRaw<'mc> for MapPaletteMapColorCache<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for MapPalette<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapPalette<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapPalette from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapPalette")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapPalette object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    //@Deprecated

    #[deprecated]
    //@NotNull

    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Get the value of the given color in the palette.
    pub fn get_color(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i8,
    ) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Byte(arg0.into());
        let cls = jni.find_class("java/awt/Color");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getColor",
            "(B)Ljava/awt/Color;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        let r = jni.call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getRed",
            "(V)I",
            &[],
        );
        let r = jni.translate_error(r)?.i()? as u8;
        let g = jni.call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getGreen",
            "(V)I",
            &[],
        );
        let g = jni.translate_error(g)?.i()? as u8;
        let b = jni.call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getBlue",
            "(V)I",
            &[],
        );
        let b = jni.translate_error(b)?.i()? as u8;
        Ok((r, g, b))
    }
    //@Deprecated

    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Magic value
    /// </div>
    /// Magic value
    ///
    /// Get the index of the closest matching color in the palette to the given color.
    pub fn match_color_with_color(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_3 = jni::objects::JValueGen::Int(
            arg2.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let cls = jni.find_class("byte");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "matchColor",
            "(III)B",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.b()?)
    }
    //

    pub fn resize_image(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let cls = jni.find_class("java/awt/image/BufferedImage");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "resizeImage",
            "(Ljava/awt/Image;)Ljava/awt/image/BufferedImage;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.l()?)
    }
    //

    //

    pub fn set_map_color_cache(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::map::MapPaletteMapColorCache<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = jni.find_class("void");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "setMapColorCache",
            "(Lorg/bukkit/map/MapPalette$MapColorCache;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(())
    }
    //

    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(
            arg0.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
        let val_2 = jni::objects::JValueGen::Int(
            arg1.ok_or(eyre::eyre!("None arguments aren't actually supported yet"))?
                .into(),
        );
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
    //

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
        Ok(res.z()?)
    }
    //

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
    //

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
