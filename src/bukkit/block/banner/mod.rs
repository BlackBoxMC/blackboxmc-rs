use crate::JNIRaw;
pub enum PatternTypeEnum {
    Base,
    SquareBottomLeft,
    SquareBottomRight,
    SquareTopLeft,
    SquareTopRight,
    StripeBottom,
    StripeTop,
    StripeLeft,
    StripeRight,
    StripeCenter,
    StripeMiddle,
    StripeDownright,
    StripeDownleft,
    StripeSmall,
    Cross,
    StraightCross,
    TriangleBottom,
    TriangleTop,
    TrianglesBottom,
    TrianglesTop,
    DiagonalLeft,
    DiagonalRight,
    DiagonalLeftMirror,
    DiagonalRightMirror,
    CircleMiddle,
    RhombusMiddle,
    HalfVertical,
    HalfHorizontal,
    HalfVerticalMirror,
    HalfHorizontalMirror,
    Border,
    CurlyBorder,
    Creeper,
    Gradient,
    GradientUp,
    Bricks,
    Skull,
    Flower,
    Mojang,
    Globe,
    Piglin,
}
impl std::fmt::Display for PatternTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PatternTypeEnum::Base => f.write_str("BASE"),
            PatternTypeEnum::SquareBottomLeft => f.write_str("SQUARE_BOTTOM_LEFT"),
            PatternTypeEnum::SquareBottomRight => f.write_str("SQUARE_BOTTOM_RIGHT"),
            PatternTypeEnum::SquareTopLeft => f.write_str("SQUARE_TOP_LEFT"),
            PatternTypeEnum::SquareTopRight => f.write_str("SQUARE_TOP_RIGHT"),
            PatternTypeEnum::StripeBottom => f.write_str("STRIPE_BOTTOM"),
            PatternTypeEnum::StripeTop => f.write_str("STRIPE_TOP"),
            PatternTypeEnum::StripeLeft => f.write_str("STRIPE_LEFT"),
            PatternTypeEnum::StripeRight => f.write_str("STRIPE_RIGHT"),
            PatternTypeEnum::StripeCenter => f.write_str("STRIPE_CENTER"),
            PatternTypeEnum::StripeMiddle => f.write_str("STRIPE_MIDDLE"),
            PatternTypeEnum::StripeDownright => f.write_str("STRIPE_DOWNRIGHT"),
            PatternTypeEnum::StripeDownleft => f.write_str("STRIPE_DOWNLEFT"),
            PatternTypeEnum::StripeSmall => f.write_str("STRIPE_SMALL"),
            PatternTypeEnum::Cross => f.write_str("CROSS"),
            PatternTypeEnum::StraightCross => f.write_str("STRAIGHT_CROSS"),
            PatternTypeEnum::TriangleBottom => f.write_str("TRIANGLE_BOTTOM"),
            PatternTypeEnum::TriangleTop => f.write_str("TRIANGLE_TOP"),
            PatternTypeEnum::TrianglesBottom => f.write_str("TRIANGLES_BOTTOM"),
            PatternTypeEnum::TrianglesTop => f.write_str("TRIANGLES_TOP"),
            PatternTypeEnum::DiagonalLeft => f.write_str("DIAGONAL_LEFT"),
            PatternTypeEnum::DiagonalRight => f.write_str("DIAGONAL_RIGHT"),
            PatternTypeEnum::DiagonalLeftMirror => f.write_str("DIAGONAL_LEFT_MIRROR"),
            PatternTypeEnum::DiagonalRightMirror => f.write_str("DIAGONAL_RIGHT_MIRROR"),
            PatternTypeEnum::CircleMiddle => f.write_str("CIRCLE_MIDDLE"),
            PatternTypeEnum::RhombusMiddle => f.write_str("RHOMBUS_MIDDLE"),
            PatternTypeEnum::HalfVertical => f.write_str("HALF_VERTICAL"),
            PatternTypeEnum::HalfHorizontal => f.write_str("HALF_HORIZONTAL"),
            PatternTypeEnum::HalfVerticalMirror => f.write_str("HALF_VERTICAL_MIRROR"),
            PatternTypeEnum::HalfHorizontalMirror => f.write_str("HALF_HORIZONTAL_MIRROR"),
            PatternTypeEnum::Border => f.write_str("BORDER"),
            PatternTypeEnum::CurlyBorder => f.write_str("CURLY_BORDER"),
            PatternTypeEnum::Creeper => f.write_str("CREEPER"),
            PatternTypeEnum::Gradient => f.write_str("GRADIENT"),
            PatternTypeEnum::GradientUp => f.write_str("GRADIENT_UP"),
            PatternTypeEnum::Bricks => f.write_str("BRICKS"),
            PatternTypeEnum::Skull => f.write_str("SKULL"),
            PatternTypeEnum::Flower => f.write_str("FLOWER"),
            PatternTypeEnum::Mojang => f.write_str("MOJANG"),
            PatternTypeEnum::Globe => f.write_str("GLOBE"),
            PatternTypeEnum::Piglin => f.write_str("PIGLIN"),
        }
    }
}
pub struct PatternType<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub PatternTypeEnum,
);
impl<'mc> std::ops::Deref for PatternType<'mc> {
    type Target = PatternTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> crate::JNIRaw<'mc> for PatternType<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PatternType<'mc> {
    pub fn from_string(str: String) -> std::option::Option<PatternTypeEnum> {
        match str.as_str() {
            "BASE" => Some(PatternTypeEnum::Base),
            "SQUARE_BOTTOM_LEFT" => Some(PatternTypeEnum::SquareBottomLeft),
            "SQUARE_BOTTOM_RIGHT" => Some(PatternTypeEnum::SquareBottomRight),
            "SQUARE_TOP_LEFT" => Some(PatternTypeEnum::SquareTopLeft),
            "SQUARE_TOP_RIGHT" => Some(PatternTypeEnum::SquareTopRight),
            "STRIPE_BOTTOM" => Some(PatternTypeEnum::StripeBottom),
            "STRIPE_TOP" => Some(PatternTypeEnum::StripeTop),
            "STRIPE_LEFT" => Some(PatternTypeEnum::StripeLeft),
            "STRIPE_RIGHT" => Some(PatternTypeEnum::StripeRight),
            "STRIPE_CENTER" => Some(PatternTypeEnum::StripeCenter),
            "STRIPE_MIDDLE" => Some(PatternTypeEnum::StripeMiddle),
            "STRIPE_DOWNRIGHT" => Some(PatternTypeEnum::StripeDownright),
            "STRIPE_DOWNLEFT" => Some(PatternTypeEnum::StripeDownleft),
            "STRIPE_SMALL" => Some(PatternTypeEnum::StripeSmall),
            "CROSS" => Some(PatternTypeEnum::Cross),
            "STRAIGHT_CROSS" => Some(PatternTypeEnum::StraightCross),
            "TRIANGLE_BOTTOM" => Some(PatternTypeEnum::TriangleBottom),
            "TRIANGLE_TOP" => Some(PatternTypeEnum::TriangleTop),
            "TRIANGLES_BOTTOM" => Some(PatternTypeEnum::TrianglesBottom),
            "TRIANGLES_TOP" => Some(PatternTypeEnum::TrianglesTop),
            "DIAGONAL_LEFT" => Some(PatternTypeEnum::DiagonalLeft),
            "DIAGONAL_RIGHT" => Some(PatternTypeEnum::DiagonalRight),
            "DIAGONAL_LEFT_MIRROR" => Some(PatternTypeEnum::DiagonalLeftMirror),
            "DIAGONAL_RIGHT_MIRROR" => Some(PatternTypeEnum::DiagonalRightMirror),
            "CIRCLE_MIDDLE" => Some(PatternTypeEnum::CircleMiddle),
            "RHOMBUS_MIDDLE" => Some(PatternTypeEnum::RhombusMiddle),
            "HALF_VERTICAL" => Some(PatternTypeEnum::HalfVertical),
            "HALF_HORIZONTAL" => Some(PatternTypeEnum::HalfHorizontal),
            "HALF_VERTICAL_MIRROR" => Some(PatternTypeEnum::HalfVerticalMirror),
            "HALF_HORIZONTAL_MIRROR" => Some(PatternTypeEnum::HalfHorizontalMirror),
            "BORDER" => Some(PatternTypeEnum::Border),
            "CURLY_BORDER" => Some(PatternTypeEnum::CurlyBorder),
            "CREEPER" => Some(PatternTypeEnum::Creeper),
            "GRADIENT" => Some(PatternTypeEnum::Gradient),
            "GRADIENT_UP" => Some(PatternTypeEnum::GradientUp),
            "BRICKS" => Some(PatternTypeEnum::Bricks),
            "SKULL" => Some(PatternTypeEnum::Skull),
            "FLOWER" => Some(PatternTypeEnum::Flower),
            "MOJANG" => Some(PatternTypeEnum::Mojang),
            "GLOBE" => Some(PatternTypeEnum::Globe),
            "PIGLIN" => Some(PatternTypeEnum::Piglin),
            _ => None,
        }
    }
    pub fn identifier(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIdentifier",
            "()Ljava/lang/String;",
            &[],
        )?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn get_by_identifier(
        mut jni: jni::JNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::block::banner::PatternType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/block/banner/PatternType")?;
        let res = jni.call_static_method(
            cls,
            "getByIdentifier",
            "(Ljava/lang/String;)Lorg/bukkit/block/banner/PatternType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::banner::PatternType(
                jni,
                raw_obj,
                crate::bukkit::block::banner::PatternType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn value_of(
        mut jni: jni::JNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::block::banner::PatternType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/block/banner/PatternType")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/banner/PatternType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::banner::PatternType(
                jni,
                raw_obj,
                crate::bukkit::block::banner::PatternType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
}
pub struct Pattern<'mc>(
    pub(crate) jni::JNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for Pattern<'mc> {
    fn jni_ref(&self) -> jni::JNIEnv<'mc> {
        unsafe { self.0.unsafe_clone() }
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Pattern<'mc> {
    pub fn from_raw(env: jni::JNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Self {
        Self(env, obj)
    }
    pub fn color(&mut self) -> Result<crate::bukkit::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::DyeColor(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::DyeColor::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn pattern(
        &mut self,
    ) -> Result<crate::bukkit::block::banner::PatternType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPattern",
            "()Lorg/bukkit/block/banner/PatternType;",
            &[],
        )?;
        let ret = {
            let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
            let variant = self
                .0
                .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = self
                .0
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::block::banner::PatternType(
                self.jni_ref(),
                raw_obj,
                crate::bukkit::block::banner::PatternType::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
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
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[])?;
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
