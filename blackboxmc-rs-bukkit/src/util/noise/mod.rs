#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Generates simplex-based noise.
/// <p>This is a modified version of the freely published version in the paper by Stefan Gustavson at <a href="http://staffwww.itn.liu.se/~stegu/simplexnoise/simplexnoise.pdf"> http://staffwww.itn.liu.se/~stegu/simplexnoise/simplexnoise.pdf</a></p>
#[repr(C)]
pub struct SimplexNoiseGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SimplexNoiseGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SimplexNoiseGenerator<'mc> {
    fn from_raw(
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
}

impl<'mc> SimplexNoiseGenerator<'mc> {
    pub fn new_with_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<crate::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0);
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::SimplexNoiseGenerator::from_raw(&jni, res)
    }
    pub fn new_with_random(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
    ) -> Result<crate::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/Random;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::SimplexNoiseGenerator::from_raw(&jni, res)
    }
    /// Computes and returns the 4D simplex noise for the given coordinates in 4D space
    pub fn noise_with_double(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: std::option::Option<f64>,
        arg3: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0);
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "D";
            let val_3 = jni::objects::JValueGen::Double(a);
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "D";
            let val_4 = jni::objects::JValueGen::Double(a);
            args.push(val_4);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "noise", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Computes and returns the 4D simplex noise for the given coordinates in 4D space
    pub fn get_noise_with_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: std::option::Option<f64>,
        arg2: std::option::Option<f64>,
        arg3: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "D";
            let val_3 = jni::objects::JValueGen::Double(a);
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "D";
            let val_4 = jni::objects::JValueGen::Double(a);
            args.push(val_4);
        }
        sig += ")D";
        let cls = jni.find_class("double");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getNoise", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn instance(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::noise::SimplexNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/noise/SimplexNoiseGenerator;");
        let cls = jni.find_class("org/bukkit/util/noise/SimplexNoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getInstance", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::noise::SimplexNoiseGenerator::from_raw(&jni, obj)
    }
    // SUPER CLASS: PerlinNoiseGenerator
    // SUPER CLASS: NoiseGenerator
    pub fn floor(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        crate::util::noise::NoiseGenerator::floor(jni, arg0)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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
#[repr(C)]
pub struct PerlinNoiseGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PerlinNoiseGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PerlinNoiseGenerator<'mc> {
    fn from_raw(
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
}

impl<'mc> PerlinNoiseGenerator<'mc> {
    pub fn new_with_random(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
    ) -> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/Random;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::PerlinNoiseGenerator::from_raw(&jni, res)
    }
    pub fn new_with_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<crate::util::noise::PerlinNoiseGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0);
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/PerlinNoiseGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::PerlinNoiseGenerator::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="NoiseGenerator.html#noise(double,double,double)">NoiseGenerator</a></code></span>
    /// Computes and returns the 3D noise for the given coordinates in 3D space
    pub fn noise(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(DDD)D");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let val_2 = jni::objects::JValueGen::Double(arg1);
        let val_3 = jni::objects::JValueGen::Double(arg2);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "noise",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    /// Generates noise for the 3D coordinates using the specified number of octaves and parameters
    pub fn get_noise_with_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: std::option::Option<f64>,
        arg2: std::option::Option<f64>,
        arg3: std::option::Option<i32>,
        arg4: std::option::Option<f64>,
        arg5: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "D";
            let val_3 = jni::objects::JValueGen::Double(a);
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "D";
            let val_5 = jni::objects::JValueGen::Double(a);
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "D";
            let val_6 = jni::objects::JValueGen::Double(a);
            args.push(val_6);
        }
        sig += ")D";
        let cls = jni.find_class("double");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getNoise", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.d()?)
    }

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
    // SUPER CLASS: NoiseGenerator
    pub fn floor(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        crate::util::noise::NoiseGenerator::floor(jni, arg0)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::util::noise::NoiseGenerator<'mc>> for PerlinNoiseGenerator<'mc> {
    fn into(self) -> crate::util::noise::NoiseGenerator<'mc> {
        crate::util::noise::NoiseGenerator::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PerlinNoiseGenerator into crate::util::noise::NoiseGenerator")
    }
}
/// Creates simplex noise through unbiased octaves
#[repr(C)]
pub struct SimplexOctaveGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for SimplexOctaveGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for SimplexOctaveGenerator<'mc> {
    fn from_raw(
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
}

impl<'mc> SimplexOctaveGenerator<'mc> {
    pub fn new_with_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: i32,
    ) -> Result<crate::util::noise::SimplexOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0);
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1);
        args.push(val_2);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/SimplexOctaveGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::SimplexOctaveGenerator::from_raw(&jni, res)
    }
    pub fn new_with_random(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        arg1: i32,
    ) -> Result<crate::util::noise::SimplexOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/Random;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1);
        args.push(val_2);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/SimplexOctaveGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::SimplexOctaveGenerator::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="OctaveGenerator.html#setScale(double)">OctaveGenerator</a></code></span>
    /// Sets the scale used for all coordinates passed to this generator.
    /// <p>This is the equivalent to setting each coordinate to the specified value.</p>
    pub fn set_scale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
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
        let val_1 = jni::objects::JValueGen::Double(arg0);
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1);
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2);
        args.push(val_3);
        sig += "D";
        let val_4 = jni::objects::JValueGen::Double(arg3);
        args.push(val_4);
        sig += "D";
        let val_5 = jni::objects::JValueGen::Double(arg4);
        args.push(val_5);
        sig += "D";
        let val_6 = jni::objects::JValueGen::Double(arg5);
        args.push(val_6);
        if let Some(a) = arg6 {
            sig += "Z";
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
    /// Sets the scale used for each W-coordinates passed
    pub fn set_wscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setWScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn wscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getWScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    // SUPER CLASS: OctaveGenerator
    pub fn set_xscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.set_xscale(arg0)
    }
    pub fn set_yscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.set_yscale(arg0)
    }
    pub fn set_zscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.set_zscale(arg0)
    }
    pub fn xscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.xscale()
    }
    pub fn yscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.yscale()
    }
    pub fn zscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.zscale()
    }
    pub fn octaves(
        &self,
    ) -> Result<Vec<crate::util::noise::NoiseGenerator<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/noise/NoiseGenerator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOctaves", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::util::noise::NoiseGenerator::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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
