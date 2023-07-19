use crate::JNIRaw;
pub struct MapFontCharacterSprite<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for MapFontCharacterSprite<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapFontCharacterSprite<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
        _arg2: Vec<bool>,
    ) -> Result<crate::bukkit::map::MapFontCharacterSprite<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let cls = &jni.find_class("org/bukkit/map/MapFont$CharacterSprite")?;
        let res = jni.new_object(
            cls,
            "(IIZ)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::map::MapFontCharacterSprite(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate MapFontCharacterSprite from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapFontCharacterSprite") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapFontCharacterSprite object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn get(&mut self, arg0: i32, arg1: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            "(II)Z",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn width(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct MapCursorCollection<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for MapCursorCollection<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapCursorCollection<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::map::MapCursorCollection<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/map/MapCursorCollection")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bukkit::map::MapCursorCollection(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MapCursorCollection from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapCursorCollection") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapCursorCollection object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn size(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn get_cursor(
        &mut self,
        arg0: i32,
    ) -> Result<crate::bukkit::map::MapCursor<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCursor",
            "(I)Lorg/bukkit/map/MapCursor;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::map::MapCursor(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn remove_cursor(
        &mut self,
        arg0: impl Into<crate::bukkit::map::MapCursor<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeCursor",
            "(Lorg/bukkit/map/MapCursor;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
/// An instantiatable struct that implements MapCanvas. Needed for returning it from Java.
pub struct MapCanvas<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MapCanvas<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapCanvas from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapCanvas") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapCanvas object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn map_view(
        &mut self,
    ) -> Result<crate::bukkit::map::MapView<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getMapView",
            "()Lorg/bukkit/map/MapView;",
            &[],
        )?;
        let ret = {
            crate::bukkit::map::MapView(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn cursors(
        &mut self,
    ) -> Result<crate::bukkit::map::MapCursorCollection<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCursors",
            "()Lorg/bukkit/map/MapCursorCollection;",
            &[],
        )?;
        let ret = {
            crate::bukkit::map::MapCursorCollection(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_cursors(
        &mut self,
        arg0: impl Into<crate::bukkit::map::MapCursorCollection<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCursors",
            "(Lorg/bukkit/map/MapCursorCollection;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn get_pixel_color(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPixelColor",
            "(II)Ljava/awt/Color;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let r = self
            .jni_ref()
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getRed",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        let g = self
            .jni_ref()
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getGreen",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        let b = self
            .jni_ref()
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getBlue",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        (r, g, b);
        Ok((r, g, b))
    }
    pub fn get_base_pixel_color(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBasePixelColor",
            "(II)Ljava/awt/Color;",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let r = self
            .jni_ref()
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getRed",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        let g = self
            .jni_ref()
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getGreen",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        let b = self
            .jni_ref()
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getBlue",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        (r, g, b);
        Ok((r, g, b))
    }
    pub fn set_pixel(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: i8,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let val_2 = jni::objects::JValueGen::Byte(arg2.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setPixel",
            "(IIB)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(())
    }
    pub fn get_pixel(&mut self, arg0: i32, arg1: i32) -> Result<i8, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPixel",
            "(II)B",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.b().unwrap())
    }
    pub fn get_base_pixel(
        &mut self,
        arg0: i32,
        arg1: i32,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBasePixel",
            "(II)B",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(res.b().unwrap())
    }
    pub fn draw_image(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let val_2 = arg2;
        self.jni_ref().call_method(
            &self.jni_object(),
            "drawImage",
            "(IILjava/awt/Image;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(())
    }
    pub fn draw_text(
        &mut self,
        arg0: i32,
        arg1: i32,
        arg2: impl Into<crate::bukkit::map::MapFont<'mc>>,
        arg3: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone()) };
        let val_3 = jni::objects::JObject::from(self.jni_ref().new_string(arg3).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "drawText",
            "(IILorg/bukkit/map/MapFont;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for MapCanvas<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct MapRenderer<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for MapRenderer<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapRenderer<'mc> {
    pub fn from_extendable(
        env: &crate::SharedJNIEnv<'mc>,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = jni::objects::JValueGen::Object(env.new_object(
            "net/ioixd/blackbox/extendables/ExtendableMapRenderer",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&jni::objects::JObject::from(
                    env.new_string(name).unwrap(),
                )),
                jni::objects::JValueGen::from(&jni::objects::JObject::from(
                    env.new_string(lib_name).unwrap(),
                )),
            ],
        )?);
        Ok(Self(env.clone(), unsafe {
            jni::objects::JObject::from_raw(obj.l()?.clone())
        }))
    }
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<bool>,
    ) -> Result<crate::bukkit::map::MapRenderer<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Bool(arg0.unwrap().into());
        let cls = &jni.find_class("org/bukkit/map/MapRenderer")?;
        let res = jni.new_object(cls, "(Z)V", &[jni::objects::JValueGen::from(&val_0)])?;
        let ret = { crate::bukkit::map::MapRenderer(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapRenderer from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapRenderer") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapRenderer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn initialize(
        &mut self,
        arg0: impl Into<crate::bukkit::map::MapView<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "initialize",
            "(Lorg/bukkit/map/MapView;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn is_contextual(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isContextual", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn render(
        &mut self,
        arg0: impl Into<crate::bukkit::map::MapView<'mc>>,
        arg1: impl Into<crate::bukkit::map::MapCanvas<'mc>>,
        arg2: impl Into<crate::bukkit::entity::Player<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "render",
            "(Lorg/bukkit/map/MapView;Lorg/bukkit/map/MapCanvas;Lorg/bukkit/entity/Player;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct MapViewScale<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for MapViewScale<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapViewScale<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapViewScale from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapViewScale") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapViewScale object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn value(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", "()B", &[])?;
        Ok(res.b().unwrap())
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[])?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn describe_constable(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
/// An instantiatable struct that implements MapView. Needed for returning it from Java.
pub struct MapView<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MapView<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapView from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapView") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapView object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_scale(
        &mut self,
        arg0: impl Into<crate::bukkit::map::MapViewScale<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            "(Lorg/bukkit/map/MapView$Scale;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn id(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getId", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn is_locked(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn world(&mut self) -> Result<crate::bukkit::World<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWorld",
            "()Lorg/bukkit/World;",
            &[],
        )?;
        let ret = {
            crate::bukkit::World(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn scale(
        &mut self,
    ) -> Result<crate::bukkit::map::MapViewScale<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getScale",
            "()Lorg/bukkit/map/MapView$Scale;",
            &[],
        )?;
        let ret = {
            crate::bukkit::map::MapViewScale(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_world(
        &mut self,
        arg0: impl Into<crate::bukkit::World<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setWorld",
            "(Lorg/bukkit/World;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn center_x(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCenterX", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn center_z(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCenterZ", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn is_virtual(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVirtual", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_center_x(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCenterX",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_center_z(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCenterZ",
            "(I)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn add_renderer(
        &mut self,
        arg0: impl Into<crate::bukkit::map::MapRenderer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "addRenderer",
            "(Lorg/bukkit/map/MapRenderer;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn remove_renderer(
        &mut self,
        arg0: impl Into<crate::bukkit::map::MapRenderer<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeRenderer",
            "(Lorg/bukkit/map/MapRenderer;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn is_tracking_position(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isTrackingPosition", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_tracking_position(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setTrackingPosition",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn is_unlimited_tracking(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isUnlimitedTracking", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_unlimited_tracking(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setUnlimitedTracking",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_locked(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLocked",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for MapView<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct MapFont<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for MapFont<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapFont<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::map::MapFont<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/map/MapFont")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bukkit::map::MapFont(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapFont from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapFont") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapFont object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_valid(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isValid",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_char(
        &mut self,
        arg0: u16,
    ) -> Result<crate::bukkit::map::MapFontCharacterSprite<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Char(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChar",
            "(C)Lorg/bukkit/map/MapFont$CharacterSprite;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::map::MapFontCharacterSprite(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_char(
        &mut self,
        arg0: u16,
        arg1: impl Into<crate::bukkit::map::MapFontCharacterSprite<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Char(arg0.into());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setChar",
            "(CLorg/bukkit/map/MapFont$CharacterSprite;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn get_width(&mut self, arg0: String) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWidth",
            "(Ljava/lang/String;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct MinecraftFont<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for MinecraftFont<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MinecraftFont<'mc> {
    pub fn new(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::map::MinecraftFont<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/map/MinecraftFont")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bukkit::map::MinecraftFont(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MinecraftFont from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MinecraftFont") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MinecraftFont object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_valid(&mut self, arg0: String) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isValid",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn get_char(
        &mut self,
        arg0: u16,
    ) -> Result<crate::bukkit::map::MapFontCharacterSprite<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Char(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChar",
            "(C)Lorg/bukkit/map/MapFont$CharacterSprite;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::map::MapFontCharacterSprite(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn set_char(
        &mut self,
        arg0: u16,
        arg1: impl Into<crate::bukkit::map::MapFontCharacterSprite<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Char(arg0.into());
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg1.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setChar",
            "(CLorg/bukkit/map/MapFont$CharacterSprite;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn height(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn get_width(&mut self, arg0: String) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWidth",
            "(Ljava/lang/String;)I",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct MapPalette<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
/// An instantiatable struct that implements MapPaletteMapColorCache. Needed for returning it from Java.
pub struct MapPaletteMapColorCache<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> MapPaletteMapColorCache<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate MapPaletteMapColorCache from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapPaletteMapColorCache") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapPaletteMapColorCache object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn is_cached(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCached", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> crate::JNIRaw<'mc> for MapPaletteMapColorCache<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> crate::JNIRaw<'mc> for MapPalette<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapPalette<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapPalette from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapPalette") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapPalette object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn get_color(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: i8,
    ) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let cls = &jni.find_class("java/awt/Color")?;
        let res = jni.call_static_method(
            cls,
            "getColor",
            "(B)Ljava/awt/Color;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let r = jni
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getRed",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        let g = jni
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getGreen",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        let b = jni
            .call_method(
                unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
                "getBlue",
                "(V)I",
                &[],
            )?
            .i()? as u8;
        (r, g, b);
        Ok((r, g, b))
    }
    pub fn resize_image(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let cls = &jni.find_class("java/awt/image/BufferedImage")?;
        let res = jni.call_static_method(
            cls,
            "resizeImage",
            "(Ljava/awt/Image;)Ljava/awt/image/BufferedImage;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn set_map_color_cache(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::bukkit::map::MapPaletteMapColorCache<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        let cls = &jni.find_class("void")?;
        let _res = jni.call_static_method(
            cls,
            "setMapColorCache",
            "(Lorg/bukkit/map/MapPalette$MapColorCache;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
pub struct MapCursor<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct MapCursorType<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for MapCursorType<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapCursorType<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapCursorType from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapCursorType") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapCursorType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn value(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", "()B", &[])?;
        Ok(res.b().unwrap())
    }
    pub fn by_value(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: i8,
    ) -> Result<crate::bukkit::map::MapCursorType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        let cls = &jni.find_class("org/bukkit/map/MapCursor$Type")?;
        let res = jni.call_static_method(
            cls,
            "byValue",
            "(B)Lorg/bukkit/map/MapCursor$Type;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::map::MapCursorType(jni, obj)
        };
        Ok(ret)
    }
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "name", "()Ljava/lang/String;", &[])?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn describe_constable(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        )?;
        Ok(res.l().unwrap())
    }
    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
impl<'mc> crate::JNIRaw<'mc> for MapCursor<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> MapCursor<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapCursor from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("MapCursor") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapCursor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn get_type(
        &mut self,
    ) -> Result<crate::bukkit::map::MapCursorType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getType",
            "()Lorg/bukkit/map/MapCursor$Type;",
            &[],
        )?;
        let ret = {
            crate::bukkit::map::MapCursorType(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn raw_type(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getRawType", "()B", &[])?;
        Ok(res.b().unwrap())
    }
    pub fn set_type(
        &mut self,
        arg0: impl Into<crate::bukkit::map::MapCursorType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.into().1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setType",
            "(Lorg/bukkit/map/MapCursor$Type;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn is_visible(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisible", "()Z", &[])?;
        Ok(res.z().unwrap())
    }
    pub fn set_visible(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Bool(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setVisible",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn x(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", "()B", &[])?;
        Ok(res.b().unwrap())
    }
    pub fn y(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", "()B", &[])?;
        Ok(res.b().unwrap())
    }
    pub fn set_x(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setX",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_y(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setY",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn direction(&mut self) -> Result<i8, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getDirection", "()B", &[])?;
        Ok(res.b().unwrap())
    }
    pub fn set_direction(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_raw_type(&mut self, arg0: i8) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Byte(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setRawType",
            "(B)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn caption(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCaption",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn set_caption(&mut self, arg0: String) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(self.jni_ref().new_string(arg0).unwrap());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setCaption",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        Ok(())
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(res.z().unwrap())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "toString",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClass",
            "()Ljava/lang/Class;",
            &[],
        )?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[])?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[])?;
        Ok(())
    }
}
