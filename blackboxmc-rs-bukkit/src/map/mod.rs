#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct MapPalette<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapPalette<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapPalette<'mc> {
    fn from_raw(
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
}

impl<'mc> MapPalette<'mc> {
    pub fn resize_image(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        image: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/awt/Image;)Ljni::objects::JObject;");
        let val_1 = jni::objects::JValueGen::Object(image);
        let cls = jni.find_class("org/bukkit/map/MapPalette");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "resizeImage",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.l()?)
    }
    #[deprecated]

    pub fn image_to_bytes(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        image: jni::objects::JObject<'mc>,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/awt/Image;)Li8;");
        let val_1 = jni::objects::JValueGen::Object(image);
        let cls = jni.find_class("org/bukkit/map/MapPalette");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "imageToBytes",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.b()?)
    }
    #[deprecated]

    pub fn match_color_with_r(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        r: i32,
        g: std::option::Option<i32>,
        b: std::option::Option<i32>,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(r);
        args.push(val_1);
        if let Some(a) = g {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = b {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")B";
        let cls = jni.find_class("org/bukkit/map/MapPalette");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "matchColor", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.b()?)
    }
    #[deprecated]

    pub fn get_color(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        index: i8,
    ) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
        let sig = String::from("(B)L(u8, u8, u8);");
        let val_1 = jni::objects::JValueGen::Byte(index);
        let cls = jni.find_class("org/bukkit/map/MapPalette");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getColor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let r = jni.call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getRed",
            "(V)I",
            vec![],
        );
        let r = jni.translate_error(r)?.i()? as u8;
        let g = jni.call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getGreen",
            "(V)I",
            vec![],
        );
        let g = jni.translate_error(g)?.i()? as u8;
        let b = jni.call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getBlue",
            "(V)I",
            vec![],
        );
        let b = jni.translate_error(b)?.i()? as u8;
        Ok((r, g, b))
    }
    pub fn set_map_color_cache(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        map_color_cache: impl Into<crate::map::MapPaletteMapColorCache<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/map/MapPalette/MapColorCache;)L();");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(map_color_cache.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/map/MapPalette");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "setMapColorCache",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        jni.translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct MapPaletteMapColorCache<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapPaletteMapColorCache<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapPaletteMapColorCache<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate MapPaletteMapColorCache from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapPalette/MapColorCache")?;
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
}

