#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Generates simplex-based noise.
/// <p>This is a modified version of the freely published version in the paper by Stefan Gustavson at <a href="http://staffwww.itn.liu.se/~stegu/simplexnoise/simplexnoise.pdf"> http://staffwww.itn.liu.se/~stegu/simplexnoise/simplexnoise.pdf</a></p>
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
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/util/noise/SimplexNoiseGenerator")?;
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
    pub fn new_with_world(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i64>,
    ) -> Result<crate::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::SimplexNoiseGenerator::from_raw(&jni, res)
    }
    pub fn new_with_random(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<blackboxmc_java::JavaRandom<'mc>>>,
    ) -> Result<crate::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/Random;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::SimplexNoiseGenerator::from_raw(&jni, res)
    }
    //

    pub fn noise_with_double(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: f64,
        arg5: f64,
        arg6: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        args.push(val_3);
        sig += "I";
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        args.push(val_4);
        sig += "D";
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        args.push(val_5);
        sig += "D";
        let val_6 = jni::objects::JValueGen::Double(arg5.into());
        args.push(val_6);
        if let Some(a) = arg6 {
            sig += "Z";
            // 6
            let val_7 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_7);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "noise", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn get_noise_with_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: std::option::Option<f64>,
        arg5: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        args.push(val_3);
        sig += "I";
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        args.push(val_4);
        if let Some(a) = arg4 {
            sig += "D";
            let val_5 = jni::objects::JValueGen::Double(a.into());
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "D";
            let val_6 = jni::objects::JValueGen::Double(a.into());
            args.push(val_6);
        }
        sig += ")D";
        let cls = jni.find_class("double");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getNoise", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn instance(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Lorg/bukkit/util/noise/PerlinNoiseGenerator;";
        let cls = jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getInstance", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::noise::PerlinNoiseGenerator::from_raw(&jni, obj)
    }
    //

    pub fn floor(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(D)I");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "floor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for SimplexNoiseGenerator<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling SimplexNoiseGenerator.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::noise::PerlinNoiseGenerator<'mc>> for SimplexNoiseGenerator<'mc> {
    fn into(self) -> crate::util::noise::PerlinNoiseGenerator<'mc> {
        crate::util::noise::PerlinNoiseGenerator::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting SimplexNoiseGenerator into crate::util::noise::PerlinNoiseGenerator",
        )
    }
}
/// Generates noise using the "classic" perlin generator
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
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/util/noise/PerlinNoiseGenerator")?;
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
    pub fn new_with_world(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<blackboxmc_java::JavaRandom<'mc>>>,
    ) -> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/Random;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::PerlinNoiseGenerator::from_raw(&jni, res)
    }
    pub fn new_with_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i64>,
    ) -> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::PerlinNoiseGenerator::from_raw(&jni, res)
    }
    //

    pub fn noise_with_double(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: f64,
        arg5: f64,
        arg6: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        args.push(val_3);
        sig += "I";
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        args.push(val_4);
        sig += "D";
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        args.push(val_5);
        sig += "D";
        let val_6 = jni::objects::JValueGen::Double(arg5.into());
        args.push(val_6);
        if let Some(a) = arg6 {
            sig += "Z";
            // 6
            let val_7 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_7);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "noise", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    /// Generates noise for the 3D coordinates using the specified number of octaves and parameters
    pub fn get_noise_with_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: std::option::Option<f64>,
        arg5: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        args.push(val_3);
        sig += "I";
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        args.push(val_4);
        if let Some(a) = arg4 {
            sig += "D";
            let val_5 = jni::objects::JValueGen::Double(a.into());
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "D";
            let val_6 = jni::objects::JValueGen::Double(a.into());
            args.push(val_6);
        }
        sig += ")D";
        let cls = jni.find_class("double");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getNoise", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn instance(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/noise/PerlinNoiseGenerator;");
        let cls = jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getInstance", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::noise::PerlinNoiseGenerator::from_raw(&jni, obj)
    }
    //

    pub fn floor(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(D)I");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "floor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for PerlinNoiseGenerator<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PerlinNoiseGenerator.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::noise::NoiseGenerator<'mc>> for PerlinNoiseGenerator<'mc> {
    fn into(self) -> crate::util::noise::NoiseGenerator<'mc> {
        crate::util::noise::NoiseGenerator::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PerlinNoiseGenerator into crate::util::noise::NoiseGenerator")
    }
}
/// Creates simplex noise through unbiased octaves
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
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/util/noise/SimplexOctaveGenerator")?;
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
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::util::noise::SimplexOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/SimplexOctaveGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::SimplexOctaveGenerator::from_raw(&jni, res)
    }
    pub fn new_with_random(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::JavaRandom<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::util::noise::SimplexOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/Random;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/SimplexOctaveGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::SimplexOctaveGenerator::from_raw(&jni, res)
    }
    //

    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="OctaveGenerator.html#setScale(double)">OctaveGenerator</a></code></span>
    /// Sets the scale used for all coordinates passed to this generator.
    /// <p>This is the equivalent to setting each coordinate to the specified value.</p>
    pub fn set_scale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    /// Generates noise for the 3D coordinates using the specified number of octaves and parameters
    pub fn noise_with_double(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: f64,
        arg6: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        args.push(val_3);
        sig += "D";
        let val_4 = jni::objects::JValueGen::Double(arg3.into());
        args.push(val_4);
        sig += "D";
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        args.push(val_5);
        sig += "D";
        let val_6 = jni::objects::JValueGen::Double(arg5.into());
        args.push(val_6);
        if let Some(a) = arg6 {
            sig += "Z";
            // 6
            let val_7 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_7);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "noise", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    /// Sets the scale used for each W-coordinates passed
    pub fn set_wscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn wscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn set_xscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setXScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_yscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_zscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn xscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getXScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn yscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn zscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for SimplexOctaveGenerator<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling SimplexOctaveGenerator.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::noise::OctaveGenerator<'mc>> for SimplexOctaveGenerator<'mc> {
    fn into(self) -> crate::util::noise::OctaveGenerator<'mc> {
        crate::util::noise::OctaveGenerator::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting SimplexOctaveGenerator into crate::util::noise::OctaveGenerator",
        )
    }
}
/// Creates noise using unbiased octaves
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/util/noise/OctaveGenerator")?;
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
    //

    /// Sets the scale used for all coordinates passed to this generator.
    /// <p>This is the equivalent to setting each coordinate to the specified value.</p>
    pub fn set_scale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    /// Sets the scale used for each X-coordinates passed
    pub fn set_xscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setXScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    /// Sets the scale used for each Y-coordinates passed
    pub fn set_yscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    /// Sets the scale used for each Z-coordinates passed
    pub fn set_zscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn xscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getXScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn yscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn zscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    //

    /// Generates noise for the 3D coordinates using the specified number of octaves and parameters
    pub fn noise_with_double(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        args.push(val_3);
        sig += "D";
        let val_4 = jni::objects::JValueGen::Double(arg3.into());
        args.push(val_4);
        sig += "D";
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        args.push(val_5);
        if let Some(a) = arg5 {
            sig += "Z";
            // 5
            let val_6 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_6);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "noise", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for OctaveGenerator<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling OctaveGenerator.toString: {}", err),
        }
    }
}

