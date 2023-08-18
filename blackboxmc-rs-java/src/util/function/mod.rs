#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents a supplier of <code>int</code>-valued results. This is the <code>int</code>-producing primitive specialization of <a title="interface in java.util.function" href="../../../java/util/function/Supplier.html"><code>Supplier</code></a>.
/// <p>There is no requirement that a distinct result be returned each time the supplier is invoked.</p>
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/IntSupplier.html#getAsInt--"><code>getAsInt()</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaIntSupplier<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaIntSupplier<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaIntSupplier<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaIntSupplier from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/IntSupplier")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaIntSupplier object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaIntSupplier<'mc> {
    pub fn as_int(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsInt", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
}
/// Represents a function that accepts a double-valued argument and produces a long-valued result. This is the <code>double</code>-to-<code>long</code> primitive specialization for <a title="interface in java.util.function" href="../../../java/util/function/Function.html"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/DoubleToLongFunction.html#applyAsLong-double-"><code>applyAsLong(double)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaDoubleToLongFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaDoubleToLongFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaDoubleToLongFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaDoubleToLongFunction from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/DoubleToLongFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDoubleToLongFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaDoubleToLongFunction<'mc> {
    pub fn apply_as_long(&self, arg0: f64) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(D)J");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
}
/// Represents an operation on a single <code>int</code>-valued operand that produces an <code>int</code>-valued result. This is the primitive type specialization of <a title="interface in java.util.function" href="../../../java/util/function/UnaryOperator.html"><code>UnaryOperator</code></a> for <code>int</code>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/IntUnaryOperator.html#applyAsInt-int-"><code>applyAsInt(int)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaIntUnaryOperator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaIntUnaryOperator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaIntUnaryOperator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaIntUnaryOperator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/IntUnaryOperator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaIntUnaryOperator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaIntUnaryOperator<'mc> {
    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaIntUnaryOperator<'mc>>,
    ) -> Result<crate::util::function::JavaIntUnaryOperator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Ljava/util/function/IntUnaryOperator;)Ljava/util/function/IntUnaryOperator;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaIntUnaryOperator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn identity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::function::JavaIntUnaryOperator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/function/IntUnaryOperator;");
        let cls = jni.find_class("java/util/function/IntUnaryOperator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "identity", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::function::JavaIntUnaryOperator::from_raw(&jni, obj)
    }

    pub fn apply_as_int(&self, arg0: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(I)I");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsInt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }

    pub fn compose(
        &self,
        arg0: impl Into<crate::util::function::JavaIntUnaryOperator<'mc>>,
    ) -> Result<crate::util::function::JavaIntUnaryOperator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Ljava/util/function/IntUnaryOperator;)Ljava/util/function/IntUnaryOperator;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compose",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaIntUnaryOperator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
/// Represents a function that accepts an int-valued argument and produces a double-valued result. This is the <code>int</code>-to-<code>double</code> primitive specialization for <a href="../../../java/util/function/Function.html" title="interface in java.util.function"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/IntToDoubleFunction.html#applyAsDouble-int-"><code>applyAsDouble(int)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaIntToDoubleFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaIntToDoubleFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaIntToDoubleFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaIntToDoubleFunction from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/IntToDoubleFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaIntToDoubleFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaIntToDoubleFunction<'mc> {
    pub fn apply_as_double(&self, arg0: i32) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(I)D");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsDouble",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
}
/// Represents an operation that accepts an object-valued and a <code>double</code>-valued argument, and returns no result. This is the <code>(reference, double)</code> specialization of <a href="../../../java/util/function/BiConsumer.html" title="interface in java.util.function"><code>BiConsumer</code></a>. Unlike most other functional interfaces, <code>ObjDoubleConsumer</code> is expected to operate via side-effects.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/ObjDoubleConsumer.html#accept-T-double-"><code>accept(Object, double)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaObjDoubleConsumer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaObjDoubleConsumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaObjDoubleConsumer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaObjDoubleConsumer from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/ObjDoubleConsumer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaObjDoubleConsumer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaObjDoubleConsumer<'mc> {
    pub fn accept(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;D)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "accept",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents a predicate (boolean-valued function) of one argument.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/Predicate.html#test-T-"><code>test(Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaPredicate<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaPredicate<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaPredicate<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaPredicate from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/Predicate")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaPredicate object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaPredicate<'mc> {
    pub fn is_equal(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::util::function::JavaPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/util/function/Predicate;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let cls = jni.find_class("java/util/function/Predicate");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "isEqual",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::function::JavaPredicate::from_raw(&jni, obj)
    }

    pub fn negate(
        &self,
    ) -> Result<crate::util::function::JavaPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/function/Predicate;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "negate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn and(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<crate::util::function::JavaPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "and",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn not(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<crate::util::function::JavaPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/function/Predicate");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "not",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::function::JavaPredicate::from_raw(&jni, obj)
    }

    pub fn or(
        &self,
        arg0: impl Into<crate::util::function::JavaPredicate<'mc>>,
    ) -> Result<crate::util::function::JavaPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "or",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn test(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "test",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
/// Represents a function that accepts one argument and produces a result.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/Function.html#apply-T-"><code>apply(Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaFunction from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/Function")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaFunction<'mc> {
    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<crate::util::function::JavaFunction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Function;)Ljava/util/function/Function;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaFunction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn apply(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn identity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::function::JavaFunction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/function/Function;");
        let cls = jni.find_class("java/util/function/Function");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "identity", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::function::JavaFunction::from_raw(&jni, obj)
    }

    pub fn compose(
        &self,
        arg0: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<crate::util::function::JavaFunction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Function;)Ljava/util/function/Function;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compose",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaFunction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
