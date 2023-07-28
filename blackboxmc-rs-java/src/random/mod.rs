#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub struct JavaRandomGeneratorFactory<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for JavaRandomGeneratorFactory<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JavaRandomGeneratorFactory<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRandomGeneratorFactory from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaRandomGeneratorFactory")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaRandomGeneratorFactory object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
/// An instantiatable struct that implements JavaRandomGeneratorSplittableGenerator. Needed for returning it from Java.
pub struct JavaRandomGeneratorSplittableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGeneratorSplittableGenerator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRandomGeneratorSplittableGenerator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaRandomGeneratorSplittableGenerator")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaRandomGeneratorSplittableGenerator object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorSplittableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::random::JavaRandomGeneratorStreamableGenerator<'mc>>
    for JavaRandomGeneratorSplittableGenerator<'mc>
{
    fn into(self) -> crate::random::JavaRandomGeneratorStreamableGenerator<'mc> {
        crate::random::JavaRandomGeneratorStreamableGenerator::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
/// An instantiatable struct that implements JavaRandomGeneratorStreamableGenerator. Needed for returning it from Java.
pub struct JavaRandomGeneratorStreamableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGeneratorStreamableGenerator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRandomGeneratorStreamableGenerator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaRandomGeneratorStreamableGenerator")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaRandomGeneratorStreamableGenerator object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorStreamableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaRandomGeneratorLeapableGenerator. Needed for returning it from Java.
pub struct JavaRandomGeneratorLeapableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGeneratorLeapableGenerator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRandomGeneratorLeapableGenerator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaRandomGeneratorLeapableGenerator")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaRandomGeneratorLeapableGenerator object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorLeapableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::random::JavaRandomGeneratorJumpableGenerator<'mc>>
    for JavaRandomGeneratorLeapableGenerator<'mc>
{
    fn into(self) -> crate::random::JavaRandomGeneratorJumpableGenerator<'mc> {
        crate::random::JavaRandomGeneratorJumpableGenerator::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
/// An instantiatable struct that implements JavaRandomGenerator. Needed for returning it from Java.
pub struct JavaRandomGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGenerator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaRandomGenerator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "JavaRandomGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaRandomGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements JavaRandomGeneratorArbitrarilyJumpableGenerator. Needed for returning it from Java.
pub struct JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
        "Tried to instantiate JavaRandomGeneratorArbitrarilyJumpableGenerator from null object.")
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "JavaRandomGeneratorArbitrarilyJumpableGenerator")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaRandomGeneratorArbitrarilyJumpableGenerator object, got {}",
        name
    )
    .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::random::JavaRandomGeneratorLeapableGenerator<'mc>>
    for JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc>
{
    fn into(self) -> crate::random::JavaRandomGeneratorLeapableGenerator<'mc> {
        crate::random::JavaRandomGeneratorLeapableGenerator::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
/// An instantiatable struct that implements JavaRandomGeneratorJumpableGenerator. Needed for returning it from Java.
pub struct JavaRandomGeneratorJumpableGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> JavaRandomGeneratorJumpableGenerator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaRandomGeneratorJumpableGenerator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "JavaRandomGeneratorJumpableGenerator")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a JavaRandomGeneratorJumpableGenerator object, got {}",
        name
    )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for JavaRandomGeneratorJumpableGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::random::JavaRandomGeneratorStreamableGenerator<'mc>>
    for JavaRandomGeneratorJumpableGenerator<'mc>
{
    fn into(self) -> crate::random::JavaRandomGeneratorStreamableGenerator<'mc> {
        crate::random::JavaRandomGeneratorStreamableGenerator::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
