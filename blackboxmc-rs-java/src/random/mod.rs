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
        let (valid, name) = env.validate_name(&obj, "java/util/random/RandomGeneratorFactory")?;
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
    //

    pub fn equidistribution(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equidistribution",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_stochastic(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStochastic", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_hardware(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isHardware", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn state_bits(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "stateBits", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn is_statistical(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStatistical", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_arbitrarily_jumpable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isArbitrarilyJumpable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_jumpable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isJumpable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_leapable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isLeapable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_splittable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isSplittable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn is_streamable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isStreamable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn all(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/stream/Stream;");
        let cls = jni.find_class("java/util/stream/Stream");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "all", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn period(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/math/BigInteger;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "period", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "name", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn group(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "group", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn default(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/random/RandomGeneratorFactory;");
        let cls = jni.find_class("java/util/random/RandomGeneratorFactory");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getDefault", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn create(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/random/RandomGenerator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "create", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn wait(
        &mut self,
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
        &mut self,
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

    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
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

    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Class;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getClass", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    //

    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}

///
/// This is a representation of an abstract class.
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
        let (valid, name) =
            env.validate_name(&obj, "java/util/random/RandomGenerator$SplittableGenerator")?;
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
    //

    pub fn rngs(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "rngs", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn splits(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/random/RandomGenerator$SplittableGenerator;";
            let val_1 = jni::objects::JValueGen::Object(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "splits", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn splits_with_long(
        &mut self,
        arg0: i64,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/util/random/RandomGenerator$SplittableGenerator;";
            let val_2 = jni::objects::JValueGen::Object(a);
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "splits", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn split(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/random/RandomGenerator$SplittableGenerator;";
            let val_1 = jni::objects::JValueGen::Object(a);
            args.push(val_1);
        }
        sig += ")Ljava/util/random/RandomGenerator$SplittableGenerator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "split", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "J";
            let val_3 = jni::objects::JValueGen::Long(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/LongStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longs", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "D";
            let val_3 = jni::objects::JValueGen::Double(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
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
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaRandomGeneratorSplittableGenerator<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}

///
/// This is a representation of an abstract class.
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
        let (valid, name) =
            env.validate_name(&obj, "java/util/random/RandomGenerator$StreamableGenerator")?;
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
    //

    pub fn rngs(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "rngs", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "J";
            let val_3 = jni::objects::JValueGen::Long(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/LongStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longs", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "D";
            let val_3 = jni::objects::JValueGen::Double(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
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

///
/// This is a representation of an abstract class.
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
        let (valid, name) =
            env.validate_name(&obj, "java/util/random/RandomGenerator$LeapableGenerator")?;
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
    //

    pub fn leap_distance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leapDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn leap(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leap", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn leaps(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leaps", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn copy_and_leap(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/random/RandomGenerator$JumpableGenerator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyAndLeap", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn copy(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/random/RandomGenerator$JumpableGenerator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn jump_distance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "jumpDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn jumps(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jumps", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn copy_and_jump(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/random/RandomGenerator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyAndJump", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn rngs(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "rngs", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn jump(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jump", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "J";
            let val_3 = jni::objects::JValueGen::Long(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/LongStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longs", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "D";
            let val_3 = jni::objects::JValueGen::Double(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
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
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaRandomGeneratorLeapableGenerator<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}

///
/// This is a representation of an abstract class.
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
        let (valid, name) = env.validate_name(&obj, "java/util/random/RandomGenerator")?;
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
    //

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "J";
            let val_3 = jni::objects::JValueGen::Long(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/LongStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longs", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "D";
            let val_3 = jni::objects::JValueGen::Double(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn default(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/random/RandomGenerator;");
        let cls = jni.find_class("java/util/random/RandomGenerator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getDefault", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        Ok(res.l()?)
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

///
/// This is a representation of an abstract class.
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
        let (valid, name) = env.validate_name(
            &obj,
            "java/util/random/RandomGenerator$ArbitrarilyJumpableGenerator",
        )?;
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
    //

    pub fn jumps(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jumps", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn jumps_with_long(
        &mut self,
        arg0: i64,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jumps", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn copy_and_jump(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/random/RandomGenerator$ArbitrarilyJumpableGenerator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copyAndJump", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn jump_power_of_two(&mut self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "jumpPowerOfTwo",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn leap(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leap", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn jump(
        &mut self,
        arg0: std::option::Option<f64>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jump", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn copy(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += ")Ljava/util/random/RandomGenerator$LeapableGenerator;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn leap_distance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "leapDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn leaps(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "leaps", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn copy_and_leap(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/random/RandomGenerator$JumpableGenerator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyAndLeap", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn jump_distance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "jumpDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn rngs(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "rngs", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "J";
            let val_3 = jni::objects::JValueGen::Long(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/LongStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longs", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "D";
            let val_3 = jni::objects::JValueGen::Double(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
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
impl<'mc> Into<jni::objects::JObject<'mc>>
    for JavaRandomGeneratorArbitrarilyJumpableGenerator<'mc>
{
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}

///
/// This is a representation of an abstract class.
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
        let (valid, name) =
            env.validate_name(&obj, "java/util/random/RandomGenerator$JumpableGenerator")?;
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
    //

    pub fn jump_distance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "jumpDistance", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn jumps(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jumps", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn copy_and_jump(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/random/RandomGenerator;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "copyAndJump", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn rngs(
        &mut self,
        arg0: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/util/stream/Stream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "rngs", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn jump(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "jump", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn copy(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/random/RandomGenerator$JumpableGenerator;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copy", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn next_boolean(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_long(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        sig += ")J";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextLong", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    pub fn next_float(
        &mut self,
        arg0: std::option::Option<f32>,
        arg1: std::option::Option<f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "F";
            let val_2 = jni::objects::JValueGen::Float(a.into());
            args.push(val_2);
        }
        sig += ")F";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextFloat", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn ints(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn ints_with_long(
        &mut self,
        arg0: i64,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/IntStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ints", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn longs(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i64>,
        arg2: std::option::Option<i64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "J";
            let val_2 = jni::objects::JValueGen::Long(a.into());
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "J";
            let val_3 = jni::objects::JValueGen::Long(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/LongStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longs", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn doubles_with_long(
        &mut self,
        arg0: i64,
        arg1: f64,
        arg2: std::option::Option<f64>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        args.push(val_1);
        sig += "D";
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "D";
            let val_3 = jni::objects::JValueGen::Double(a.into());
            args.push(val_3);
        }
        sig += ")Ljava/util/stream/DoubleStream;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "doubles", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
    //

    pub fn next_bytes(&mut self, arg0: Vec<i8>) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(B)V");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextBytes", sig.as_str(), vec![]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn next_gaussian(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextGaussian", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn is_deprecated(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isDeprecated", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn next_exponential(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "nextExponential", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_double(
        &mut self,
        arg0: std::option::Option<f64>,
        arg1: std::option::Option<f64>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "D";
            let val_2 = jni::objects::JValueGen::Double(a.into());
            args.push(val_2);
        }
        sig += ")D";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextDouble", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
    //

    pub fn next_int(
        &mut self,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a.into());
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a.into());
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "nextInt", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
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
impl<'mc> Into<jni::objects::JObject<'mc>> for JavaRandomGeneratorJumpableGenerator<'mc> {
    fn into(self) -> jni::objects::JObject<'mc> {
        self.1
    }
}