/// Represents a function that produces a long-valued result. This is the <code>long</code>-producing primitive specialization for <a href="../../../java/util/function/Function.html" title="interface in java.util.function"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/ToLongFunction.html#applyAsLong-T-"><code>applyAsLong(Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaToLongFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaToLongFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaToLongFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaToLongFunction from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/ToLongFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaToLongFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaToLongFunction<'mc> {
    pub fn apply_as_long(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)J");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
}
/// Represents an operation that accepts a single input argument and returns no result. Unlike most other functional interfaces, <code>Consumer</code> is expected to operate via side-effects.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/Consumer.html#accept-T-"><code>accept(Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaConsumer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaConsumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaConsumer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaConsumer from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/Consumer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaConsumer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaConsumer<'mc> {
    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaConsumer<'mc>>,
    ) -> Result<crate::util::function::JavaConsumer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Consumer;)Ljava/util/function/Consumer;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaConsumer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn accept(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "accept",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents a function that accepts two arguments and produces an int-valued result. This is the <code>int</code>-producing primitive specialization for <a title="interface in java.util.function" href="../../../java/util/function/BiFunction.html"><code>BiFunction</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/ToIntBiFunction.html#applyAsInt-T-U-"><code>applyAsInt(Object, Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaToIntBiFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaToIntBiFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaToIntBiFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaToIntBiFunction from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/ToIntBiFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaToIntBiFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaToIntBiFunction<'mc> {
    pub fn apply_as_int(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsInt",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
}
/// Represents an operation that accepts an object-valued and a <code>long</code>-valued argument, and returns no result. This is the <code>(reference, long)</code> specialization of <a title="interface in java.util.function" href="../../../java/util/function/BiConsumer.html"><code>BiConsumer</code></a>. Unlike most other functional interfaces, <code>ObjLongConsumer</code> is expected to operate via side-effects.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/ObjLongConsumer.html#accept-T-long-"><code>accept(Object, long)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaObjLongConsumer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaObjLongConsumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaObjLongConsumer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaObjLongConsumer from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/ObjLongConsumer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaObjLongConsumer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaObjLongConsumer<'mc> {
    pub fn accept(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;J)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Long(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "accept",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents a function that accepts a long-valued argument and produces an int-valued result. This is the <code>long</code>-to-<code>int</code> primitive specialization for <a title="interface in java.util.function" href="../../../java/util/function/Function.html"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/LongToIntFunction.html#applyAsInt-long-"><code>applyAsInt(long)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaLongToIntFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLongToIntFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLongToIntFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaLongToIntFunction from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/LongToIntFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLongToIntFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLongToIntFunction<'mc> {
    pub fn apply_as_int(&self, arg0: i64) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(J)I");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsInt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
}
/// Represents a supplier of results.
/// <p>There is no requirement that a new or distinct result be returned each time the supplier is invoked.</p>
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/Supplier.html#get--"><code>get()</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaSupplier<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaSupplier<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaSupplier<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate JavaSupplier from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/Supplier")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaSupplier object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaSupplier<'mc> {
    pub fn get(&self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/Object;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "get", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}
/// Represents an operation on a single operand that produces a result of the same type as its operand. This is a specialization of <code>Function</code> for the case where the operand and result are of the same type.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/Function.html#apply-T-"><code>Function.apply(Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaUnaryOperator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaUnaryOperator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaUnaryOperator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaUnaryOperator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/UnaryOperator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaUnaryOperator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaUnaryOperator<'mc> {
    pub fn identity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::function::JavaUnaryOperator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/function/UnaryOperator;");
        let cls = jni.find_class("java/util/function/UnaryOperator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "identity", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::function::JavaUnaryOperator::from_raw(&jni, obj)
    }

    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<crate::util::function::JavaFunction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Function;)Ljava/util/function/Function;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaFunction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn apply(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }

    pub fn compose(
        &self,
        arg0: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<crate::util::function::JavaFunction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Function;)Ljava/util/function/Function;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compose",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaFunction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> Into<crate::util::function::JavaFunction<'mc>> for JavaUnaryOperator<'mc> {
    fn into(self) -> crate::util::function::JavaFunction<'mc> {
        crate::util::function::JavaFunction::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting JavaUnaryOperator into crate::util::function::JavaFunction")
    }
}
/// Represents an operation that accepts an object-valued and a <code>int</code>-valued argument, and returns no result. This is the <code>(reference, int)</code> specialization of <a href="../../../java/util/function/BiConsumer.html" title="interface in java.util.function"><code>BiConsumer</code></a>. Unlike most other functional interfaces, <code>ObjIntConsumer</code> is expected to operate via side-effects.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/ObjIntConsumer.html#accept-T-int-"><code>accept(Object, int)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaObjIntConsumer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaObjIntConsumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaObjIntConsumer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaObjIntConsumer from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/ObjIntConsumer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaObjIntConsumer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaObjIntConsumer<'mc> {
    pub fn accept(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;I)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "accept",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents an operation upon two operands of the same type, producing a result of the same type as the operands. This is a specialization of <a title="interface in java.util.function" href="../../../java/util/function/BiFunction.html"><code>BiFunction</code></a> for the case where the operands and the result are all of the same type.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/BiFunction.html#apply-T-U-"><code>BiFunction.apply(Object, Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaBinaryOperator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaBinaryOperator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaBinaryOperator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaBinaryOperator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/BinaryOperator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaBinaryOperator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaBinaryOperator<'mc> {
    pub fn max_by(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaComparator<'mc>>,
    ) -> Result<crate::util::function::JavaBinaryOperator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Comparator;)Ljava/util/function/BinaryOperator;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/function/BinaryOperator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "maxBy",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::function::JavaBinaryOperator::from_raw(&jni, obj)
    }

    pub fn min_by(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::util::JavaComparator<'mc>>,
    ) -> Result<crate::util::function::JavaBinaryOperator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Comparator;)Ljava/util/function/BinaryOperator;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/function/BinaryOperator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "minBy",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::function::JavaBinaryOperator::from_raw(&jni, obj)
    }

    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<crate::util::function::JavaBiFunction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Function;)Ljava/util/function/BiFunction;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaBiFunction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn apply(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}