/// Creates perlin noise through unbiased octaves
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
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/util/noise/PerlinOctaveGenerator")?;
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
    pub fn new_with_world(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::JavaRandom<'mc>>,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::util::noise::PerlinOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/Random;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/PerlinOctaveGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::PerlinOctaveGenerator::from_raw(&jni, res)
    }
    pub fn new_with_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: std::option::Option<i32>,
    ) -> Result<crate::util::noise::PerlinOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/PerlinOctaveGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::PerlinOctaveGenerator::from_raw(&jni, res)
    }
    //

    pub fn set_scale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_xscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setXScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_yscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_zscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn xscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getXScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn yscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn zscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    //

    pub fn noise_with_double(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        args.push(val_3);
        sig += "D";
        let val_4 = jni::objects::JValueGen::Double(arg3.into());
        args.push(val_4);
        sig += "D";
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        args.push(val_5);
        if let Some(a) = arg5 {
            sig += "Z";
            // 5
            let val_6 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_6);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "noise", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for PerlinOctaveGenerator<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PerlinOctaveGenerator.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::util::noise::OctaveGenerator<'mc>> for PerlinOctaveGenerator<'mc> {
    fn into(self) -> crate::util::noise::OctaveGenerator<'mc> {
        crate::util::noise::OctaveGenerator::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting PerlinOctaveGenerator into crate::util::noise::OctaveGenerator",
        )
    }
}
/// Base class for all noise generators
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/util/noise/NoiseGenerator")?;
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
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::noise::NoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/util/noise/NoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::NoiseGenerator::from_raw(&jni, res)
    }
    //

    /// Generates noise for the 3D coordinates using the specified number of octaves and parameters
    pub fn noise_with_double(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: i32,
        arg4: f64,
        arg5: f64,
        arg6: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2.into());
        args.push(val_3);
        sig += "I";
        let val_4 = jni::objects::JValueGen::Int(arg3.into());
        args.push(val_4);
        sig += "D";
        let val_5 = jni::objects::JValueGen::Double(arg4.into());
        args.push(val_5);
        sig += "D";
        let val_6 = jni::objects::JValueGen::Double(arg5.into());
        args.push(val_6);
        if let Some(a) = arg6 {
            sig += "Z";
            // 6
            let val_7 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_7);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "noise", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    /// Speedy floor, faster than (int)Math.floor(x)
    pub fn floor(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(D)I");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "floor",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn wait(
        &self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "wait", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn equals(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    #[doc(hidden)]
    pub fn internal_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

impl<'mc> std::string::ToString for NoiseGenerator<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling NoiseGenerator.toString: {}", err),
        }
    }
}