#[repr(C)]
pub struct OctaveGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for OctaveGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for OctaveGenerator<'mc> {
    fn from_raw(
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
}

impl<'mc> OctaveGenerator<'mc> {
    /// Sets the scale used for all coordinates passed to this generator.
    /// <p>This is the equivalent to setting each coordinate to the specified value.</p>
    pub fn set_scale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the scale used for each X-coordinates passed
    pub fn set_xscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setXScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the scale used for each Y-coordinates passed
    pub fn set_yscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setYScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the scale used for each Z-coordinates passed
    pub fn set_zscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setZScale",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn xscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getXScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn yscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getYScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn zscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getZScale", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn octaves(
        &self,
    ) -> Result<Vec<crate::util::noise::NoiseGenerator<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/noise/NoiseGenerator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOctaves", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::util::noise::NoiseGenerator::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    /// Generates noise for the 3D coordinates using the specified number of octaves and parameters
    pub fn noise_with_double(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: std::option::Option<f64>,
        arg4: std::option::Option<f64>,
        arg5: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0);
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1);
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2);
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "D";
            let val_4 = jni::objects::JValueGen::Double(a);
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "D";
            let val_5 = jni::objects::JValueGen::Double(a);
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "Z";
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Creates perlin noise through unbiased octaves
#[repr(C)]
pub struct PerlinOctaveGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PerlinOctaveGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PerlinOctaveGenerator<'mc> {
    fn from_raw(
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
}

impl<'mc> PerlinOctaveGenerator<'mc> {
    pub fn new_with_random(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        arg1: i32,
    ) -> Result<crate::util::noise::PerlinOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/Random;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1);
        args.push(val_2);
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
        arg1: i32,
    ) -> Result<crate::util::noise::PerlinOctaveGenerator<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0);
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1);
        args.push(val_2);
        sig += ")V";
        let cls = jni.find_class("org/bukkit/util/noise/PerlinOctaveGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::util::noise::PerlinOctaveGenerator::from_raw(&jni, res)
    }
    // SUPER CLASS: OctaveGenerator
    pub fn set_scale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.set_scale(arg0)
    }
    pub fn set_xscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.set_xscale(arg0)
    }
    pub fn set_yscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.set_yscale(arg0)
    }
    pub fn set_zscale(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.set_zscale(arg0)
    }
    pub fn xscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.xscale()
    }
    pub fn yscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.yscale()
    }
    pub fn zscale(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let temp_clone = crate::util::noise::OctaveGenerator::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::util::noise::OctaveGenerator = temp_clone.into();
        real.zscale()
    }
    pub fn octaves(
        &self,
    ) -> Result<Vec<crate::util::noise::NoiseGenerator<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/util/noise/NoiseGenerator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getOctaves", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let arr = Into::<jni::objects::JObjectArray>::into(res.l()?);
        let len = self.jni_ref().get_array_length(&arr)?;
        let mut vec = Vec::new();
        for i in 0..len {
            let res = self.jni_ref().get_object_array_element(&arr, i)?;
            vec.push({ crate::util::noise::NoiseGenerator::from_raw(&self.jni_ref(), res)? });
        }
        Ok(vec)
    }
    pub fn noise_with_double(
        &self,
        arg0: f64,
        arg1: f64,
        arg2: f64,
        arg3: std::option::Option<f64>,
        arg4: std::option::Option<f64>,
        arg5: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0);
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1);
        args.push(val_2);
        sig += "D";
        let val_3 = jni::objects::JValueGen::Double(arg2);
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "D";
            let val_4 = jni::objects::JValueGen::Double(a);
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "D";
            let val_5 = jni::objects::JValueGen::Double(a);
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "Z";
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
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
#[repr(C)]
pub struct NoiseGenerator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for NoiseGenerator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for NoiseGenerator<'mc> {
    fn from_raw(
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
    /// Generates noise for the 3D coordinates using the specified number of octaves and parameters
    pub fn noise_with_double(
        &self,
        arg0: f64,
        arg1: std::option::Option<f64>,
        arg2: std::option::Option<f64>,
        arg3: std::option::Option<i32>,
        arg4: std::option::Option<f64>,
        arg5: std::option::Option<f64>,
        arg6: std::option::Option<bool>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "D";
            let val_3 = jni::objects::JValueGen::Double(a);
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "D";
            let val_5 = jni::objects::JValueGen::Double(a);
            args.push(val_5);
        }
        if let Some(a) = arg5 {
            sig += "D";
            let val_6 = jni::objects::JValueGen::Double(a);
            args.push(val_6);
        }
        if let Some(a) = arg6 {
            sig += "Z";
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
    /// Speedy floor, faster than (int)Math.floor(x)
    pub fn floor(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(D)I");
        let val_1 = jni::objects::JValueGen::Double(arg0);
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
    // SUPER CLASS: Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