impl<'mc> Into<crate::util::function::JavaBiFunction<'mc>> for JavaBinaryOperator<'mc> {
    fn into(self) -> crate::util::function::JavaBiFunction<'mc> {
        crate::util::function::JavaBiFunction::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting JavaBinaryOperator into crate::util::function::JavaBiFunction",
        )
    }
}
/// Represents a supplier of <code>boolean</code>-valued results. This is the <code>boolean</code>-producing primitive specialization of <a title="interface in java.util.function" href="../../../java/util/function/Supplier.html"><code>Supplier</code></a>.
/// <p>There is no requirement that a new or distinct result be returned each time the supplier is invoked.</p>
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/BooleanSupplier.html#getAsBoolean--"><code>getAsBoolean()</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaBooleanSupplier<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaBooleanSupplier<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaBooleanSupplier<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaBooleanSupplier from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/BooleanSupplier")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaBooleanSupplier object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaBooleanSupplier<'mc> {
    pub fn as_boolean(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAsBoolean", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
/// Represents a function that produces a double-valued result. This is the <code>double</code>-producing primitive specialization for <a href="../../../java/util/function/Function.html" title="interface in java.util.function"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/ToDoubleFunction.html#applyAsDouble-T-"><code>applyAsDouble(Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaToDoubleFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaToDoubleFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaToDoubleFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaToDoubleFunction from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/ToDoubleFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaToDoubleFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaToDoubleFunction<'mc> {
    pub fn apply_as_double(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)D");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsDouble",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
}
/// Represents an operation on a single <code>double</code>-valued operand that produces a <code>double</code>-valued result. This is the primitive type specialization of <a title="interface in java.util.function" href="../../../java/util/function/UnaryOperator.html"><code>UnaryOperator</code></a> for <code>double</code>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/DoubleUnaryOperator.html#applyAsDouble-double-"><code>applyAsDouble(double)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaDoubleUnaryOperator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaDoubleUnaryOperator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaDoubleUnaryOperator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaDoubleUnaryOperator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/DoubleUnaryOperator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDoubleUnaryOperator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaDoubleUnaryOperator<'mc> {
    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaDoubleUnaryOperator<'mc>>,
    ) -> Result<crate::util::function::JavaDoubleUnaryOperator<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Ljava/util/function/DoubleUnaryOperator;)Ljava/util/function/DoubleUnaryOperator;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaDoubleUnaryOperator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn identity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::function::JavaDoubleUnaryOperator<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Ljava/util/function/DoubleUnaryOperator;");
        let cls = jni.find_class("java/util/function/DoubleUnaryOperator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "identity", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::function::JavaDoubleUnaryOperator::from_raw(&jni, obj)
    }

    pub fn apply_as_double(&self, arg0: f64) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(D)D");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsDouble",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }

    pub fn compose(
        &self,
        arg0: impl Into<crate::util::function::JavaDoubleUnaryOperator<'mc>>,
    ) -> Result<crate::util::function::JavaDoubleUnaryOperator<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Ljava/util/function/DoubleUnaryOperator;)Ljava/util/function/DoubleUnaryOperator;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compose",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaDoubleUnaryOperator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
