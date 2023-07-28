#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub struct SimplexNoiseGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SimplexNoiseGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SimplexNoiseGenerator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate SimplexNoiseGenerator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "SimplexNoiseGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SimplexNoiseGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_random(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::World<'mc>>>,
    ) -> Result<crate::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/World;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::util::noise::SimplexNoiseGenerator::from_raw(&jni, res)
    }
    pub fn new_with_long(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i64>,
    ) -> Result<crate::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator")?;
        let res = jni.new_object(cls, "(J)V", &[jni::objects::JValueGen::from(&val_1)])?;
        crate::util::noise::SimplexNoiseGenerator::from_raw(&jni, res)
    }
    pub fn instance(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator")?;
        let res = jni.call_static_method(
            cls,
            "getInstance",
            "()Lorg/bukkit/util/noise/PerlinNoiseGenerator;",
            &[],
        )?;
        let mut obj = res.l()?;
        crate::util::noise::PerlinNoiseGenerator::from_raw(&jni, obj)
    }
    pub fn noise_with_double(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: f64,
        arg5: f64,
        arg6: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        let val_6 = jni::objects::JValueGen::Double(arg5.unwrap().into());
        // 6
        let val_7 = jni::objects::JValueGen::Bool(arg6.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDIDDZ)D",
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
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn get_noise_with_double(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: std::option::Option<f64>,
        arg5: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let val_5 = jni::objects::JValueGen::Double(arg4.unwrap().into());
        let val_6 = jni::objects::JValueGen::Double(arg5.unwrap().into());
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(
            cls,
            "getNoise",
            "(DDDIDD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
            ],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn floor(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "floor",
            "(D)I",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.i().unwrap())
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
impl<'mc> Into<crate::util::noise::PerlinNoiseGenerator<'mc>> for SimplexNoiseGenerator<'mc> {
    fn into(self) -> crate::util::noise::PerlinNoiseGenerator<'mc> {
        crate::util::noise::PerlinNoiseGenerator::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct PerlinNoiseGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PerlinNoiseGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PerlinNoiseGenerator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PerlinNoiseGenerator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "PerlinNoiseGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PerlinNoiseGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_random(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::World<'mc>>>,
    ) -> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/World;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::util::noise::PerlinNoiseGenerator::from_raw(&jni, res)
    }
    pub fn new_with_long(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i64>,
    ) -> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator")?;
        let res = jni.new_object(cls, "(J)V", &[jni::objects::JValueGen::from(&val_1)])?;
        crate::util::noise::PerlinNoiseGenerator::from_raw(&jni, res)
    }
    pub fn instance(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator")?;
        let res = jni.call_static_method(
            cls,
            "getInstance",
            "()Lorg/bukkit/util/noise/PerlinNoiseGenerator;",
            &[],
        )?;
        let mut obj = res.l()?;
        crate::util::noise::PerlinNoiseGenerator::from_raw(&jni, obj)
    }
    pub fn noise_with_double(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: f64,
        arg5: f64,
        arg6: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        let val_6 = jni::objects::JValueGen::Double(arg5.unwrap().into());
        // 6
        let val_7 = jni::objects::JValueGen::Bool(arg6.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDIDDZ)D",
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
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn get_noise_with_double(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: std::option::Option<f64>,
        arg5: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let val_5 = jni::objects::JValueGen::Double(arg4.unwrap().into());
        let val_6 = jni::objects::JValueGen::Double(arg5.unwrap().into());
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(
            cls,
            "getNoise",
            "(DDDIDD)D",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
                jni::objects::JValueGen::from(&val_6),
            ],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn floor(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "floor",
            "(D)I",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.i().unwrap())
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
impl<'mc> Into<crate::util::noise::NoiseGenerator<'mc>> for PerlinNoiseGenerator<'mc> {
    fn into(self) -> crate::util::noise::NoiseGenerator<'mc> {
        crate::util::noise::NoiseGenerator::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct SimplexOctaveGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SimplexOctaveGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SimplexOctaveGenerator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate SimplexOctaveGenerator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "SimplexOctaveGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SimplexOctaveGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_world(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc blackboxmc_java::JavaRandom<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::util::noise::SimplexOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/SimplexOctaveGenerator")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Random;I)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::util::noise::SimplexOctaveGenerator::from_raw(&jni, res)
    }
    pub fn new_with_long(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::util::noise::SimplexOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/SimplexOctaveGenerator")?;
        let res = jni.new_object(
            cls,
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::util::noise::SimplexOctaveGenerator::from_raw(&jni, res)
    }
    pub fn set_scale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn noise_with_double(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = jni::objects::JValueGen::Double(arg3.into());
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        let val_6 = jni::objects::JValueGen::Double(arg5.unwrap().into());
        // 6
        let val_7 = jni::objects::JValueGen::Bool(arg6.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDDDDZ)D",
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
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn set_wscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn wscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWScale", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn set_xscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setXScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_yscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_zscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn xscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getXScale", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn yscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYScale", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn zscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZScale", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
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
impl<'mc> Into<crate::util::noise::OctaveGenerator<'mc>> for SimplexOctaveGenerator<'mc> {
    fn into(self) -> crate::util::noise::OctaveGenerator<'mc> {
        crate::util::noise::OctaveGenerator::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct OctaveGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for OctaveGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> OctaveGenerator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate OctaveGenerator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "OctaveGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a OctaveGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_scale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_xscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setXScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_yscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_zscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn xscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getXScale", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn yscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYScale", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn zscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZScale", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn noise_with_double(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = jni::objects::JValueGen::Double(arg3.into());
        let val_5 = jni::objects::JValueGen::Double(arg4.unwrap().into());
        // 5
        let val_6 = jni::objects::JValueGen::Bool(arg5.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDDDZ)D",
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
        Ok(res.d().unwrap())
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
pub struct PerlinOctaveGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PerlinOctaveGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PerlinOctaveGenerator<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PerlinOctaveGenerator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "PerlinOctaveGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PerlinOctaveGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_random(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::util::noise::PerlinOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinOctaveGenerator")?;
        let res = jni.new_object(
            cls,
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::util::noise::PerlinOctaveGenerator::from_raw(&jni, res)
    }
    pub fn new_with_world(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::World<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::util::noise::PerlinOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_2 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinOctaveGenerator")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/World;I)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::util::noise::PerlinOctaveGenerator::from_raw(&jni, res)
    }
    pub fn set_scale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_xscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setXScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_yscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn set_zscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn xscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getXScale", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn yscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYScale", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn zscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZScale", "()D", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
    }
    pub fn noise_with_double(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = jni::objects::JValueGen::Double(arg3.into());
        let val_5 = jni::objects::JValueGen::Double(arg4.unwrap().into());
        // 5
        let val_6 = jni::objects::JValueGen::Bool(arg5.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDDDZ)D",
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
        Ok(res.d().unwrap())
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
impl<'mc> Into<crate::util::noise::OctaveGenerator<'mc>> for PerlinOctaveGenerator<'mc> {
    fn into(self) -> crate::util::noise::OctaveGenerator<'mc> {
        crate::util::noise::OctaveGenerator::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct NoiseGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for NoiseGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> NoiseGenerator<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "NoiseGenerator", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate NoiseGenerator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "NoiseGenerator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a NoiseGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::noise::NoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/util/noise/NoiseGenerator")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::util::noise::NoiseGenerator::from_raw(&jni, res)
    }
    pub fn floor(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "floor",
            "(D)I",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        Ok(res.i().unwrap())
    }
    pub fn noise_with_double(
        &mut self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: f64,
        arg5: f64,
        arg6: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        let val_6 = jni::objects::JValueGen::Double(arg5.unwrap().into());
        // 6
        let val_7 = jni::objects::JValueGen::Bool(arg6.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDIDDZ)D",
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
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d().unwrap())
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
