#![allow(deprecated)]
use crate::JNIRaw;
pub struct SimplexNoiseGenerator<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for SimplexNoiseGenerator<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SimplexNoiseGenerator<'mc> {
    pub fn new_with_world(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i64>,
    ) -> Result<crate::bukkit::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator")?;
        let res = jni.new_object(cls, "(J)V", &[jni::objects::JValueGen::from(&val_0)])?;
        let ret = { crate::bukkit::util::noise::SimplexNoiseGenerator(jni, res) };
        Ok(ret)
    }
    pub fn new_with_random(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::bukkit::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = arg0.unwrap();
        let cls = &jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Random;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::util::noise::SimplexNoiseGenerator(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate SimplexNoiseGenerator from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SimplexNoiseGenerator") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SimplexNoiseGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.into());
        let val_4 = jni::objects::JValueGen::Double(arg4.into());
        let val_5 = jni::objects::JValueGen::Double(arg5.into());
        // 6
        let val_6 = jni::objects::JValueGen::Bool(arg6.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDIDDZ)D",
            &[
                jni::objects::JValueGen::from(&val_0),
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
    pub fn get_noise_with_double(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: std::option::Option<f64>,
        arg5: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.into());
        let val_4 = jni::objects::JValueGen::Double(arg4.unwrap().into());
        let val_5 = jni::objects::JValueGen::Double(arg5.unwrap().into());
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(
            cls,
            "getNoise",
            "(DDDIDD)D",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
            ],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn instance(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>>
    {
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator")?;
        let res = jni.call_static_method(
            cls,
            "getInstance",
            "()Lorg/bukkit/util/noise/PerlinNoiseGenerator;",
            &[],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::noise::PerlinNoiseGenerator(jni, obj)
        };
        Ok(ret)
    }
    pub fn floor(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "floor",
            "(D)I",
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
pub struct PerlinNoiseGenerator<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PerlinNoiseGenerator<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PerlinNoiseGenerator<'mc> {
    pub fn new_with_world(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::bukkit::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = arg0.unwrap();
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Random;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = { crate::bukkit::util::noise::PerlinNoiseGenerator(jni, res) };
        Ok(ret)
    }
    pub fn new_with_long(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i64>,
    ) -> Result<crate::bukkit::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator")?;
        let res = jni.new_object(cls, "(J)V", &[jni::objects::JValueGen::from(&val_0)])?;
        let ret = { crate::bukkit::util::noise::PerlinNoiseGenerator(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PerlinNoiseGenerator from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PerlinNoiseGenerator") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PerlinNoiseGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.into());
        let val_4 = jni::objects::JValueGen::Double(arg4.into());
        let val_5 = jni::objects::JValueGen::Double(arg5.into());
        // 6
        let val_6 = jni::objects::JValueGen::Bool(arg6.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDIDDZ)D",
            &[
                jni::objects::JValueGen::from(&val_0),
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
    pub fn get_noise_with_double(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: std::option::Option<f64>,
        arg5: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.into());
        let val_4 = jni::objects::JValueGen::Double(arg4.unwrap().into());
        let val_5 = jni::objects::JValueGen::Double(arg5.unwrap().into());
        let cls = &jni.find_class("double")?;
        let res = jni.call_static_method(
            cls,
            "getNoise",
            "(DDDIDD)D",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
            ],
        )?;
        Ok(res.d().unwrap())
    }
    pub fn instance(
        jni: crate::SharedJNIEnv<'mc>,
    ) -> Result<crate::bukkit::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>>
    {
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator")?;
        let res = jni.call_static_method(
            cls,
            "getInstance",
            "()Lorg/bukkit/util/noise/PerlinNoiseGenerator;",
            &[],
        )?;
        let ret = {
            let obj = res.l()?;
            crate::bukkit::util::noise::PerlinNoiseGenerator(jni, obj)
        };
        Ok(ret)
    }
    pub fn floor(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "floor",
            "(D)I",
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
pub struct SimplexOctaveGenerator<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for SimplexOctaveGenerator<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SimplexOctaveGenerator<'mc> {
    pub fn new_with_world(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::bukkit::util::noise::SimplexOctaveGenerator<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JValueGen::Long(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/SimplexOctaveGenerator")?;
        let res = jni.new_object(
            cls,
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::util::noise::SimplexOctaveGenerator(jni, res) };
        Ok(ret)
    }
    pub fn new_with_random(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::bukkit::util::noise::SimplexOctaveGenerator<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = arg0;
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/SimplexOctaveGenerator")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Random;I)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::util::noise::SimplexOctaveGenerator(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate SimplexOctaveGenerator from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("SimplexOctaveGenerator") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SimplexOctaveGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_scale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
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
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let val_3 = jni::objects::JValueGen::Double(arg3.into());
        let val_4 = jni::objects::JValueGen::Double(arg4.into());
        let val_5 = jni::objects::JValueGen::Double(arg5.into());
        // 6
        let val_6 = jni::objects::JValueGen::Bool(arg6.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDDDDZ)D",
            &[
                jni::objects::JValueGen::from(&val_0),
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
    pub fn set_wscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setWScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn wscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWScale", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn set_xscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setXScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_yscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setYScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_zscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setZScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn xscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getXScale", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn yscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYScale", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn zscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZScale", "()D", &[])?;
        Ok(res.d().unwrap())
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
pub struct OctaveGenerator<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for OctaveGenerator<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> OctaveGenerator<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate OctaveGenerator from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("OctaveGenerator") {
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
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_xscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setXScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_yscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setYScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_zscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setZScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn xscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getXScale", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn yscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYScale", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn zscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZScale", "()D", &[])?;
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
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let val_3 = jni::objects::JValueGen::Double(arg3.into());
        let val_4 = jni::objects::JValueGen::Double(arg4.into());
        // 5
        let val_5 = jni::objects::JValueGen::Bool(arg5.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDDDZ)D",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
            ],
        )?;
        Ok(res.d().unwrap())
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
pub struct PerlinOctaveGenerator<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for PerlinOctaveGenerator<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PerlinOctaveGenerator<'mc> {
    pub fn new_with_world(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::bukkit::util::noise::PerlinOctaveGenerator<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = arg0;
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinOctaveGenerator")?;
        let res = jni.new_object(
            cls,
            "(Ljava/util/Random;I)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::util::noise::PerlinOctaveGenerator(jni, res) };
        Ok(ret)
    }
    pub fn new_with_long(
        jni: crate::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::bukkit::util::noise::PerlinOctaveGenerator<'mc>, Box<dyn std::error::Error>>
    {
        let val_0 = jni::objects::JValueGen::Long(arg0.into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let cls = &jni.find_class("org/bukkit/util/noise/PerlinOctaveGenerator")?;
        let res = jni.new_object(
            cls,
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        let ret = { crate::bukkit::util::noise::PerlinOctaveGenerator(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PerlinOctaveGenerator from null object."
            )
            .into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("PerlinOctaveGenerator") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PerlinOctaveGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn set_scale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_xscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setXScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_yscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setYScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_zscale(&mut self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setZScale",
            "(D)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn xscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getXScale", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn yscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYScale", "()D", &[])?;
        Ok(res.d().unwrap())
    }
    pub fn zscale(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZScale", "()D", &[])?;
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
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let val_3 = jni::objects::JValueGen::Double(arg3.into());
        let val_4 = jni::objects::JValueGen::Double(arg4.into());
        // 5
        let val_5 = jni::objects::JValueGen::Bool(arg5.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDDDZ)D",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
                jni::objects::JValueGen::from(&val_4),
                jni::objects::JValueGen::from(&val_5),
            ],
        )?;
        Ok(res.d().unwrap())
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
pub struct NoiseGenerator<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for NoiseGenerator<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> NoiseGenerator<'mc> {
    pub fn from_extendable(
        _plugin: &crate::bukkit::plugin::Plugin,
        env: &crate::SharedJNIEnv<'mc>,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = jni::objects::JValueGen::Object(env.new_object(
            "net/ioixd/blackbox/extendables/ExtendableNoiseGenerator",
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
    ) -> Result<crate::bukkit::util::noise::NoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/util/noise/NoiseGenerator")?;
        let res = jni.new_object(cls, "()V", &[])?;
        let ret = { crate::bukkit::util::noise::NoiseGenerator(jni, res) };
        Ok(ret)
    }
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate NoiseGenerator from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("NoiseGenerator") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a NoiseGenerator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
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
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let val_1 = jni::objects::JValueGen::Double(arg1.into());
        let val_2 = jni::objects::JValueGen::Double(arg2.into());
        let val_3 = jni::objects::JValueGen::Int(arg3.into());
        let val_4 = jni::objects::JValueGen::Double(arg4.into());
        let val_5 = jni::objects::JValueGen::Double(arg5.into());
        // 6
        let val_6 = jni::objects::JValueGen::Bool(arg6.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            "(DDDIDDZ)D",
            &[
                jni::objects::JValueGen::from(&val_0),
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
        jni: crate::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Double(arg0.into());
        let cls = &jni.find_class("int")?;
        let res = jni.call_static_method(
            cls,
            "floor",
            "(D)I",
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