/// Represents an operation upon two <code>long</code>-valued operands and producing a <code>long</code>-valued result. This is the primitive type specialization of <a href="../../../java/util/function/BinaryOperator.html" title="interface in java.util.function"><code>BinaryOperator</code></a> for <code>long</code>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/LongBinaryOperator.html#applyAsLong-long-long-"><code>applyAsLong(long, long)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaLongBinaryOperator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLongBinaryOperator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLongBinaryOperator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaLongBinaryOperator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/LongBinaryOperator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLongBinaryOperator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLongBinaryOperator<'mc> {
    pub fn apply_as_long(&self, arg0: i64, arg1: i64) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(JJ)J");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let val_2 = jni::objects::JValueGen::Long(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsLong",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
}
/// Represents a predicate (boolean-valued function) of one <code>int</code>-valued argument. This is the <code>int</code>-consuming primitive type specialization of <a title="interface in java.util.function" href="../../../java/util/function/Predicate.html"><code>Predicate</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/IntPredicate.html#test-int-"><code>test(int)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaIntPredicate<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaIntPredicate<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaIntPredicate<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaIntPredicate from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/IntPredicate")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaIntPredicate object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaIntPredicate<'mc> {
    pub fn negate(
        &self,
    ) -> Result<crate::util::function::JavaIntPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/function/IntPredicate;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "negate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaIntPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn and(
        &self,
        arg0: impl Into<crate::util::function::JavaIntPredicate<'mc>>,
    ) -> Result<crate::util::function::JavaIntPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/util/function/IntPredicate;)Ljava/util/function/IntPredicate;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "and",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaIntPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn or(
        &self,
        arg0: impl Into<crate::util::function::JavaIntPredicate<'mc>>,
    ) -> Result<crate::util::function::JavaIntPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/util/function/IntPredicate;)Ljava/util/function/IntPredicate;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "or",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaIntPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn test(&self, arg0: i32) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Z");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "test",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
/// Represents a predicate (boolean-valued function) of one <code>double</code>-valued argument. This is the <code>double</code>-consuming primitive type specialization of <a title="interface in java.util.function" href="../../../java/util/function/Predicate.html"><code>Predicate</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/DoublePredicate.html#test-double-"><code>test(double)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaDoublePredicate<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaDoublePredicate<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaDoublePredicate<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaDoublePredicate from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/DoublePredicate")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDoublePredicate object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaDoublePredicate<'mc> {
    pub fn negate(
        &self,
    ) -> Result<crate::util::function::JavaDoublePredicate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/function/DoublePredicate;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "negate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaDoublePredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn and(
        &self,
        arg0: impl Into<crate::util::function::JavaDoublePredicate<'mc>>,
    ) -> Result<crate::util::function::JavaDoublePredicate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Ljava/util/function/DoublePredicate;)Ljava/util/function/DoublePredicate;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "and",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaDoublePredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn or(
        &self,
        arg0: impl Into<crate::util::function::JavaDoublePredicate<'mc>>,
    ) -> Result<crate::util::function::JavaDoublePredicate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Ljava/util/function/DoublePredicate;)Ljava/util/function/DoublePredicate;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "or",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaDoublePredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn test(&self, arg0: f64) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(D)Z");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "test",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