impl<'mc> MapPaletteMapColorCache<'mc> {
    pub fn is_cached(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Lbool;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isCached", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct MapCursor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapCursor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapCursor<'mc> {
    fn from_raw(
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
}

impl<'mc> MapCursor<'mc> {
    pub fn new_with_x(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        x: i8,
        y: i8,
        direction: i8,
        val_type: impl Into<crate::map::MapCursorType<'mc>>,
        visible: bool,
        caption: std::option::Option<impl Into<String>>,
    ) -> Result<crate::map::MapCursor<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "B";
        let val_1 = jni::objects::JValueGen::Byte(x);
        args.push(val_1);
        sig += "B";
        let val_2 = jni::objects::JValueGen::Byte(y);
        args.push(val_2);
        sig += "B";
        let val_3 = jni::objects::JValueGen::Byte(direction);
        args.push(val_3);
        sig += "Lorg/bukkit/map/MapCursor/Type;";
        let val_4 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(val_type.into().jni_object().clone())
        });
        args.push(val_4);
        sig += "Z";
        let val_5 = jni::objects::JValueGen::Bool(visible.into());
        args.push(val_5);
        if let Some(a) = caption {
            sig += "Ljava/lang/String;";
            let val_6 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_6);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/map/MapCursor");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MapCursor::from_raw(&jni, res)
    }
    pub fn x(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()Li8;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getX", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    pub fn y(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()Li8;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getY", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    pub fn direction(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()Li8;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDirection", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    pub fn get_type(&self) -> Result<crate::map::MapCursorType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::map::MapCursorType;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapCursorType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn raw_type(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()Li8;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRawType", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    pub fn is_visible(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Lbool;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVisible", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn set_x(&self, x: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)L();");
        let val_1 = jni::objects::JValueGen::Byte(x);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setX",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_y(&self, y: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)L();");
        let val_1 = jni::objects::JValueGen::Byte(y);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setY",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_direction(&self, direction: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)L();");
        let val_1 = jni::objects::JValueGen::Byte(direction);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDirection",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_type(
        &self,
        val_type: impl Into<crate::map::MapCursorType<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/map/MapCursor/Type;)L();");
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
    #[deprecated]

    pub fn set_raw_type(&self, val_type: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)L();");
        let val_1 = jni::objects::JValueGen::Byte(val_type);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRawType",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_visible(&self, visible: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)L();");
        let val_1 = jni::objects::JValueGen::Bool(visible.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setVisible",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn caption(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()LString;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCaption", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }
    pub fn set_caption(
        &self,
        caption: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)L();");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(caption.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCaption",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum MapCursorType<'mc> {
    WhitePointer { inner: MapCursorTypeStruct<'mc> },
    GreenPointer { inner: MapCursorTypeStruct<'mc> },
    RedPointer { inner: MapCursorTypeStruct<'mc> },
    BluePointer { inner: MapCursorTypeStruct<'mc> },
    WhiteCross { inner: MapCursorTypeStruct<'mc> },
    RedMarker { inner: MapCursorTypeStruct<'mc> },
    WhiteCircle { inner: MapCursorTypeStruct<'mc> },
    SmallWhiteCircle { inner: MapCursorTypeStruct<'mc> },
    Mansion { inner: MapCursorTypeStruct<'mc> },
    Temple { inner: MapCursorTypeStruct<'mc> },
    BannerWhite { inner: MapCursorTypeStruct<'mc> },
    BannerOrange { inner: MapCursorTypeStruct<'mc> },
    BannerMagenta { inner: MapCursorTypeStruct<'mc> },
    BannerLightBlue { inner: MapCursorTypeStruct<'mc> },
    BannerYellow { inner: MapCursorTypeStruct<'mc> },
    BannerLime { inner: MapCursorTypeStruct<'mc> },
    BannerPink { inner: MapCursorTypeStruct<'mc> },
    BannerGray { inner: MapCursorTypeStruct<'mc> },
    BannerLightGray { inner: MapCursorTypeStruct<'mc> },
    BannerCyan { inner: MapCursorTypeStruct<'mc> },
    BannerPurple { inner: MapCursorTypeStruct<'mc> },
    BannerBlue { inner: MapCursorTypeStruct<'mc> },
    BannerBrown { inner: MapCursorTypeStruct<'mc> },
    BannerGreen { inner: MapCursorTypeStruct<'mc> },
    BannerRed { inner: MapCursorTypeStruct<'mc> },
    BannerBlack { inner: MapCursorTypeStruct<'mc> },
    RedX { inner: MapCursorTypeStruct<'mc> },
    DesertVillage { inner: MapCursorTypeStruct<'mc> },
    PlainsVillage { inner: MapCursorTypeStruct<'mc> },
    SavannaVillage { inner: MapCursorTypeStruct<'mc> },
    SnowyVillage { inner: MapCursorTypeStruct<'mc> },
    TaigaVillage { inner: MapCursorTypeStruct<'mc> },
    JungleTemple { inner: MapCursorTypeStruct<'mc> },
    SwampHut { inner: MapCursorTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for MapCursorType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapCursorType::WhitePointer { .. } => f.write_str("WHITE_POINTER"),
            MapCursorType::GreenPointer { .. } => f.write_str("GREEN_POINTER"),
            MapCursorType::RedPointer { .. } => f.write_str("RED_POINTER"),
            MapCursorType::BluePointer { .. } => f.write_str("BLUE_POINTER"),
            MapCursorType::WhiteCross { .. } => f.write_str("WHITE_CROSS"),
            MapCursorType::RedMarker { .. } => f.write_str("RED_MARKER"),
            MapCursorType::WhiteCircle { .. } => f.write_str("WHITE_CIRCLE"),
            MapCursorType::SmallWhiteCircle { .. } => f.write_str("SMALL_WHITE_CIRCLE"),
            MapCursorType::Mansion { .. } => f.write_str("MANSION"),
            MapCursorType::Temple { .. } => f.write_str("TEMPLE"),
            MapCursorType::BannerWhite { .. } => f.write_str("BANNER_WHITE"),
            MapCursorType::BannerOrange { .. } => f.write_str("BANNER_ORANGE"),
            MapCursorType::BannerMagenta { .. } => f.write_str("BANNER_MAGENTA"),
            MapCursorType::BannerLightBlue { .. } => f.write_str("BANNER_LIGHT_BLUE"),
            MapCursorType::BannerYellow { .. } => f.write_str("BANNER_YELLOW"),
            MapCursorType::BannerLime { .. } => f.write_str("BANNER_LIME"),
            MapCursorType::BannerPink { .. } => f.write_str("BANNER_PINK"),
            MapCursorType::BannerGray { .. } => f.write_str("BANNER_GRAY"),
            MapCursorType::BannerLightGray { .. } => f.write_str("BANNER_LIGHT_GRAY"),
            MapCursorType::BannerCyan { .. } => f.write_str("BANNER_CYAN"),
            MapCursorType::BannerPurple { .. } => f.write_str("BANNER_PURPLE"),
            MapCursorType::BannerBlue { .. } => f.write_str("BANNER_BLUE"),
            MapCursorType::BannerBrown { .. } => f.write_str("BANNER_BROWN"),
            MapCursorType::BannerGreen { .. } => f.write_str("BANNER_GREEN"),
            MapCursorType::BannerRed { .. } => f.write_str("BANNER_RED"),
            MapCursorType::BannerBlack { .. } => f.write_str("BANNER_BLACK"),
            MapCursorType::RedX { .. } => f.write_str("RED_X"),
            MapCursorType::DesertVillage { .. } => f.write_str("DESERT_VILLAGE"),
            MapCursorType::PlainsVillage { .. } => f.write_str("PLAINS_VILLAGE"),
            MapCursorType::SavannaVillage { .. } => f.write_str("SAVANNA_VILLAGE"),
            MapCursorType::SnowyVillage { .. } => f.write_str("SNOWY_VILLAGE"),
            MapCursorType::TaigaVillage { .. } => f.write_str("TAIGA_VILLAGE"),
            MapCursorType::JungleTemple { .. } => f.write_str("JUNGLE_TEMPLE"),
            MapCursorType::SwampHut { .. } => f.write_str("SWAMP_HUT"),
        }
    }
}

impl<'mc> MapCursorType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<MapCursorType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/map/MapCursor/Type");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/map/MapCursor/Type;",
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
            "WHITE_POINTER" => Ok(MapCursorType::WhitePointer {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "GREEN_POINTER" => Ok(MapCursorType::GreenPointer {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "RED_POINTER" => Ok(MapCursorType::RedPointer {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BLUE_POINTER" => Ok(MapCursorType::BluePointer {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "WHITE_CROSS" => Ok(MapCursorType::WhiteCross {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "RED_MARKER" => Ok(MapCursorType::RedMarker {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "WHITE_CIRCLE" => Ok(MapCursorType::WhiteCircle {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "SMALL_WHITE_CIRCLE" => Ok(MapCursorType::SmallWhiteCircle {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "MANSION" => Ok(MapCursorType::Mansion {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "TEMPLE" => Ok(MapCursorType::Temple {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_WHITE" => Ok(MapCursorType::BannerWhite {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_ORANGE" => Ok(MapCursorType::BannerOrange {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_MAGENTA" => Ok(MapCursorType::BannerMagenta {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_LIGHT_BLUE" => Ok(MapCursorType::BannerLightBlue {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_YELLOW" => Ok(MapCursorType::BannerYellow {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_LIME" => Ok(MapCursorType::BannerLime {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_PINK" => Ok(MapCursorType::BannerPink {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_GRAY" => Ok(MapCursorType::BannerGray {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_LIGHT_GRAY" => Ok(MapCursorType::BannerLightGray {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_CYAN" => Ok(MapCursorType::BannerCyan {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_PURPLE" => Ok(MapCursorType::BannerPurple {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_BLUE" => Ok(MapCursorType::BannerBlue {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_BROWN" => Ok(MapCursorType::BannerBrown {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_GREEN" => Ok(MapCursorType::BannerGreen {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_RED" => Ok(MapCursorType::BannerRed {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "BANNER_BLACK" => Ok(MapCursorType::BannerBlack {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "RED_X" => Ok(MapCursorType::RedX {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "DESERT_VILLAGE" => Ok(MapCursorType::DesertVillage {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "PLAINS_VILLAGE" => Ok(MapCursorType::PlainsVillage {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "SAVANNA_VILLAGE" => Ok(MapCursorType::SavannaVillage {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "SNOWY_VILLAGE" => Ok(MapCursorType::SnowyVillage {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "TAIGA_VILLAGE" => Ok(MapCursorType::TaigaVillage {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "JUNGLE_TEMPLE" => Ok(MapCursorType::JungleTemple {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),
            "SWAMP_HUT" => Ok(MapCursorType::SwampHut {
                inner: MapCursorTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct MapCursorTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapCursorType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::WhitePointer { inner } => inner.0.clone(),
            Self::GreenPointer { inner } => inner.0.clone(),
            Self::RedPointer { inner } => inner.0.clone(),
            Self::BluePointer { inner } => inner.0.clone(),
            Self::WhiteCross { inner } => inner.0.clone(),
            Self::RedMarker { inner } => inner.0.clone(),
            Self::WhiteCircle { inner } => inner.0.clone(),
            Self::SmallWhiteCircle { inner } => inner.0.clone(),
            Self::Mansion { inner } => inner.0.clone(),
            Self::Temple { inner } => inner.0.clone(),
            Self::BannerWhite { inner } => inner.0.clone(),
            Self::BannerOrange { inner } => inner.0.clone(),
            Self::BannerMagenta { inner } => inner.0.clone(),
            Self::BannerLightBlue { inner } => inner.0.clone(),
            Self::BannerYellow { inner } => inner.0.clone(),
            Self::BannerLime { inner } => inner.0.clone(),
            Self::BannerPink { inner } => inner.0.clone(),
            Self::BannerGray { inner } => inner.0.clone(),
            Self::BannerLightGray { inner } => inner.0.clone(),
            Self::BannerCyan { inner } => inner.0.clone(),
            Self::BannerPurple { inner } => inner.0.clone(),
            Self::BannerBlue { inner } => inner.0.clone(),
            Self::BannerBrown { inner } => inner.0.clone(),
            Self::BannerGreen { inner } => inner.0.clone(),
            Self::BannerRed { inner } => inner.0.clone(),
            Self::BannerBlack { inner } => inner.0.clone(),
            Self::RedX { inner } => inner.0.clone(),
            Self::DesertVillage { inner } => inner.0.clone(),
            Self::PlainsVillage { inner } => inner.0.clone(),
            Self::SavannaVillage { inner } => inner.0.clone(),
            Self::SnowyVillage { inner } => inner.0.clone(),
            Self::TaigaVillage { inner } => inner.0.clone(),
            Self::JungleTemple { inner } => inner.0.clone(),
            Self::SwampHut { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::WhitePointer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::GreenPointer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RedPointer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BluePointer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WhiteCross { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RedMarker { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WhiteCircle { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SmallWhiteCircle { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Mansion { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Temple { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::BannerWhite { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerOrange { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerMagenta { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerLightBlue { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerYellow { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerLime { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerPink { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerGray { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerLightGray { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerCyan { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerPurple { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerBlue { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerBrown { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerGreen { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerRed { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BannerBlack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RedX { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::DesertVillage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PlainsVillage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SavannaVillage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SnowyVillage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TaigaVillage { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::JungleTemple { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SwampHut { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapCursorType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapCursorType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapCursor/Type")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapCursorType object, got {}",
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
                "WHITE_POINTER" => Ok(MapCursorType::WhitePointer {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "GREEN_POINTER" => Ok(MapCursorType::GreenPointer {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "RED_POINTER" => Ok(MapCursorType::RedPointer {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BLUE_POINTER" => Ok(MapCursorType::BluePointer {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "WHITE_CROSS" => Ok(MapCursorType::WhiteCross {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "RED_MARKER" => Ok(MapCursorType::RedMarker {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "WHITE_CIRCLE" => Ok(MapCursorType::WhiteCircle {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "SMALL_WHITE_CIRCLE" => Ok(MapCursorType::SmallWhiteCircle {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "MANSION" => Ok(MapCursorType::Mansion {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "TEMPLE" => Ok(MapCursorType::Temple {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_WHITE" => Ok(MapCursorType::BannerWhite {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_ORANGE" => Ok(MapCursorType::BannerOrange {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_MAGENTA" => Ok(MapCursorType::BannerMagenta {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_LIGHT_BLUE" => Ok(MapCursorType::BannerLightBlue {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_YELLOW" => Ok(MapCursorType::BannerYellow {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_LIME" => Ok(MapCursorType::BannerLime {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_PINK" => Ok(MapCursorType::BannerPink {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_GRAY" => Ok(MapCursorType::BannerGray {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_LIGHT_GRAY" => Ok(MapCursorType::BannerLightGray {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_CYAN" => Ok(MapCursorType::BannerCyan {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_PURPLE" => Ok(MapCursorType::BannerPurple {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_BLUE" => Ok(MapCursorType::BannerBlue {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_BROWN" => Ok(MapCursorType::BannerBrown {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_GREEN" => Ok(MapCursorType::BannerGreen {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_RED" => Ok(MapCursorType::BannerRed {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "BANNER_BLACK" => Ok(MapCursorType::BannerBlack {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "RED_X" => Ok(MapCursorType::RedX {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "DESERT_VILLAGE" => Ok(MapCursorType::DesertVillage {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "PLAINS_VILLAGE" => Ok(MapCursorType::PlainsVillage {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "SAVANNA_VILLAGE" => Ok(MapCursorType::SavannaVillage {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "SNOWY_VILLAGE" => Ok(MapCursorType::SnowyVillage {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "TAIGA_VILLAGE" => Ok(MapCursorType::TaigaVillage {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "JUNGLE_TEMPLE" => Ok(MapCursorType::JungleTemple {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                "SWAMP_HUT" => Ok(MapCursorType::SwampHut {
                    inner: MapCursorTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for MapCursorTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapCursorTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MapCursorTypeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapCursor/Type")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapCursorTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MapCursorTypeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::map::MapCursorType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::map::MapCursorType;");
        let cls = jni.find_class("org/bukkit/map/MapCursor/Type");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::map::MapCursorType::from_raw(&jni, obj)
    }
    #[deprecated]

    pub fn value(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()Li8;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    #[deprecated]

    pub fn by_value(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        value: i8,
    ) -> Result<Option<crate::map::MapCursorType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(B)Lcrate::map::MapCursorType;");
        let val_1 = jni::objects::JValueGen::Byte(value);
        let cls = jni.find_class("org/bukkit/map/MapCursor/Type");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "byValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(crate::map::MapCursorType::from_raw(&jni, obj)?))
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct MinecraftFont<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MinecraftFont<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MinecraftFont<'mc> {
    fn from_raw(
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
}

impl<'mc> MinecraftFont<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::map::MinecraftFont<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/map/MinecraftFont");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MinecraftFont::from_raw(&jni, res)
    }
    // SUPER CLASS: MapFont

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::map::MapFont<'mc>> for MinecraftFont<'mc> {
    fn into(self) -> crate::map::MapFont<'mc> {
        crate::map::MapFont::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting MinecraftFont into crate::map::MapFont")
    }
}
#[repr(C)]
pub struct MapCursorCollection<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapCursorCollection<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapCursorCollection<'mc> {
    fn from_raw(
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
}

impl<'mc> MapCursorCollection<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::map::MapCursorCollection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/map/MapCursorCollection");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MapCursorCollection::from_raw(&jni, res)
    }
    pub fn size(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()Li32;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "size", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn get_cursor(
        &self,
        index: i32,
    ) -> Result<crate::map::MapCursor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lcrate::map::MapCursor;");
        let val_1 = jni::objects::JValueGen::Int(index);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getCursor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapCursor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn remove_cursor(
        &self,
        cursor: impl Into<crate::map::MapCursor<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/map/MapCursor;)Lbool;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(cursor.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeCursor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    #[deprecated]

    pub fn add_cursor_with_x(
        &self,
        x: i32,
        y: std::option::Option<i32>,
        direction: std::option::Option<i8>,
        val_type: std::option::Option<i8>,
        visible: std::option::Option<bool>,
        caption: std::option::Option<impl Into<String>>,
    ) -> Result<crate::map::MapCursor<'mc>, Box<dyn std::error::Error>> {
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
        if let Some(a) = direction {
            sig += "B";
            let val_3 = jni::objects::JValueGen::Byte(a);
            args.push(val_3);
        }
        if let Some(a) = val_type {
            sig += "B";
            let val_4 = jni::objects::JValueGen::Byte(a);
            args.push(val_4);
        }
        if let Some(a) = visible {
            sig += "Z";
            let val_5 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_5);
        }
        if let Some(a) = caption {
            sig += "Ljava/lang/String;";
            let val_6 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_6);
        }
        sig += ")Lorg/bukkit/map/MapCursor;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addCursor", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapCursor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct MapRenderer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapRenderer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapRenderer<'mc> {
    fn from_raw(
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
    pub fn new_with_contextual(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        contextual: std::option::Option<bool>,
    ) -> Result<crate::map::MapRenderer<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = contextual {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/map/MapRenderer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MapRenderer::from_raw(&jni, res)
    }
    pub fn is_contextual(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Lbool;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isContextual", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn initialize(
        &self,
        map: impl Into<crate::map::MapView<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/map/MapView;)L();");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(map.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "initialize",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct MapCanvas<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapCanvas<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapCanvas<'mc> {
    fn from_raw(
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
}

impl<'mc> MapCanvas<'mc> {
    pub fn map_view(&self) -> Result<crate::map::MapView<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::map::MapView;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMapView", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapView::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn cursors(
        &self,
    ) -> Result<crate::map::MapCursorCollection<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::map::MapCursorCollection;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCursors", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapCursorCollection::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn set_cursors(
        &self,
        cursors: impl Into<crate::map::MapCursorCollection<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/map/MapCursorCollection;)L();");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(cursors.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCursors",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn get_pixel_color(
        &self,
        x: i32,
        y: i32,
    ) -> Result<Option<(u8, u8, u8)>, Box<dyn std::error::Error>> {
        let sig = String::from("(II)L(u8, u8, u8);");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPixelColor",
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
        let r = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getRed",
            "(V)I",
            vec![],
        );
        let r = self.jni_ref().translate_error(r)?.i()? as u8;
        let g = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getGreen",
            "(V)I",
            vec![],
        );
        let g = self.jni_ref().translate_error(g)?.i()? as u8;
        let b = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getBlue",
            "(V)I",
            vec![],
        );
        let b = self.jni_ref().translate_error(b)?.i()? as u8;
        Ok(Some((r, g, b)))
    }
    pub fn get_base_pixel_color(
        &self,
        x: i32,
        y: i32,
    ) -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
        let sig = String::from("(II)L(u8, u8, u8);");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBasePixelColor",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let r = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getRed",
            "(V)I",
            vec![],
        );
        let r = self.jni_ref().translate_error(r)?.i()? as u8;
        let g = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getGreen",
            "(V)I",
            vec![],
        );
        let g = self.jni_ref().translate_error(g)?.i()? as u8;
        let b = self.jni_ref().call_method(
            unsafe { jni::objects::JObject::from_raw(res.as_jni().l) },
            "getBlue",
            "(V)I",
            vec![],
        );
        let b = self.jni_ref().translate_error(b)?.i()? as u8;
        Ok((r, g, b))
    }
    pub fn set_pixel(&self, x: i32, y: i32, color: i8) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(IIB)L();");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let val_3 = jni::objects::JValueGen::Byte(color);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPixel",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    #[deprecated]

    pub fn get_pixel(&self, x: i32, y: i32) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("(II)Li8;");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPixel",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    #[deprecated]

    pub fn get_base_pixel(&self, x: i32, y: i32) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("(II)Li8;");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getBasePixel",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    pub fn draw_image(
        &self,
        x: i32,
        y: i32,
        image: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(IILjava/awt/Image;)L();");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let val_3 = jni::objects::JValueGen::Object(image);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "drawImage",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn draw_text(
        &self,
        x: i32,
        y: i32,
        font: impl Into<crate::map::MapFont<'mc>>,
        text: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(IILorg/bukkit/map/MapFont;Ljava/lang/String;)L();");
        let val_1 = jni::objects::JValueGen::Int(x);
        let val_2 = jni::objects::JValueGen::Int(y);
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(font.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(text.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "drawText",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
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
pub struct MapFont<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapFont<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapFont<'mc> {
    fn from_raw(
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
}

impl<'mc> MapFont<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::map::MapFont<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/map/MapFont");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MapFont::from_raw(&jni, res)
    }
    pub fn set_char(
        &self,
        ch: u16,
        sprite: impl Into<crate::map::MapFontCharacterSprite<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(CLorg/bukkit/map/MapFont/CharacterSprite;)L();");
        let val_1 = jni::objects::JValueGen::Char(ch);
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(sprite.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setChar",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn get_char(
        &self,
        ch: u16,
    ) -> Result<Option<crate::map::MapFontCharacterSprite<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(C)Lcrate::map::MapFontCharacterSprite;");
        let val_1 = jni::objects::JValueGen::Char(ch);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChar",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
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
    pub fn get_width(&self, text: impl Into<String>) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Li32;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(text.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWidth",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn height(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()Li32;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn is_valid(&self, text: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lbool;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(text.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isValid",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct MapFontCharacterSprite<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapFontCharacterSprite<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapFontCharacterSprite<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate MapFontCharacterSprite from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapFont/CharacterSprite")?;
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
}

impl<'mc> MapFontCharacterSprite<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        width: i32,
        height: i32,
        data: bool,
    ) -> Result<crate::map::MapFontCharacterSprite<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(IIZ)V");
        let val_1 = jni::objects::JValueGen::Int(width);
        let val_2 = jni::objects::JValueGen::Int(height);
        let val_3 = jni::objects::JValueGen::Bool(data.into());
        let cls = jni.find_class("org/bukkit/map/MapFont/CharacterSprite");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::map::MapFontCharacterSprite::from_raw(&jni, res)
    }
    pub fn get(&self, row: i32, col: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(II)Lbool;");
        let val_1 = jni::objects::JValueGen::Int(row);
        let val_2 = jni::objects::JValueGen::Int(col);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "get",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn width(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()Li32;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWidth", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn height(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()Li32;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeight", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct MapView<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapView<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapView<'mc> {
    fn from_raw(
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
}

impl<'mc> MapView<'mc> {
    pub fn id(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()Li32;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getId", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn is_virtual(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Lbool;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isVirtual", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn scale(&self) -> Result<crate::map::MapViewScale<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::map::MapViewScale;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::map::MapViewScale::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn set_scale(
        &self,
        scale: impl Into<crate::map::MapViewScale<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/map/MapView/Scale;)L();");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(scale.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn center_x(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()Li32;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCenterX", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn center_z(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()Li32;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCenterZ", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn set_center_x(&self, x: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)L();");
        let val_1 = jni::objects::JValueGen::Int(x);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCenterX",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_center_z(&self, z: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)L();");
        let val_1 = jni::objects::JValueGen::Int(z);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setCenterZ",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn world(&self) -> Result<Option<crate::World<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::World;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWorld", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::World::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })?))
    }
    pub fn set_world(
        &self,
        world: impl Into<crate::World<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/World;)L();");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(world.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWorld",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn renderers(&self) -> Result<Vec<jni::objects::JObject<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()LVec;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getRenderers", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(obj);
        }
        Ok(new_vec)
    }
    pub fn add_renderer(
        &self,
        renderer: impl Into<crate::map::MapRenderer<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/map/MapRenderer;)L();");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(renderer.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addRenderer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn remove_renderer(
        &self,
        renderer: impl Into<crate::map::MapRenderer<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/map/MapRenderer;)Lbool;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(renderer.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeRenderer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn is_tracking_position(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Lbool;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isTrackingPosition",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn set_tracking_position(
        &self,
        tracking_position: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)L();");
        let val_1 = jni::objects::JValueGen::Bool(tracking_position.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setTrackingPosition",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_unlimited_tracking(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Lbool;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isUnlimitedTracking",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn set_unlimited_tracking(
        &self,
        unlimited: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)L();");
        let val_1 = jni::objects::JValueGen::Bool(unlimited.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setUnlimitedTracking",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn is_locked(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Lbool;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isLocked", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    pub fn set_locked(&self, locked: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)L();");
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum MapViewScale<'mc> {
    Closest { inner: MapViewScaleStruct<'mc> },
    Close { inner: MapViewScaleStruct<'mc> },
    Normal { inner: MapViewScaleStruct<'mc> },
    Far { inner: MapViewScaleStruct<'mc> },
    Farthest { inner: MapViewScaleStruct<'mc> },
}
impl<'mc> std::fmt::Display for MapViewScale<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapViewScale::Closest { .. } => f.write_str("CLOSEST"),
            MapViewScale::Close { .. } => f.write_str("CLOSE"),
            MapViewScale::Normal { .. } => f.write_str("NORMAL"),
            MapViewScale::Far { .. } => f.write_str("FAR"),
            MapViewScale::Farthest { .. } => f.write_str("FARTHEST"),
        }
    }
}

impl<'mc> MapViewScale<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<MapViewScale<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/map/MapView/Scale");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/map/MapView/Scale;",
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
            "CLOSEST" => Ok(MapViewScale::Closest {
                inner: MapViewScaleStruct::from_raw(env, obj)?,
            }),
            "CLOSE" => Ok(MapViewScale::Close {
                inner: MapViewScaleStruct::from_raw(env, obj)?,
            }),
            "NORMAL" => Ok(MapViewScale::Normal {
                inner: MapViewScaleStruct::from_raw(env, obj)?,
            }),
            "FAR" => Ok(MapViewScale::Far {
                inner: MapViewScaleStruct::from_raw(env, obj)?,
            }),
            "FARTHEST" => Ok(MapViewScale::Farthest {
                inner: MapViewScaleStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct MapViewScaleStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for MapViewScale<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Closest { inner } => inner.0.clone(),
            Self::Close { inner } => inner.0.clone(),
            Self::Normal { inner } => inner.0.clone(),
            Self::Far { inner } => inner.0.clone(),
            Self::Farthest { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Closest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Close { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Normal { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Far { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Farthest { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapViewScale<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate MapViewScale from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapView/Scale")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapViewScale object, got {}",
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
                "CLOSEST" => Ok(MapViewScale::Closest {
                    inner: MapViewScaleStruct::from_raw(env, obj)?,
                }),
                "CLOSE" => Ok(MapViewScale::Close {
                    inner: MapViewScaleStruct::from_raw(env, obj)?,
                }),
                "NORMAL" => Ok(MapViewScale::Normal {
                    inner: MapViewScaleStruct::from_raw(env, obj)?,
                }),
                "FAR" => Ok(MapViewScale::Far {
                    inner: MapViewScaleStruct::from_raw(env, obj)?,
                }),
                "FARTHEST" => Ok(MapViewScale::Farthest {
                    inner: MapViewScaleStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for MapViewScaleStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for MapViewScaleStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate MapViewScaleStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/map/MapView/Scale")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a MapViewScaleStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> MapViewScaleStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::map::MapViewScale<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::map::MapViewScale;");
        let cls = jni.find_class("org/bukkit/map/MapView/Scale");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::map::MapViewScale::from_raw(&jni, obj)
    }
    #[deprecated]

    pub fn value(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()Li8;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
