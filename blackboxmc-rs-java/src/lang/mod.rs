#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// The <code>Integer</code> class wraps a value of the primitive type
/// <code>int</code> in an object. An object of type <code>Integer</code>
/// contains a single field whose type is <code>int</code>.
/// <p>In addition, this class provides several methods for converting
/// an <code>int</code> to a <code>String</code> and a <code>String</code> to an
/// <code>int</code>, as well as other constants and methods useful when
/// dealing with an <code>int</code>.
/// </p><p>Implementation note: The implementations of the "bit twiddling"
/// methods (such as <a href="../../java/lang/Integer.html#highestOneBit-int-"><code>highestOneBit</code></a> and
/// <a href="../../java/lang/Integer.html#numberOfTrailingZeros-int-"><code>numberOfTrailingZeros</code></a>) are
/// based on material from Henry S. Warren, Jr.'s <i>Hacker's
/// Delight</i>, (Addison Wesley, 2002).</p>
#[repr(C)]
pub struct JavaInteger<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaInteger<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JavaInteger<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaInteger from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/lang/Integer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaInteger object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaInteger<'mc> {
    #[deprecated]

    pub fn new_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<crate::lang::JavaInteger<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("java/lang/Integer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaInteger::from_raw(&jni, res)
    }

    pub fn number_of_leading_zeros(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "numberOfLeadingZeros",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn number_of_trailing_zeros(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "numberOfTrailingZeros",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn bit_count(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "bitCount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

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

    pub fn to_string_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i32>,
        arg1: std::option::Option<i32>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "hashCode", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn min(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "min",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn max(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "max",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn signum(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "signum",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn reverse_bytes(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "reverseBytes",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn compare_to_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "compareTo", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn compare(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compare",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn byte_value(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "byteValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn short_value(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "shortValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn int_value(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "intValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn long_value(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn float_value(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "floatValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn double_value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "doubleValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn to_hex_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toHexString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn decode(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/lang/Integer;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/lang/Integer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "decode",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn describe_constable(
        &self,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Optional;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn reverse(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "reverse",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn to_unsigned_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(I)J");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toUnsignedLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn sum(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "sum",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn compare_unsigned(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compareUnsigned",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn to_unsigned_string_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: std::option::Option<i32>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toUnsignedString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn get_integer_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/Integer;";
        let cls = jni.find_class("java/lang/Integer");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getInteger", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn to_octal_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toOctalString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_binary_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toBinaryString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn divide_unsigned(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "divideUnsigned",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn remainder_unsigned(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "remainderUnsigned",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn highest_one_bit(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "highestOneBit",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn lowest_one_bit(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "lowestOneBit",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn rotate_left(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "rotateLeft",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn rotate_right(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "rotateRight",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }
    //Number
    //crate::lang::JavaNumber
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The <code>Float</code> class wraps a value of primitive type
/// <code>float</code> in an object. An object of type
/// <code>Float</code> contains a single field whose type is
/// <code>float</code>.
/// <p>In addition, this class provides several methods for converting a
/// <code>float</code> to a <code>String</code> and a
/// <code>String</code> to a <code>float</code>, as well as other
/// constants and methods useful when dealing with a
/// <code>float</code>.</p>
#[repr(C)]
pub struct JavaFloat<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaFloat<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JavaFloat<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaFloat from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/lang/Float")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaFloat object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaFloat<'mc> {
    #[deprecated]

    pub fn new_with_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<crate::lang::JavaFloat<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "D";
        let val_1 = jni::objects::JValueGen::Double(arg0);
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("java/lang/Float");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaFloat::from_raw(&jni, res)
    }
    #[deprecated]

    pub fn new_with_float(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f32,
    ) -> Result<crate::lang::JavaFloat<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "F";
        let val_1 = jni::objects::JValueGen::Float(arg0);
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("java/lang/Float");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaFloat::from_raw(&jni, res)
    }

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

    pub fn to_string_with_float(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f32>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a);
            args.push(val_1);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code_with_float(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a);
            args.push(val_1);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "hashCode", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn min(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f32,
        arg1: f32,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("(FF)F");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let val_2 = jni::objects::JValueGen::Float(arg1);
        let cls = jni.find_class("float");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "min",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn max(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f32,
        arg1: f32,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("(FF)F");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let val_2 = jni::objects::JValueGen::Float(arg1);
        let cls = jni.find_class("float");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "max",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn float_to_raw_int_bits(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(F)I");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "floatToRawIntBits",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn float_to_int_bits(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(F)I");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "floatToIntBits",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn int_bits_to_float(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)F");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("float");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "intBitsToFloat",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn compare_to_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "compareTo", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn compare(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f32,
        arg1: f32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(FF)I");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let val_2 = jni::objects::JValueGen::Float(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compare",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn byte_value(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "byteValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn short_value(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "shortValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn int_value(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "intValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn long_value(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn float_value(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "floatValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn double_value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "doubleValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn to_hex_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(F)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toHexString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn describe_constable(
        &self,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Optional;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_na_n_with_float(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a);
            args.push(val_1);
        }
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isNaN", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn sum(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f32,
        arg1: f32,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("(FF)F");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let val_2 = jni::objects::JValueGen::Float(arg1);
        let cls = jni.find_class("float");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "sum",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn parse_float(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)F");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("float");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "parseFloat",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn is_infinite_with_float(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "F";
            let val_1 = jni::objects::JValueGen::Float(a);
            args.push(val_1);
        }
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isInfinite", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_finite(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(F)Z");
        let val_1 = jni::objects::JValueGen::Float(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isFinite",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }
    //Number
    //crate::lang::JavaNumber
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The <code>Character</code> class wraps a value of the primitive
/// type <code>char</code> in an object. An object of class
/// <code>Character</code> contains a single field whose type is
/// <code>char</code>.
/// <p>
/// In addition, this class provides a large number of static methods for
/// determining a character's category (lowercase letter, digit, etc.)
/// and for converting characters from uppercase to lowercase and vice
/// versa.
/// </p><h3><a id="conformance">Unicode Conformance</a></h3>
/// <p>
/// The fields and methods of class <code>Character</code> are defined in terms
/// of character information from the Unicode Standard, specifically the
/// <i>UnicodeData</i> file that is part of the Unicode Character Database.
/// This file specifies properties including name and category for every
/// assigned Unicode code point or character range. The file is available
/// from the Unicode Consortium at
/// <a href="http://www.unicode.org">http://www.unicode.org</a>.
/// </p><p>
/// The Java SE 8 Platform uses character information from version 6.2
/// of the Unicode Standard, with three extensions. First, in recognition of the fact
/// that new currencies appear frequently, the Java SE 8 Platform allows an
/// implementation of class <code>Character</code> to use the Currency Symbols
/// block from version 10.0 of the Unicode Standard. Second, the Java SE 8 Platform
/// allows an implementation of class <code>Character</code> to use the code points
/// in the range of <code>U+9FCD</code> to <code>U+9FEF</code> from version 11.0 of the
/// Unicode Standard and in the <code>CJK Unified Ideographs Extension E</code> block
/// from version 8.0 of the Unicode Standard, in order for the class to allow the
/// "Implementation Level 2" of the Chinese GB18030-2022 standard.
/// Third, the Java SE 8 Platform
/// allows an implementation of class <code>Character</code> to use the Japanese Era
/// code point, <code>U+32FF</code>, from the Unicode Standard version 12.1.
/// Consequently, the
/// behavior of fields and methods of class <code>Character</code> may vary across
/// implementations of the Java SE 8 Platform when processing the aforementioned
/// code points ( outside of version 6.2 ), except for the following methods
/// that define Java identifiers:
/// <a href="../../java/lang/Character.html#isJavaIdentifierStart-int-"><code>isJavaIdentifierStart(int)</code></a>, <a href="../../java/lang/Character.html#isJavaIdentifierStart-char-"><code>isJavaIdentifierStart(char)</code></a>,
/// <a href="../../java/lang/Character.html#isJavaIdentifierPart-int-"><code>isJavaIdentifierPart(int)</code></a>, and <a href="../../java/lang/Character.html#isJavaIdentifierPart-char-"><code>isJavaIdentifierPart(char)</code></a>.
/// Code points in Java identifiers must be drawn from version 6.2 of
/// the Unicode Standard.
/// </p><h3><a name="unicode">Unicode Character Representations</a></h3>
/// <p>The <code>char</code> data type (and therefore the value that a
/// <code>Character</code> object encapsulates) are based on the
/// original Unicode specification, which defined characters as
/// fixed-width 16-bit entities. The Unicode Standard has since been
/// changed to allow for characters whose representation requires more
/// than 16 bits.The range of legal <em>code point</em>s is now
/// U+0000 to U+10FFFF, known as <em>Unicode scalar value</em>.
/// (Refer to the <a href="http://www.unicode.org/reports/tr27/#notation"><i>
/// definition</i></a> of the U+<i>n</i> notation in the Unicode
/// Standard.)
/// </p><p><a name="BMP">The set of characters from U+0000 to U+FFFF</a> is
/// sometimes referred to as the <em>Basic Multilingual Plane (BMP)</em>.
/// <a name="supplementary">Characters</a> whose code points are greater
/// than U+FFFF are called <em>supplementary character</em>s.The Java
/// platform uses the UTF-16 representation in <code>char</code> arrays and
/// in the <code>String</code> and <code>StringBuffer</code> classes. In
/// this representation, supplementary characters are represented as a pair
/// of <code>char</code> values, the first from the <em>high-surrogates</em>
/// range, (\uD800-\uDBFF), the second from the
/// <em>low-surrogates</em> range (\uDC00-\uDFFF).
/// </p><p>A <code>char</code> value, therefore, represents Basic
/// Multilingual Plane (BMP) code points, including the surrogate
/// code points, or code units of the UTF-16 encoding. An
/// <code>int</code> value represents all Unicode code points,
/// including supplementary code points. The lower (least significant)
/// 21 bits of <code>int</code> are used to represent Unicode code
/// points and the upper (most significant) 11 bits must be zero.
/// Unless otherwise specified, the behavior with respect to
/// supplementary characters and surrogate <code>char</code> values is
/// as follows:
/// </p><ul>
/// <li>The methods that only accept a <code>char</code> value cannot support
/// supplementary characters. They treat <code>char</code> values from the
/// surrogate ranges as undefined characters. For example,
/// <code>Character.isLetter('\uD840')</code> returns <code>false</code>, even though
/// this specific value if followed by any low-surrogate value in a string
/// would represent a letter.
/// </li><li>The methods that accept an <code>int</code> value support all
/// Unicode characters, including supplementary characters. For
/// example, <code>Character.isLetter(0x2F81A)</code> returns
/// <code>true</code> because the code point value represents a letter
/// (a CJK ideograph).
/// </li></ul>
/// <p>In the Java SE API documentation, <em>Unicode code point</em> is
/// used for character values in the range between U+0000 and U+10FFFF,
/// and <em>Unicode code unit</em> is used for 16-bit
/// <code>char</code> values that are code units of the <em>UTF-16</em>
/// encoding. For more information on Unicode terminology, refer to the
/// <a href="http://www.unicode.org/glossary/">Unicode Glossary</a>.</p>
#[repr(C)]
pub struct JavaCharacter<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaCharacter<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JavaCharacter<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaCharacter from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/lang/Character")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaCharacter object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaCharacter<'mc> {
    #[deprecated]

    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<crate::lang::JavaCharacter<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(C)V");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let cls = jni.find_class("java/lang/Character");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaCharacter::from_raw(&jni, res)
    }

    pub fn get_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_java_identifier_start_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isJavaIdentifierStart", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_java_identifier_part_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isJavaIdentifierPart", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

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

    pub fn to_string_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<u16>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "C";
            let val_1 = jni::objects::JValueGen::Char(a);
            args.push(val_1);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<u16>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "C";
            let val_1 = jni::objects::JValueGen::Char(a);
            args.push(val_1);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "hashCode", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn reverse_bytes(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<u16, Box<dyn std::error::Error>> {
        let sig = String::from("(C)C");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let cls = jni.find_class("char");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "reverseBytes",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.c()?)
    }

    pub fn compare_to_with_character(&self, arg0: u16) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Character;";
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Character",
            "(C)V",
            vec![arg0.into()],
        )?);
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "compareTo", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_digit_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isDigit", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_lower_case_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isLowerCase", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_upper_case_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isUpperCase", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_whitespace_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isWhitespace", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn compare(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
        arg1: u16,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(CC)I");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let val_2 = jni::objects::JValueGen::Char(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compare",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn char_value(&self) -> Result<u16, Box<dyn std::error::Error>> {
        let sig = String::from("()C");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "charValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.c()?)
    }

    pub fn to_chars_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: std::option::Option<Vec<u16>>,
        arg2: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "[C";
            let arr = jni.new_char_array(a.len() as i32);
            let mut vec = Vec::new();
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_2 = *a.get(i).unwrap();
                vec.push(val_2)
            }
            jni.set_char_array_region(&arr, 0, &vec)?;
            let val_2 = jni::objects::JValueGen::Object(arr);
            args.push(val_2.l()?.into());
        }
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toChars", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_high_surrogate(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(C)Z");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isHighSurrogate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_low_surrogate(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(C)Z");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isLowSurrogate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_surrogate(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(C)Z");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isSurrogate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_supplementary_code_point(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Z");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isSupplementaryCodePoint",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn high_surrogate(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<u16, Box<dyn std::error::Error>> {
        let sig = String::from("(I)C");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("char");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "highSurrogate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.c()?)
    }

    pub fn low_surrogate(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<u16, Box<dyn std::error::Error>> {
        let sig = String::from("(I)C");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("char");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "lowSurrogate",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.c()?)
    }

    pub fn to_code_point(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
        arg1: u16,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(CC)I");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let val_2 = jni::objects::JValueGen::Char(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toCodePoint",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn code_point_at_with_chars(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<u16>,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[C";
        let arr = jni.new_char_array(arg0.len() as i32);
        let mut vec = Vec::new();
        let arr = jni.translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = *arg0.get(i).unwrap();
            vec.push(val_1)
        }
        jni.set_char_array_region(&arr, 0, &vec)?;
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "codePointAt", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn code_point_before_with_chars(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<u16>,
        arg1: i32,
        arg2: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[C";
        let arr = jni.new_char_array(arg0.len() as i32);
        let mut vec = Vec::new();
        let arr = jni.translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = *arg0.get(i).unwrap();
            vec.push(val_1)
        }
        jni.set_char_array_region(&arr, 0, &vec)?;
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1);
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "codePointBefore", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn offset_by_code_points_with_chars(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<u16>,
        arg1: i32,
        arg2: i32,
        arg3: std::option::Option<i32>,
        arg4: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[C";
        let arr = jni.new_char_array(arg0.len() as i32);
        let mut vec = Vec::new();
        let arr = jni.translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = *arg0.get(i).unwrap();
            vec.push(val_1)
        }
        jni.set_char_array_region(&arr, 0, &vec)?;
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1);
        args.push(val_2);
        sig += "I";
        let val_3 = jni::objects::JValueGen::Int(arg2);
        args.push(val_3);
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        if let Some(a) = arg4 {
            sig += "I";
            let val_5 = jni::objects::JValueGen::Int(a);
            args.push(val_5);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "offsetByCodePoints", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn to_lower_case_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<u16, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")C";
        let cls = jni.find_class("char");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toLowerCase", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.c()?)
    }

    pub fn to_upper_case_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toUpperCase", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_bmp_code_point(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Z");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isBmpCodePoint",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn describe_constable(
        &self,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Optional;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_type_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getType", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_letter_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isLetter", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_letter_or_digit_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isLetterOrDigit", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_valid_code_point(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Z");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isValidCodePoint",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_title_case_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isTitleCase", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_defined_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isDefined", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_ideographic(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Z");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isIdeographic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_unicode_identifier_start_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isUnicodeIdentifierStart", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_unicode_identifier_part_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isUnicodeIdentifierPart", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_identifier_ignorable_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isIdentifierIgnorable", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn to_title_case_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<u16, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")C";
        let cls = jni.find_class("char");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toTitleCase", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.c()?)
    }

    pub fn digit_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1);
        args.push(val_2);
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "digit", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn get_numeric_value_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getNumericValue", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_space_char_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isSpaceChar", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_isocontrol_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isISOControl", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn get_directionality_with_char(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "C";
        let val_1 = jni::objects::JValueGen::Char(arg0);
        args.push(val_1);
        sig += ")B";
        let cls = jni.find_class("byte");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getDirectionality", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn is_mirrored_with_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isMirrored", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_surrogate_pair(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
        arg1: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(CC)Z");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let val_2 = jni::objects::JValueGen::Char(arg1);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isSurrogatePair",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn char_count(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "charCount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn is_java_letter(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(C)Z");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isJavaLetter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_java_letter_or_digit(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(C)Z");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isJavaLetterOrDigit",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_alphabetic(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Z");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isAlphabetic",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_space(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: u16,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(C)Z");
        let val_1 = jni::objects::JValueGen::Char(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isSpace",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn for_digit(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i32,
        arg1: i32,
    ) -> Result<u16, Box<dyn std::error::Error>> {
        let sig = String::from("(II)C");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("char");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "forDigit",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.c()?)
    }

    pub fn code_point_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)I");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "codePointOf",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The <code>Byte</code> class wraps a value of primitive type <code>byte</code>
/// in an object.An object of type <code>Byte</code> contains a single
/// field whose type is <code>byte</code>.
/// <p>In addition, this class provides several methods for converting
/// a <code>byte</code> to a <code>String</code> and a <code>String</code> to a <code>byte</code>, as well as other constants and methods useful when dealing
/// with a <code>byte</code>.</p>
#[repr(C)]
pub struct JavaByte<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaByte<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JavaByte<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaByte from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/lang/Byte")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaByte object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaByte<'mc> {
    #[deprecated]

    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::lang::JavaByte<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("java/lang/Byte");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaByte::from_raw(&jni, res)
    }

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

    pub fn to_string_with_byte(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i8>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "B";
            let val_1 = jni::objects::JValueGen::Byte(a);
            args.push(val_1);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code_with_byte(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i8>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "B";
            let val_1 = jni::objects::JValueGen::Byte(a);
            args.push(val_1);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "hashCode", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn compare_to_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "compareTo", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn compare(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i8,
        arg1: i8,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(BB)I");
        let val_1 = jni::objects::JValueGen::Byte(arg0);
        let val_2 = jni::objects::JValueGen::Byte(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compare",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn byte_value(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "byteValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn short_value(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "shortValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn int_value(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "intValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn long_value(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn float_value(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "floatValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn double_value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "doubleValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn decode(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/lang/Byte;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/lang/Byte");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "decode",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn describe_constable(
        &self,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Optional;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn to_unsigned_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i8,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(B)J");
        let val_1 = jni::objects::JValueGen::Byte(arg0);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toUnsignedLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn to_unsigned_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i8,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(B)I");
        let val_1 = jni::objects::JValueGen::Byte(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toUnsignedInt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn parse_byte_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<i8, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")B";
        let cls = jni.find_class("byte");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "parseByte", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn compare_unsigned(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i8,
        arg1: i8,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(BB)I");
        let val_1 = jni::objects::JValueGen::Byte(arg0);
        let val_2 = jni::objects::JValueGen::Byte(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compareUnsigned",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }
    //Number
    //crate::lang::JavaNumber
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The <code>Long</code> class wraps a value of the primitive type <code>long</code> in an object. An object of type <code>Long</code> contains a
/// single field whose type is <code>long</code>.
/// <p> In addition, this class provides several methods for converting
/// a <code>long</code> to a <code>String</code> and a <code>String</code> to a <code>long</code>, as well as other constants and methods useful when dealing
/// with a <code>long</code>.
/// </p><p>Implementation note: The implementations of the "bit twiddling"
/// methods (such as <a href="../../java/lang/Long.html#highestOneBit-long-"><code>highestOneBit</code></a> and
/// <a href="../../java/lang/Long.html#numberOfTrailingZeros-long-"><code>numberOfTrailingZeros</code></a>) are
/// based on material from Henry S. Warren, Jr.'s <i>Hacker's
/// Delight</i>, (Addison Wesley, 2002).</p>
#[repr(C)]
pub struct JavaLong<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLong<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JavaLong<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaLong from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/lang/Long")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLong object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLong<'mc> {
    #[deprecated]

    pub fn new_with_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<crate::lang::JavaLong<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0);
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("java/lang/Long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaLong::from_raw(&jni, res)
    }

    pub fn number_of_leading_zeros(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(J)I");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "numberOfLeadingZeros",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn number_of_trailing_zeros(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(J)I");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "numberOfTrailingZeros",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn bit_count(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(J)I");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "bitCount",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

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

    pub fn to_string_with_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a);
            args.push(val_1);
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code_with_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i64>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "J";
            let val_1 = jni::objects::JValueGen::Long(a);
            args.push(val_1);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "hashCode", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn min(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(JJ)J");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let val_2 = jni::objects::JValueGen::Long(arg1);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "min",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn max(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(JJ)J");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let val_2 = jni::objects::JValueGen::Long(arg1);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "max",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn signum(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(J)I");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "signum",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn reverse_bytes(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(J)J");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "reverseBytes",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn compare_to_with_long(&self, arg0: i64) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Long;";
        let val_1 = jni::objects::JValueGen::Object(self.jni_ref().new_object(
            "java/lang/Long",
            "(J)V",
            vec![arg0.into()],
        )?);
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "compareTo", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn get_long_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: std::option::Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/Long;";
            let val_2 = jni::objects::JValueGen::Object(jni.new_object(
                "java/lang/Long",
                "(J)V",
                vec![a.into()],
            )?);
            args.push(val_2);
        }
        sig += ")Ljava/lang/Long;";
        let cls = jni.find_class("java/lang/Long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "getLong", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn compare(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: i64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(JJ)I");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let val_2 = jni::objects::JValueGen::Long(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compare",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn byte_value(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "byteValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn short_value(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "shortValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn int_value(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "intValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn long_value(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn float_value(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "floatValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn double_value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "doubleValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn to_hex_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(J)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toHexString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn decode(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/lang/Long;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/lang/Long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "decode",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn describe_constable(
        &self,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Optional;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn reverse(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(J)J");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "reverse",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn sum(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(JJ)J");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let val_2 = jni::objects::JValueGen::Long(arg1);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "sum",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn compare_unsigned(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: i64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(JJ)I");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let val_2 = jni::objects::JValueGen::Long(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compareUnsigned",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn to_unsigned_string_with_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: std::option::Option<i32>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "J";
        let val_1 = jni::objects::JValueGen::Long(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toUnsignedString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_octal_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(J)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toOctalString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_binary_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(J)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toBinaryString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn divide_unsigned(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(JJ)J");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let val_2 = jni::objects::JValueGen::Long(arg1);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "divideUnsigned",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn remainder_unsigned(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(JJ)J");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let val_2 = jni::objects::JValueGen::Long(arg1);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "remainderUnsigned",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn highest_one_bit(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(J)J");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "highestOneBit",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn lowest_one_bit(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(J)J");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "lowestOneBit",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn rotate_left(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: i32,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(JI)J");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "rotateLeft",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn rotate_right(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
        arg1: i32,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(JI)J");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "rotateRight",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }
    //Number
    //crate::lang::JavaNumber
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The <code>String</code> class represents character strings. All
/// string literals in Java programs, such as <code>"abc"</code>, are
/// implemented as instances of this class.
/// <p>
/// Strings are constant; their values cannot be changed after they
/// are created. String buffers support mutable strings.
/// Because String objects are immutable they can be shared. For example:
/// </p><blockquote><pre> String str = "abc";
/// </pre></blockquote><p>
/// is equivalent to:
/// </p><blockquote><pre> char data[] = {'a', 'b', 'c'};
/// String str = new String(data);
/// </pre></blockquote><p>
/// Here are some more examples of how strings can be used:
/// </p><blockquote><pre> System.out.println("abc");
/// String cde = "cde";
/// System.out.println("abc" + cde);
/// String c = "abc".substring(2,3);
/// String d = cde.substring(1, 2);
/// </pre></blockquote>
/// <p>
/// The class <code>String</code> includes methods for examining
/// individual characters of the sequence, for comparing strings, for
/// searching strings, for extracting substrings, and for creating a
/// copy of a string with all characters translated to uppercase or to
/// lowercase. Case mapping is based on the Unicode Standard version
/// specified by the <a href="../../java/lang/Character.html" title="class in java.lang"><code>Character</code></a> class.
/// </p><p>
/// The Java language provides special support for the string
/// concatenation operator (&nbsp;+&nbsp;), and for conversion of
/// other objects to strings. String concatenation is implemented
/// through the <code>StringBuilder</code>(or <code>StringBuffer</code>)
/// class and its <code>append</code> method.
/// String conversions are implemented through the method
/// <code>toString</code>, defined by <code>Object</code> and
/// inherited by all classes in Java. For additional information on
/// string concatenation and conversion, see Gosling, Joy, and Steele,
/// <i>The Java Language Specification</i>.
/// </p><p> Unless otherwise noted, passing a <tt>null</tt> argument to a constructor
/// or method in this class will cause a <a title="class in java.lang" href="../../java/lang/NullPointerException.html"><code>NullPointerException</code></a> to be
/// thrown.
/// </p><p>A <code>String</code> represents a string in the UTF-16 format
/// in which <em>supplementary characters</em> are represented by <em>surrogate
/// pairs</em> (see the section <a href="Character.html#unicode">Unicode
/// Character Representations</a> in the <code>Character</code> class for
/// more information).
/// Index values refer to <code>char</code> code units, so a supplementary
/// character uses two positions in a <code>String</code>.
/// </p><p>The <code>String</code> class provides methods for dealing with
/// Unicode code points (i.e., characters), in addition to those for
/// dealing with Unicode code units (i.e., <code>char</code> values).</p>
#[repr(C)]
pub struct JavaString<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaString<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JavaString<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaString from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/lang/String")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaString object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaString<'mc> {
    #[deprecated]

    pub fn new_with_bytes(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<i8>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::lang::JavaString<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[B";
            let arr = jni.new_byte_array(a.len() as i32);
            let mut vec = Vec::new();
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = *a.get(i).unwrap();
                vec.push(val_1)
            }
            jni.set_byte_array_region(&arr, 0, &vec)?;
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaString::from_raw(&jni, res)
    }
    pub fn new_with_chars(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<u16>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::lang::JavaString<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[C";
            let arr = jni.new_char_array(a.len() as i32);
            let mut vec = Vec::new();
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = *a.get(i).unwrap();
                vec.push(val_1)
            }
            jni.set_char_array_region(&arr, 0, &vec)?;
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaString::from_raw(&jni, res)
    }
    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<String>>,
    ) -> Result<crate::lang::JavaString<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaString::from_raw(&jni, res)
    }
    pub fn new_with_ints(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<Vec<i32>>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<crate::lang::JavaString<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "[I";
            let arr = jni.new_int_array(a.len() as i32);
            let mut vec = Vec::new();
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_1 = *a.get(i).unwrap();
                vec.push(val_1)
            }
            jni.set_int_array_region(&arr, 0, &vec)?;
            let val_1 = jni::objects::JValueGen::Object(arr);
            args.push(val_1.l()?.into());
        }
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")V";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaString::from_raw(&jni, res)
    }

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

    pub fn length(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "length", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

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

    pub fn hash_code(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn get_chars(
        &self,
        arg0: i32,
        arg1: i32,
        arg2: Vec<u16>,
        arg3: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(II[CI)V");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let arr = self.jni_ref().new_char_array(arg2.len() as i32);
        let mut vec = Vec::new();
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg2.len() {
            let val_3 = *arg2.get(i).unwrap();
            vec.push(val_3)
        }
        self.jni_ref().set_char_array_region(&arr, 0, &vec)?;
        let val_3 = jni::objects::JValueGen::Object(arr);
        let val_4 = jni::objects::JValueGen::Int(arg3);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getChars",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3.l()?),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn compare_to_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "compareTo", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn index_of_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "indexOf", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn index_of_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "indexOf", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn char_at(&self, arg0: i32) -> Result<u16, Box<dyn std::error::Error>> {
        let sig = String::from("(I)C");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "charAt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.c()?)
    }

    pub fn code_point_at(&self, arg0: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "codePointAt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn code_point_before(&self, arg0: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "codePointBefore",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn code_point_count(
        &self,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "codePointCount",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn offset_by_code_points(
        &self,
        arg0: i32,
        arg1: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "offsetByCodePoints",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn region_matches_with_boolean(
        &self,
        arg0: bool,
        arg1: i32,
        arg2: impl Into<String>,
        arg3: i32,
        arg4: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Z";
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        args.push(val_1);
        sig += "I";
        let val_2 = jni::objects::JValueGen::Int(arg1);
        args.push(val_2);
        sig += "Ljava/lang/String;";
        let val_3 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg2.into())?,
        ));
        args.push(val_3);
        sig += "I";
        let val_4 = jni::objects::JValueGen::Int(arg3);
        args.push(val_4);
        if let Some(a) = arg4 {
            sig += "I";
            let val_5 = jni::objects::JValueGen::Int(a);
            args.push(val_5);
        }
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "regionMatches", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn starts_with_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Z";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "startsWith", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn last_index_of_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "lastIndexOf", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn last_index_of_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "lastIndexOf", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn substring_with_int(
        &self,
        arg0: i32,
        arg1: std::option::Option<i32>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "I";
        let val_1 = jni::objects::JValueGen::Int(arg0);
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "substring", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_empty(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEmpty", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn matches(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "matches",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn replace_first(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceFirst",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn replace_all(
        &self,
        arg0: impl Into<String>,
        arg1: impl Into<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg1.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "replaceAll",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_lower_case_with_locale(
        &self,
        arg0: std::option::Option<impl Into<crate::util::JavaLocale<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/Locale;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toLowerCase", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn to_upper_case_with_locale(
        &self,
        arg0: std::option::Option<impl Into<crate::util::JavaLocale<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Ljava/util/Locale;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")Ljava/lang/String;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "toUpperCase", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn trim(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "trim", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn strip(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "strip", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn strip_leading(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "stripLeading", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn strip_trailing(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "stripTrailing", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn repeat(&self, arg0: i32) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "repeat",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn is_blank(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isBlank", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn format_with_locale(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaLocale<'mc>>,
        arg1: impl Into<String>,
        arg2: std::option::Option<Vec<jni::objects::JObject<'mc>>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/util/Locale;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Ljava/lang/String;";
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        args.push(val_2);
        if let Some(a) = arg2 {
            sig += "[Ljava/lang/Object;";
            let arr = jni.new_object_array(
                a.len() as i32,
                "java/lang/Object",
                jni::objects::JObject::null(),
            );
            let arr = jni.translate_error_no_gen(arr)?;
            for i in 0..a.len() {
                let val_3 = jni::objects::JValueGen::Object(a.get(i).unwrap());
                jni.set_object_array_element(&arr, i as i32, val_3.l()?)?;
            }
            let val_3 = jni::objects::JValueGen::Object(arr);
            args.push(val_3.l()?.into());
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "format", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn equals_ignore_case(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equalsIgnoreCase",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn compare_to_ignore_case(
        &self,
        arg0: impl Into<String>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)I");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareToIgnoreCase",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn ends_with(&self, arg0: impl Into<String>) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "endsWith",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn concat(&self, arg0: impl Into<String>) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "concat",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn indent(&self, arg0: i32) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "indent",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn strip_indent(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "stripIndent", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn translate_escapes(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "translateEscapes",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn transform(
        &self,
        arg0: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Function;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "transform",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn formatted(
        &self,
        arg0: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("([Ljava/lang/Object;)Ljava/lang/String;");
        let arr = self.jni_ref().new_object_array(
            arg0.len() as i32,
            "java/lang/Object",
            jni::objects::JObject::null(),
        );
        let arr = self.jni_ref().translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = jni::objects::JValueGen::Object(arg0.get(i).unwrap());
            self.jni_ref()
                .set_object_array_element(&arr, i as i32, val_1.l()?)?;
        }
        let val_1 = jni::objects::JValueGen::Object(arr);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "formatted",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1.l()?)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn copy_value_of_with_chars(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: Vec<u16>,
        arg1: std::option::Option<i32>,
        arg2: std::option::Option<i32>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "[C";
        let arr = jni.new_char_array(arg0.len() as i32);
        let mut vec = Vec::new();
        let arr = jni.translate_error_no_gen(arr)?;
        for i in 0..arg0.len() {
            let val_1 = *arg0.get(i).unwrap();
            vec.push(val_1)
        }
        jni.set_char_array_region(&arr, 0, &vec)?;
        let val_1 = jni::objects::JValueGen::Object(arr);
        args.push(val_1.l()?.into());
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "I";
            let val_3 = jni::objects::JValueGen::Int(a);
            args.push(val_3);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "copyValueOf", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn intern(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "intern", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn describe_constable(
        &self,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Optional;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> std::string::ToString for JavaString<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling JavaString.toString: {}", err),
        }
    }
}

/// The Boolean class wraps a value of the primitive type
/// <code>boolean</code> in an object. An object of type
/// <code>Boolean</code> contains a single field whose type is
/// <code>boolean</code>.
/// <p>
/// In addition, this class provides many methods for
/// converting a <code>boolean</code> to a <code>String</code> and a
/// <code>String</code> to a <code>boolean</code>, as well as other
/// constants and methods useful when dealing with a
/// <code>boolean</code>.</p>
#[repr(C)]
pub struct JavaBoolean<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaBoolean<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JavaBoolean<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaBoolean from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/lang/Boolean")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaBoolean object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaBoolean<'mc> {
    #[deprecated]

    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::lang::JavaBoolean<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("java/lang/Boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaBoolean::from_raw(&jni, res)
    }

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

    pub fn to_string_with_boolean(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<bool>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code_with_boolean(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<bool>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "hashCode", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn compare_to_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "compareTo", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn get_boolean(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getBoolean",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn compare(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: bool,
        arg1: bool,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(ZZ)I");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compare",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn boolean_value(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "booleanValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn describe_constable(
        &self,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Optional;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn parse_boolean(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Z");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "parseBoolean",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn logical_and(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: bool,
        arg1: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(ZZ)Z");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "logicalAnd",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn logical_or(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: bool,
        arg1: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(ZZ)Z");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "logicalOr",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn logical_xor(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: bool,
        arg1: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(ZZ)Z");
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "logicalXor",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The <code>Double</code> class wraps a value of the primitive type
/// <code>double</code> in an object. An object of type
/// <code>Double</code> contains a single field whose type is
/// <code>double</code>.
/// <p>In addition, this class provides several methods for converting a
/// <code>double</code> to a <code>String</code> and a
/// <code>String</code> to a <code>double</code>, as well as other
/// constants and methods useful when dealing with a
/// <code>double</code>.</p>
#[repr(C)]
pub struct JavaDouble<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaDouble<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JavaDouble<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaDouble from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/lang/Double")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDouble object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaDouble<'mc> {
    #[deprecated]

    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::lang::JavaDouble<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("java/lang/Double");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaDouble::from_raw(&jni, res)
    }

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

    pub fn to_string_with_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f64>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a);
            args.push(val_1);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code_with_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f64>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a);
            args.push(val_1);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "hashCode", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn min(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: f64,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(DD)D");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let val_2 = jni::objects::JValueGen::Double(arg1);
        let cls = jni.find_class("double");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "min",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn max(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: f64,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(DD)D");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let val_2 = jni::objects::JValueGen::Double(arg1);
        let cls = jni.find_class("double");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "max",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn double_to_raw_long_bits(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(D)J");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "doubleToRawLongBits",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn double_to_long_bits(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(D)J");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "doubleToLongBits",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn long_bits_to_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i64,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(J)D");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let cls = jni.find_class("double");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "longBitsToDouble",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn compare_to_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "compareTo", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn compare(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: f64,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(DD)I");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let val_2 = jni::objects::JValueGen::Double(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compare",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn byte_value(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "byteValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn short_value(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "shortValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn int_value(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "intValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn long_value(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn float_value(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "floatValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn double_value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "doubleValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn to_hex_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("(D)Ljava/lang/String;");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toHexString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn describe_constable(
        &self,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Optional;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_na_n_with_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f64>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a);
            args.push(val_1);
        }
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isNaN", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn sum(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
        arg1: f64,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(DD)D");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let val_2 = jni::objects::JValueGen::Double(arg1);
        let cls = jni.find_class("double");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "sum",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn is_infinite_with_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<f64>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "D";
            let val_1 = jni::objects::JValueGen::Double(a);
            args.push(val_1);
        }
        sig += ")Z";
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "isInfinite", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn is_finite(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: f64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(D)Z");
        let val_1 = jni::objects::JValueGen::Double(arg0);
        let cls = jni.find_class("boolean");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isFinite",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn parse_double(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)D");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("double");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "parseDouble",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.d()?)
    }
    //Number
    //crate::lang::JavaNumber
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// The <code>Short</code> class wraps a value of primitive type <code>short</code> in an object.An object of type <code>Short</code> contains a
/// single field whose type is <code>short</code>.
/// <p>In addition, this class provides several methods for converting
/// a <code>short</code> to a <code>String</code> and a <code>String</code> to a
/// <code>short</code>, as well as other constants and methods useful when
/// dealing with a <code>short</code>.</p>
#[repr(C)]
pub struct JavaShort<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaShort<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for JavaShort<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaShort from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/lang/Short")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaShort object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaShort<'mc> {
    #[deprecated]

    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<crate::lang::JavaShort<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let cls = jni.find_class("java/lang/Short");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::lang::JavaShort::from_raw(&jni, res)
    }

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

    pub fn to_string_with_short(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i16>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "S";
            let val_1 = jni::objects::JValueGen::Short(a);
            args.push(val_1);
        }
        sig += ")Ljava/lang/String;";
        let cls = jni.find_class("java/lang/String");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "toString", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(jni
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn hash_code_with_short(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<i16>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = arg0 {
            sig += "S";
            let val_1 = jni::objects::JValueGen::Short(a);
            args.push(val_1);
        }
        sig += ")I";
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "hashCode", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn reverse_bytes(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i16,
    ) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("(S)S");
        let val_1 = jni::objects::JValueGen::Short(arg0);
        let cls = jni.find_class("short");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "reverseBytes",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn compare_to_with_object(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/Object;";
        let val_1 = jni::objects::JValueGen::Object(arg0);
        args.push(val_1);
        sig += ")I";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "compareTo", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn compare(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i16,
        arg1: i16,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(SS)I");
        let val_1 = jni::objects::JValueGen::Short(arg0);
        let val_2 = jni::objects::JValueGen::Short(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compare",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn byte_value(&self) -> Result<i8, Box<dyn std::error::Error>> {
        let sig = String::from("()B");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "byteValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.b()?)
    }

    pub fn short_value(&self) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("()S");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "shortValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn int_value(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "intValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn long_value(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "longValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn float_value(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "floatValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn double_value(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "doubleValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn decode(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<i16, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)Ljava/lang/Short;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        let cls = jni.find_class("java/lang/Short");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "decode",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.s()?)
    }

    pub fn describe_constable(
        &self,
    ) -> Result<crate::util::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Optional;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn to_unsigned_long(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i16,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(S)J");
        let val_1 = jni::objects::JValueGen::Short(arg0);
        let cls = jni.find_class("long");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toUnsignedLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn to_unsigned_int(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i16,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(S)I");
        let val_1 = jni::objects::JValueGen::Short(arg0);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "toUnsignedInt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn compare_unsigned(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: i16,
        arg1: i16,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(SS)I");
        let val_1 = jni::objects::JValueGen::Short(arg0);
        let val_2 = jni::objects::JValueGen::Short(arg1);
        let cls = jni.find_class("int");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "compareUnsigned",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn parse_short_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: std::option::Option<i32>,
    ) -> Result<i16, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "I";
            let val_2 = jni::objects::JValueGen::Int(a);
            args.push(val_2);
        }
        sig += ")S";
        let cls = jni.find_class("short");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "parseShort", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        Ok(res.s()?)
    }
    //Number
    //crate::lang::JavaNumber
    //Object

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