/// Represents a function that accepts two arguments and produces a double-valued result. This is the <code>double</code>-producing primitive specialization for <a title="interface in java.util.function" href="../../../java/util/function/BiFunction.html"><code>BiFunction</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/ToDoubleBiFunction.html#applyAsDouble-T-U-"><code>applyAsDouble(Object, Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaToDoubleBiFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaToDoubleBiFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaToDoubleBiFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaToDoubleBiFunction from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/ToDoubleBiFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaToDoubleBiFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaToDoubleBiFunction<'mc> {
    pub fn apply_as_double(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)D");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsDouble",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
}
/// Represents a predicate (boolean-valued function) of one <code>long</code>-valued argument. This is the <code>long</code>-consuming primitive type specialization of <a href="../../../java/util/function/Predicate.html" title="interface in java.util.function"><code>Predicate</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/LongPredicate.html#test-long-"><code>test(long)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaLongPredicate<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLongPredicate<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLongPredicate<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLongPredicate from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/LongPredicate")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLongPredicate object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLongPredicate<'mc> {
    pub fn negate(
        &self,
    ) -> Result<crate::util::function::JavaLongPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/function/LongPredicate;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "negate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaLongPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn and(
        &self,
        arg0: impl Into<crate::util::function::JavaLongPredicate<'mc>>,
    ) -> Result<crate::util::function::JavaLongPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/util/function/LongPredicate;)Ljava/util/function/LongPredicate;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "and",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaLongPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn or(
        &self,
        arg0: impl Into<crate::util::function::JavaLongPredicate<'mc>>,
    ) -> Result<crate::util::function::JavaLongPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/util/function/LongPredicate;)Ljava/util/function/LongPredicate;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "or",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaLongPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn test(&self, arg0: i64) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(J)Z");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "test",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
/// Represents an operation that accepts a single <code>int</code>-valued argument and returns no result. This is the primitive type specialization of <a title="interface in java.util.function" href="../../../java/util/function/Consumer.html"><code>Consumer</code></a> for <code>int</code>. Unlike most other functional interfaces, <code>IntConsumer</code> is expected to operate via side-effects.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/IntConsumer.html#accept-int-"><code>accept(int)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaIntConsumer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaIntConsumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaIntConsumer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaIntConsumer from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/IntConsumer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaIntConsumer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaIntConsumer<'mc> {
    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaIntConsumer<'mc>>,
    ) -> Result<crate::util::function::JavaIntConsumer<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/util/function/IntConsumer;)Ljava/util/function/IntConsumer;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaIntConsumer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn accept(&self, arg0: i32) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(I)V");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "accept",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents a function that accepts a long-valued argument and produces a double-valued result. This is the <code>long</code>-to-<code>double</code> primitive specialization for <a title="interface in java.util.function" href="../../../java/util/function/Function.html"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/LongToDoubleFunction.html#applyAsDouble-long-"><code>applyAsDouble(long)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaLongToDoubleFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLongToDoubleFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLongToDoubleFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaLongToDoubleFunction from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/LongToDoubleFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLongToDoubleFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLongToDoubleFunction<'mc> {
    pub fn apply_as_double(&self, arg0: i64) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(J)D");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsDouble",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
}
/// Represents a function that accepts a double-valued argument and produces a result. This is the <code>double</code>-consuming primitive specialization for <a title="interface in java.util.function" href="../../../java/util/function/Function.html"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/DoubleFunction.html#apply-double-"><code>apply(double)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaDoubleFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaDoubleFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaDoubleFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaDoubleFunction from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/DoubleFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDoubleFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaDoubleFunction<'mc> {
    pub fn apply(
        &self,
        arg0: f64,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(D)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}
