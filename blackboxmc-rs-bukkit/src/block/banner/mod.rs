#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct Pattern<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Pattern<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Pattern<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Pattern from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/banner/Pattern")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Pattern object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Pattern<'mc> {
    pub fn new_with_color(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        color: impl Into<crate::DyeColor<'mc>>,
        pattern: std::option::Option<impl Into<crate::block::banner::PatternType<'mc>>>,
    ) -> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/DyeColor;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(color.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = pattern {
            sig += "Lorg/bukkit/block/banner/PatternType;";
            let val_2 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/block/banner/Pattern");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::block::banner::Pattern::from_raw(&jni, res)
    }
    pub fn serialize(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/Map;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "serialize", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn color(&self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::DyeColor;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getColor", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::DyeColor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn pattern(
        &self,
    ) -> Result<crate::block::banner::PatternType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::block::banner::PatternType;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPattern", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::block::banner::PatternType::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()Li32;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    pub fn equals(
        &self,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Lbool;");
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for Pattern<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(&self.jni_ref(), self.1).expect("Error converting Pattern into crate::configuration::serialization::ConfigurationSerializable")
    }
}
pub enum PatternType<'mc> {
    Base { inner: PatternTypeStruct<'mc> },
    SquareBottomLeft { inner: PatternTypeStruct<'mc> },
    SquareBottomRight { inner: PatternTypeStruct<'mc> },
    SquareTopLeft { inner: PatternTypeStruct<'mc> },
    SquareTopRight { inner: PatternTypeStruct<'mc> },
    StripeBottom { inner: PatternTypeStruct<'mc> },
    StripeTop { inner: PatternTypeStruct<'mc> },
    StripeLeft { inner: PatternTypeStruct<'mc> },
    StripeRight { inner: PatternTypeStruct<'mc> },
    StripeCenter { inner: PatternTypeStruct<'mc> },
    StripeMiddle { inner: PatternTypeStruct<'mc> },
    StripeDownright { inner: PatternTypeStruct<'mc> },
    StripeDownleft { inner: PatternTypeStruct<'mc> },
    StripeSmall { inner: PatternTypeStruct<'mc> },
    Cross { inner: PatternTypeStruct<'mc> },
    StraightCross { inner: PatternTypeStruct<'mc> },
    TriangleBottom { inner: PatternTypeStruct<'mc> },
    TriangleTop { inner: PatternTypeStruct<'mc> },
    TrianglesBottom { inner: PatternTypeStruct<'mc> },
    TrianglesTop { inner: PatternTypeStruct<'mc> },
    DiagonalLeft { inner: PatternTypeStruct<'mc> },
    DiagonalRight { inner: PatternTypeStruct<'mc> },
    DiagonalLeftMirror { inner: PatternTypeStruct<'mc> },
    DiagonalRightMirror { inner: PatternTypeStruct<'mc> },
    CircleMiddle { inner: PatternTypeStruct<'mc> },
    RhombusMiddle { inner: PatternTypeStruct<'mc> },
    HalfVertical { inner: PatternTypeStruct<'mc> },
    HalfHorizontal { inner: PatternTypeStruct<'mc> },
    HalfVerticalMirror { inner: PatternTypeStruct<'mc> },
    HalfHorizontalMirror { inner: PatternTypeStruct<'mc> },
    Border { inner: PatternTypeStruct<'mc> },
    CurlyBorder { inner: PatternTypeStruct<'mc> },
    Creeper { inner: PatternTypeStruct<'mc> },
    Gradient { inner: PatternTypeStruct<'mc> },
    GradientUp { inner: PatternTypeStruct<'mc> },
    Bricks { inner: PatternTypeStruct<'mc> },
    Skull { inner: PatternTypeStruct<'mc> },
    Flower { inner: PatternTypeStruct<'mc> },
    Mojang { inner: PatternTypeStruct<'mc> },
    Globe { inner: PatternTypeStruct<'mc> },
    Piglin { inner: PatternTypeStruct<'mc> },
}
impl<'mc> std::fmt::Display for PatternType<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternType::Base { .. } => f.write_str("BASE"),
            PatternType::SquareBottomLeft { .. } => f.write_str("SQUARE_BOTTOM_LEFT"),
            PatternType::SquareBottomRight { .. } => f.write_str("SQUARE_BOTTOM_RIGHT"),
            PatternType::SquareTopLeft { .. } => f.write_str("SQUARE_TOP_LEFT"),
            PatternType::SquareTopRight { .. } => f.write_str("SQUARE_TOP_RIGHT"),
            PatternType::StripeBottom { .. } => f.write_str("STRIPE_BOTTOM"),
            PatternType::StripeTop { .. } => f.write_str("STRIPE_TOP"),
            PatternType::StripeLeft { .. } => f.write_str("STRIPE_LEFT"),
            PatternType::StripeRight { .. } => f.write_str("STRIPE_RIGHT"),
            PatternType::StripeCenter { .. } => f.write_str("STRIPE_CENTER"),
            PatternType::StripeMiddle { .. } => f.write_str("STRIPE_MIDDLE"),
            PatternType::StripeDownright { .. } => f.write_str("STRIPE_DOWNRIGHT"),
            PatternType::StripeDownleft { .. } => f.write_str("STRIPE_DOWNLEFT"),
            PatternType::StripeSmall { .. } => f.write_str("STRIPE_SMALL"),
            PatternType::Cross { .. } => f.write_str("CROSS"),
            PatternType::StraightCross { .. } => f.write_str("STRAIGHT_CROSS"),
            PatternType::TriangleBottom { .. } => f.write_str("TRIANGLE_BOTTOM"),
            PatternType::TriangleTop { .. } => f.write_str("TRIANGLE_TOP"),
            PatternType::TrianglesBottom { .. } => f.write_str("TRIANGLES_BOTTOM"),
            PatternType::TrianglesTop { .. } => f.write_str("TRIANGLES_TOP"),
            PatternType::DiagonalLeft { .. } => f.write_str("DIAGONAL_LEFT"),
            PatternType::DiagonalRight { .. } => f.write_str("DIAGONAL_RIGHT"),
            PatternType::DiagonalLeftMirror { .. } => f.write_str("DIAGONAL_LEFT_MIRROR"),
            PatternType::DiagonalRightMirror { .. } => f.write_str("DIAGONAL_RIGHT_MIRROR"),
            PatternType::CircleMiddle { .. } => f.write_str("CIRCLE_MIDDLE"),
            PatternType::RhombusMiddle { .. } => f.write_str("RHOMBUS_MIDDLE"),
            PatternType::HalfVertical { .. } => f.write_str("HALF_VERTICAL"),
            PatternType::HalfHorizontal { .. } => f.write_str("HALF_HORIZONTAL"),
            PatternType::HalfVerticalMirror { .. } => f.write_str("HALF_VERTICAL_MIRROR"),
            PatternType::HalfHorizontalMirror { .. } => f.write_str("HALF_HORIZONTAL_MIRROR"),
            PatternType::Border { .. } => f.write_str("BORDER"),
            PatternType::CurlyBorder { .. } => f.write_str("CURLY_BORDER"),
            PatternType::Creeper { .. } => f.write_str("CREEPER"),
            PatternType::Gradient { .. } => f.write_str("GRADIENT"),
            PatternType::GradientUp { .. } => f.write_str("GRADIENT_UP"),
            PatternType::Bricks { .. } => f.write_str("BRICKS"),
            PatternType::Skull { .. } => f.write_str("SKULL"),
            PatternType::Flower { .. } => f.write_str("FLOWER"),
            PatternType::Mojang { .. } => f.write_str("MOJANG"),
            PatternType::Globe { .. } => f.write_str("GLOBE"),
            PatternType::Piglin { .. } => f.write_str("PIGLIN"),
        }
    }
}

impl<'mc> PatternType<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PatternType<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/block/banner/PatternType");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/banner/PatternType;",
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
            "BASE" => Ok(PatternType::Base {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "SQUARE_BOTTOM_LEFT" => Ok(PatternType::SquareBottomLeft {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "SQUARE_BOTTOM_RIGHT" => Ok(PatternType::SquareBottomRight {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "SQUARE_TOP_LEFT" => Ok(PatternType::SquareTopLeft {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "SQUARE_TOP_RIGHT" => Ok(PatternType::SquareTopRight {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "STRIPE_BOTTOM" => Ok(PatternType::StripeBottom {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "STRIPE_TOP" => Ok(PatternType::StripeTop {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "STRIPE_LEFT" => Ok(PatternType::StripeLeft {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "STRIPE_RIGHT" => Ok(PatternType::StripeRight {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "STRIPE_CENTER" => Ok(PatternType::StripeCenter {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "STRIPE_MIDDLE" => Ok(PatternType::StripeMiddle {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "STRIPE_DOWNRIGHT" => Ok(PatternType::StripeDownright {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "STRIPE_DOWNLEFT" => Ok(PatternType::StripeDownleft {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "STRIPE_SMALL" => Ok(PatternType::StripeSmall {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "CROSS" => Ok(PatternType::Cross {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "STRAIGHT_CROSS" => Ok(PatternType::StraightCross {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "TRIANGLE_BOTTOM" => Ok(PatternType::TriangleBottom {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "TRIANGLE_TOP" => Ok(PatternType::TriangleTop {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "TRIANGLES_BOTTOM" => Ok(PatternType::TrianglesBottom {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "TRIANGLES_TOP" => Ok(PatternType::TrianglesTop {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "DIAGONAL_LEFT" => Ok(PatternType::DiagonalLeft {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "DIAGONAL_RIGHT" => Ok(PatternType::DiagonalRight {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "DIAGONAL_LEFT_MIRROR" => Ok(PatternType::DiagonalLeftMirror {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "DIAGONAL_RIGHT_MIRROR" => Ok(PatternType::DiagonalRightMirror {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "CIRCLE_MIDDLE" => Ok(PatternType::CircleMiddle {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "RHOMBUS_MIDDLE" => Ok(PatternType::RhombusMiddle {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "HALF_VERTICAL" => Ok(PatternType::HalfVertical {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "HALF_HORIZONTAL" => Ok(PatternType::HalfHorizontal {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "HALF_VERTICAL_MIRROR" => Ok(PatternType::HalfVerticalMirror {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "HALF_HORIZONTAL_MIRROR" => Ok(PatternType::HalfHorizontalMirror {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "BORDER" => Ok(PatternType::Border {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "CURLY_BORDER" => Ok(PatternType::CurlyBorder {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "CREEPER" => Ok(PatternType::Creeper {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "GRADIENT" => Ok(PatternType::Gradient {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "GRADIENT_UP" => Ok(PatternType::GradientUp {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "BRICKS" => Ok(PatternType::Bricks {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "SKULL" => Ok(PatternType::Skull {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "FLOWER" => Ok(PatternType::Flower {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "MOJANG" => Ok(PatternType::Mojang {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "GLOBE" => Ok(PatternType::Globe {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),
            "PIGLIN" => Ok(PatternType::Piglin {
                inner: PatternTypeStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PatternTypeStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PatternType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Base { inner } => inner.0.clone(),
            Self::SquareBottomLeft { inner } => inner.0.clone(),
            Self::SquareBottomRight { inner } => inner.0.clone(),
            Self::SquareTopLeft { inner } => inner.0.clone(),
            Self::SquareTopRight { inner } => inner.0.clone(),
            Self::StripeBottom { inner } => inner.0.clone(),
            Self::StripeTop { inner } => inner.0.clone(),
            Self::StripeLeft { inner } => inner.0.clone(),
            Self::StripeRight { inner } => inner.0.clone(),
            Self::StripeCenter { inner } => inner.0.clone(),
            Self::StripeMiddle { inner } => inner.0.clone(),
            Self::StripeDownright { inner } => inner.0.clone(),
            Self::StripeDownleft { inner } => inner.0.clone(),
            Self::StripeSmall { inner } => inner.0.clone(),
            Self::Cross { inner } => inner.0.clone(),
            Self::StraightCross { inner } => inner.0.clone(),
            Self::TriangleBottom { inner } => inner.0.clone(),
            Self::TriangleTop { inner } => inner.0.clone(),
            Self::TrianglesBottom { inner } => inner.0.clone(),
            Self::TrianglesTop { inner } => inner.0.clone(),
            Self::DiagonalLeft { inner } => inner.0.clone(),
            Self::DiagonalRight { inner } => inner.0.clone(),
            Self::DiagonalLeftMirror { inner } => inner.0.clone(),
            Self::DiagonalRightMirror { inner } => inner.0.clone(),
            Self::CircleMiddle { inner } => inner.0.clone(),
            Self::RhombusMiddle { inner } => inner.0.clone(),
            Self::HalfVertical { inner } => inner.0.clone(),
            Self::HalfHorizontal { inner } => inner.0.clone(),
            Self::HalfVerticalMirror { inner } => inner.0.clone(),
            Self::HalfHorizontalMirror { inner } => inner.0.clone(),
            Self::Border { inner } => inner.0.clone(),
            Self::CurlyBorder { inner } => inner.0.clone(),
            Self::Creeper { inner } => inner.0.clone(),
            Self::Gradient { inner } => inner.0.clone(),
            Self::GradientUp { inner } => inner.0.clone(),
            Self::Bricks { inner } => inner.0.clone(),
            Self::Skull { inner } => inner.0.clone(),
            Self::Flower { inner } => inner.0.clone(),
            Self::Mojang { inner } => inner.0.clone(),
            Self::Globe { inner } => inner.0.clone(),
            Self::Piglin { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Base { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SquareBottomLeft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SquareBottomRight { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SquareTopLeft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SquareTopRight { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StripeBottom { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StripeTop { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StripeLeft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StripeRight { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StripeCenter { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StripeMiddle { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StripeDownright { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StripeDownleft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StripeSmall { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Cross { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::StraightCross { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TriangleBottom { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TriangleTop { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrianglesBottom { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrianglesTop { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DiagonalLeft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DiagonalRight { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DiagonalLeftMirror { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DiagonalRightMirror { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CircleMiddle { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RhombusMiddle { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HalfVertical { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HalfHorizontal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HalfVerticalMirror { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::HalfHorizontalMirror { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Border { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::CurlyBorder { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Creeper { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Gradient { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::GradientUp { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Bricks { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Skull { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Flower { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Mojang { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Globe { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Piglin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PatternType<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PatternType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/banner/PatternType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PatternType object, got {}",
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
                "BASE" => Ok(PatternType::Base {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "SQUARE_BOTTOM_LEFT" => Ok(PatternType::SquareBottomLeft {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "SQUARE_BOTTOM_RIGHT" => Ok(PatternType::SquareBottomRight {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "SQUARE_TOP_LEFT" => Ok(PatternType::SquareTopLeft {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "SQUARE_TOP_RIGHT" => Ok(PatternType::SquareTopRight {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "STRIPE_BOTTOM" => Ok(PatternType::StripeBottom {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "STRIPE_TOP" => Ok(PatternType::StripeTop {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "STRIPE_LEFT" => Ok(PatternType::StripeLeft {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "STRIPE_RIGHT" => Ok(PatternType::StripeRight {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "STRIPE_CENTER" => Ok(PatternType::StripeCenter {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "STRIPE_MIDDLE" => Ok(PatternType::StripeMiddle {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "STRIPE_DOWNRIGHT" => Ok(PatternType::StripeDownright {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "STRIPE_DOWNLEFT" => Ok(PatternType::StripeDownleft {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "STRIPE_SMALL" => Ok(PatternType::StripeSmall {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "CROSS" => Ok(PatternType::Cross {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "STRAIGHT_CROSS" => Ok(PatternType::StraightCross {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "TRIANGLE_BOTTOM" => Ok(PatternType::TriangleBottom {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "TRIANGLE_TOP" => Ok(PatternType::TriangleTop {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "TRIANGLES_BOTTOM" => Ok(PatternType::TrianglesBottom {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "TRIANGLES_TOP" => Ok(PatternType::TrianglesTop {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "DIAGONAL_LEFT" => Ok(PatternType::DiagonalLeft {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "DIAGONAL_RIGHT" => Ok(PatternType::DiagonalRight {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "DIAGONAL_LEFT_MIRROR" => Ok(PatternType::DiagonalLeftMirror {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "DIAGONAL_RIGHT_MIRROR" => Ok(PatternType::DiagonalRightMirror {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "CIRCLE_MIDDLE" => Ok(PatternType::CircleMiddle {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "RHOMBUS_MIDDLE" => Ok(PatternType::RhombusMiddle {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "HALF_VERTICAL" => Ok(PatternType::HalfVertical {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "HALF_HORIZONTAL" => Ok(PatternType::HalfHorizontal {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "HALF_VERTICAL_MIRROR" => Ok(PatternType::HalfVerticalMirror {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "HALF_HORIZONTAL_MIRROR" => Ok(PatternType::HalfHorizontalMirror {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "BORDER" => Ok(PatternType::Border {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "CURLY_BORDER" => Ok(PatternType::CurlyBorder {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "CREEPER" => Ok(PatternType::Creeper {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "GRADIENT" => Ok(PatternType::Gradient {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "GRADIENT_UP" => Ok(PatternType::GradientUp {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "BRICKS" => Ok(PatternType::Bricks {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "SKULL" => Ok(PatternType::Skull {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "FLOWER" => Ok(PatternType::Flower {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "MOJANG" => Ok(PatternType::Mojang {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "GLOBE" => Ok(PatternType::Globe {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                "PIGLIN" => Ok(PatternType::Piglin {
                    inner: PatternTypeStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PatternTypeStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PatternTypeStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PatternTypeStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/block/banner/PatternType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PatternTypeStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PatternTypeStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::block::banner::PatternType<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lcrate::block::banner::PatternType;");
        let cls = jni.find_class("org/bukkit/block/banner/PatternType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::block::banner::PatternType::from_raw(&jni, obj)
    }
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/NamespacedKey;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn identifier(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()LString;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getIdentifier", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]

    pub fn get_by_identifier(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        identifier: impl Into<String>,
    ) -> Result<Option<crate::block::banner::PatternType<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Lcrate::block::banner::PatternType;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(identifier.into())?,
        ));
        let cls = jni.find_class("org/bukkit/block/banner/PatternType");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByIdentifier",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(crate::block::banner::PatternType::from_raw(
            &jni, obj,
        )?))
    }
    // SUPER CLASS: Enum

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