/// Represents an operation that accepts a single <code>long</code>-valued argument and returns no result. This is the primitive type specialization of <a title="interface in java.util.function" href="../../../java/util/function/Consumer.html"><code>Consumer</code></a> for <code>long</code>. Unlike most other functional interfaces, <code>LongConsumer</code> is expected to operate via side-effects.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/LongConsumer.html#accept-long-"><code>accept(long)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaLongConsumer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLongConsumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLongConsumer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLongConsumer from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/LongConsumer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLongConsumer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLongConsumer<'mc> {
    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaLongConsumer<'mc>>,
    ) -> Result<crate::util::function::JavaLongConsumer<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/util/function/LongConsumer;)Ljava/util/function/LongConsumer;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaLongConsumer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn accept(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "accept",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents an operation that accepts a single <code>double</code>-valued argument and returns no result. This is the primitive type specialization of <a href="../../../java/util/function/Consumer.html" title="interface in java.util.function"><code>Consumer</code></a> for <code>double</code>. Unlike most other functional interfaces, <code>DoubleConsumer</code> is expected to operate via side-effects.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/DoubleConsumer.html#accept-double-"><code>accept(double)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaDoubleConsumer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaDoubleConsumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaDoubleConsumer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaDoubleConsumer from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/DoubleConsumer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDoubleConsumer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaDoubleConsumer<'mc> {
    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaDoubleConsumer<'mc>>,
    ) -> Result<crate::util::function::JavaDoubleConsumer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Ljava/util/function/DoubleConsumer;)Ljava/util/function/DoubleConsumer;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaDoubleConsumer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn accept(&self, arg0: f64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(D)V");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "accept",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents an operation upon two <code>double</code>-valued operands and producing a <code>double</code>-valued result. This is the primitive type specialization of <a title="interface in java.util.function" href="../../../java/util/function/BinaryOperator.html"><code>BinaryOperator</code></a> for <code>double</code>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/DoubleBinaryOperator.html#applyAsDouble-double-double-"><code>applyAsDouble(double, double)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaDoubleBinaryOperator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaDoubleBinaryOperator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaDoubleBinaryOperator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaDoubleBinaryOperator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/DoubleBinaryOperator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDoubleBinaryOperator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaDoubleBinaryOperator<'mc> {
    pub fn apply_as_double(&self, arg0: f64, arg1: f64) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("(DD)D");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let val_2 = jni::objects::JValueGen::Double(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsDouble",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
}
/// Represents a function that accepts a long-valued argument and produces a result. This is the <code>long</code>-consuming primitive specialization for <a href="../../../java/util/function/Function.html" title="interface in java.util.function"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/LongFunction.html#apply-long-"><code>apply(long)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaLongFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLongFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLongFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLongFunction from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/LongFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLongFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLongFunction<'mc> {
    pub fn apply(
        &self,
        arg0: i64,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(J)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}
/// Represents a function that accepts two arguments and produces a result. This is the two-arity specialization of <a title="interface in java.util.function" href="../../../java/util/function/Function.html"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/BiFunction.html#apply-T-U-"><code>apply(Object, Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaBiFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaBiFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaBiFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaBiFunction from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/BiFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaBiFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaBiFunction<'mc> {
    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaFunction<'mc>>,
    ) -> Result<crate::util::function::JavaBiFunction<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/Function;)Ljava/util/function/BiFunction;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaBiFunction::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn apply(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}
/// Represents a supplier of <code>double</code>-valued results. This is the <code>double</code>-producing primitive specialization of <a title="interface in java.util.function" href="../../../java/util/function/Supplier.html"><code>Supplier</code></a>.
/// <p>There is no requirement that a distinct result be returned each time the supplier is invoked.</p>
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/DoubleSupplier.html#getAsDouble--"><code>getAsDouble()</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaDoubleSupplier<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaDoubleSupplier<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaDoubleSupplier<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaDoubleSupplier from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/DoubleSupplier")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDoubleSupplier object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaDoubleSupplier<'mc> {
    pub fn as_double(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let sig = String::from("()D");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAsDouble", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.d()?)
    }
}
/// Represents a function that accepts an int-valued argument and produces a result. This is the <code>int</code>-consuming primitive specialization for <a href="../../../java/util/function/Function.html" title="interface in java.util.function"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/IntFunction.html#apply-int-"><code>apply(int)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaIntFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaIntFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaIntFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaIntFunction from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/IntFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaIntFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaIntFunction<'mc> {
    pub fn apply(
        &self,
        arg0: i32,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Ljava/lang/Object;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "apply",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l()?)
    }
}
/// Represents an operation upon two <code>int</code>-valued operands and producing an <code>int</code>-valued result. This is the primitive type specialization of <a title="interface in java.util.function" href="../../../java/util/function/BinaryOperator.html"><code>BinaryOperator</code></a> for <code>int</code>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/IntBinaryOperator.html#applyAsInt-int-int-"><code>applyAsInt(int, int)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaIntBinaryOperator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaIntBinaryOperator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaIntBinaryOperator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaIntBinaryOperator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/IntBinaryOperator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaIntBinaryOperator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaIntBinaryOperator<'mc> {
    pub fn apply_as_int(&self, arg0: i32, arg1: i32) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(II)I");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let val_2 = jni::objects::JValueGen::Int(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsInt",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
}
/// Represents an operation on a single <code>long</code>-valued operand that produces a <code>long</code>-valued result. This is the primitive type specialization of <a title="interface in java.util.function" href="../../../java/util/function/UnaryOperator.html"><code>UnaryOperator</code></a> for <code>long</code>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/LongUnaryOperator.html#applyAsLong-long-"><code>applyAsLong(long)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaLongUnaryOperator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLongUnaryOperator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLongUnaryOperator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaLongUnaryOperator from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/LongUnaryOperator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLongUnaryOperator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLongUnaryOperator<'mc> {
    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaLongUnaryOperator<'mc>>,
    ) -> Result<crate::util::function::JavaLongUnaryOperator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Ljava/util/function/LongUnaryOperator;)Ljava/util/function/LongUnaryOperator;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaLongUnaryOperator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn identity(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::util::function::JavaLongUnaryOperator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/function/LongUnaryOperator;");
        let cls = jni.find_class("java/util/function/LongUnaryOperator");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "identity", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::util::function::JavaLongUnaryOperator::from_raw(&jni, obj)
    }

    pub fn apply_as_long(&self, arg0: i64) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(J)J");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn compose(
        &self,
        arg0: impl Into<crate::util::function::JavaLongUnaryOperator<'mc>>,
    ) -> Result<crate::util::function::JavaLongUnaryOperator<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Ljava/util/function/LongUnaryOperator;)Ljava/util/function/LongUnaryOperator;",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compose",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaLongUnaryOperator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
/// Represents a function that accepts an int-valued argument and produces a long-valued result. This is the <code>int</code>-to-<code>long</code> primitive specialization for <a href="../../../java/util/function/Function.html" title="interface in java.util.function"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/IntToLongFunction.html#applyAsLong-int-"><code>applyAsLong(int)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaIntToLongFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaIntToLongFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaIntToLongFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaIntToLongFunction from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/IntToLongFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaIntToLongFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaIntToLongFunction<'mc> {
    pub fn apply_as_long(&self, arg0: i32) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(I)J");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsLong",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
}
/// Represents a function that produces an int-valued result. This is the <code>int</code>-producing primitive specialization for <a href="../../../java/util/function/Function.html" title="interface in java.util.function"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/ToIntFunction.html#applyAsInt-T-"><code>applyAsInt(Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaToIntFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaToIntFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaToIntFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaToIntFunction from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/ToIntFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaToIntFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaToIntFunction<'mc> {
    pub fn apply_as_int(
        &self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;)I");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsInt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
}
/// Represents a supplier of <code>long</code>-valued results. This is the <code>long</code>-producing primitive specialization of <a href="../../../java/util/function/Supplier.html" title="interface in java.util.function"><code>Supplier</code></a>.
/// <p>There is no requirement that a distinct result be returned each time the supplier is invoked.</p>
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/LongSupplier.html#getAsLong--"><code>getAsLong()</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaLongSupplier<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaLongSupplier<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaLongSupplier<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaLongSupplier from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/LongSupplier")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaLongSupplier object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaLongSupplier<'mc> {
    pub fn as_long(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getAsLong", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
}
/// Represents a function that accepts two arguments and produces a long-valued result. This is the <code>long</code>-producing primitive specialization for <a title="interface in java.util.function" href="../../../java/util/function/BiFunction.html"><code>BiFunction</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/ToLongBiFunction.html#applyAsLong-T-U-"><code>applyAsLong(Object, Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaToLongBiFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaToLongBiFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaToLongBiFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaToLongBiFunction from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/ToLongBiFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaToLongBiFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaToLongBiFunction<'mc> {
    pub fn apply_as_long(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)J");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsLong",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
}
/// Represents an operation that accepts two input arguments and returns no result. This is the two-arity specialization of <a href="../../../java/util/function/Consumer.html" title="interface in java.util.function"><code>Consumer</code></a>. Unlike most other functional interfaces, <code>BiConsumer</code> is expected to operate via side-effects.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/BiConsumer.html#accept-T-U-"><code>accept(Object, Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaBiConsumer<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaBiConsumer<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaBiConsumer<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaBiConsumer from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/BiConsumer")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaBiConsumer object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaBiConsumer<'mc> {
    pub fn from_rust_fn<'mc2, A, B, C>(
        env: &'mc2 blackboxmc_general::SharedJNIEnv<'mc>,
        f: A,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        A: Fn(&'mc2 blackboxmc_general::SharedJNIEnv<'mc>, B, C),
        B: blackboxmc_general::JNIInstantiatable<'mc>,
        C: blackboxmc_general::JNIInstantiatable<'mc>,
    {
        // Create the closure.
        let closure: &_ = Box::leak(Box::new(
            move |a: *mut jni::sys::_jobject, b: *mut jni::sys::_jobject| unsafe {
                f(
                    env,
                    B::from_raw(&env, jni::objects::JObject::from_raw(a)).unwrap(),
                    C::from_raw(&env, jni::objects::JObject::from_raw(b)).unwrap(),
                );
            },
        ));
        let callback = libffi::high::Closure2::new(closure);
        let &code = callback.code_ptr();
        let ptr: unsafe extern "C" fn(*const jni::sys::jobject) =
            unsafe { std::mem::transmute(code) };
        std::mem::forget(callback);

        // Create the java object.
        let obj = env.new_object(
            "net/ioixd/blackbox/BiConsumerLink",
            "(J)V",
            vec![(ptr as i64).into()],
        );
        let obj = env.translate_error_no_gen(obj)?;

        Self::from_raw(&env, obj)
    }

    pub fn and_then(
        &self,
        arg0: impl Into<crate::util::function::JavaBiConsumer<'mc>>,
    ) -> Result<crate::util::function::JavaBiConsumer<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/function/BiConsumer;)Ljava/util/function/BiConsumer;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "andThen",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaBiConsumer::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn accept(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)V");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "accept",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Represents a predicate (boolean-valued function) of two arguments. This is the two-arity specialization of <a title="interface in java.util.function" href="../../../java/util/function/Predicate.html"><code>Predicate</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/BiPredicate.html#test-T-U-"><code>test(Object, Object)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaBiPredicate<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaBiPredicate<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaBiPredicate<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate JavaBiPredicate from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/BiPredicate")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaBiPredicate object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaBiPredicate<'mc> {
    pub fn negate(
        &self,
    ) -> Result<crate::util::function::JavaBiPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/function/BiPredicate;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "negate", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaBiPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn and(
        &self,
        arg0: impl Into<crate::util::function::JavaBiPredicate<'mc>>,
    ) -> Result<crate::util::function::JavaBiPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/util/function/BiPredicate;)Ljava/util/function/BiPredicate;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "and",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaBiPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn or(
        &self,
        arg0: impl Into<crate::util::function::JavaBiPredicate<'mc>>,
    ) -> Result<crate::util::function::JavaBiPredicate<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/util/function/BiPredicate;)Ljava/util/function/BiPredicate;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "or",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::util::function::JavaBiPredicate::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn test(
        &self,
        arg0: jni::objects::JObject<'mc>,
        arg1: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/Object;Ljava/lang/Object;)Z");
        let val_1 = jni::objects::JValueGen::Object(arg0);
        let val_2 = jni::objects::JValueGen::Object(arg1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "test",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
}
/// Represents a function that accepts a double-valued argument and produces an int-valued result. This is the <code>double</code>-to-<code>int</code> primitive specialization for <a title="interface in java.util.function" href="../../../java/util/function/Function.html"><code>Function</code></a>.
/// <p>This is a <a href="package-summary.html">functional interface</a> whose functional method is <a href="../../../java/util/function/DoubleToIntFunction.html#applyAsInt-double-"><code>applyAsInt(double)</code></a>.</p>
///
/// This is a representation of an abstract class.
pub struct JavaDoubleToIntFunction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for JavaDoubleToIntFunction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for JavaDoubleToIntFunction<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate JavaDoubleToIntFunction from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "java/util/function/DoubleToIntFunction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a JavaDoubleToIntFunction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> JavaDoubleToIntFunction<'mc> {
    pub fn apply_as_int(&self, arg0: f64) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("(D)I");
        let val_1 = jni::objects::JValueGen::Double(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "applyAsInt",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
}
