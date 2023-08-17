#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIInstantiatableEnum;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents an object that may become a server operator, such as a <a title="interface in org.bukkit.entity" href="../entity/Player.html"><code>Player</code></a>
///
/// This is a representation of an abstract class.
pub struct ServerOperator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ServerOperator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for ServerOperator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ServerOperator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/permissions/ServerOperator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServerOperator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ServerOperator<'mc> {
    //

    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// Sets the operator status of this object
    pub fn set_op(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
/// Base Permissible for use in any Permissible object via proxy or extension
pub struct PermissibleBase<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PermissibleBase<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for PermissibleBase<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PermissibleBase from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/permissions/PermissibleBase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PermissibleBase object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PermissibleBase<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::permissions::ServerOperator<'mc>>,
    ) -> Result<crate::permissions::PermissibleBase<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/ServerOperator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/permissions/PermissibleBase");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::permissions::PermissibleBase::from_raw(&jni, res)
    }
    //

    pub fn is_permission_set_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_permission_with_permission(
        &self,
        arg0: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_attachment_with_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            // 2
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a.into());
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove_attachment(
        &self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn recalculate_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@NotNull

    pub fn effective_permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ServerOperator.html#setOp(boolean)">ServerOperator</a></code></span>
    /// Sets the operator status of this object
    pub fn set_op(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn clear_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clearPermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn wait_with_long(
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

impl<'mc> std::string::ToString for PermissibleBase<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PermissibleBase.toString: {}", err),
        }
    }
}

impl<'mc> Into<crate::permissions::Permissible<'mc>> for PermissibleBase<'mc> {
    fn into(self) -> crate::permissions::Permissible<'mc> {
        crate::permissions::Permissible::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PermissibleBase into crate::permissions::Permissible")
    }
}
#[derive(PartialEq, Eq)]
pub enum PermissionDefaultEnum {
    VariantTrue,
    VariantFalse,
    Op,
    NotOp,
}
impl std::fmt::Display for PermissionDefaultEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionDefaultEnum::VariantTrue => f.write_str("TRUE"),
            PermissionDefaultEnum::VariantFalse => f.write_str("FALSE"),
            PermissionDefaultEnum::Op => f.write_str("OP"),
            PermissionDefaultEnum::NotOp => f.write_str("NOT_OP"),
        }
    }
}
impl<'mc> std::fmt::Display for PermissionDefault<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.2.fmt(f)
    }
}
pub struct PermissionDefault<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub PermissionDefaultEnum,
);
impl<'mc> std::ops::Deref for PermissionDefault<'mc> {
    type Target = PermissionDefaultEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}

impl<'mc> JNIRaw<'mc> for PermissionDefault<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatableEnum<'mc> for PermissionDefault<'mc> {
    type Enum = PermissionDefaultEnum;

    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,

        e: Self::Enum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PermissionDefault from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/permissions/PermissionDefault")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PermissionDefault object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
}

impl<'mc> PermissionDefault<'mc> {
    pub const TRUE: PermissionDefaultEnum = PermissionDefaultEnum::VariantTrue;
    pub const FALSE: PermissionDefaultEnum = PermissionDefaultEnum::VariantFalse;
    pub const OP: PermissionDefaultEnum = PermissionDefaultEnum::Op;
    pub const NOT_OP: PermissionDefaultEnum = PermissionDefaultEnum::NotOp;
    pub fn from_string(str: String) -> std::option::Option<PermissionDefaultEnum> {
        match str.as_str() {
            "TRUE" => Some(PermissionDefaultEnum::VariantTrue),
            "FALSE" => Some(PermissionDefaultEnum::VariantFalse),
            "OP" => Some(PermissionDefaultEnum::Op),
            "NOT_OP" => Some(PermissionDefaultEnum::NotOp),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PermissionDefault<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/permissions/PermissionDefault");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/permissions/PermissionDefault;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = jni.translate_error(variant)?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        PermissionDefault::from_raw(
            &jni,
            raw_obj,
            PermissionDefault::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Represents an object that may be assigned permissions
///
/// This is a representation of an abstract class.
pub struct Permissible<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Permissible<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for Permissible<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Permissible from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/permissions/Permissible")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Permissible object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Permissible<'mc> {
    //

    pub fn is_permission_set_with_permission(
        &self,
        arg0: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn has_permission_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn add_attachment_with_plugin(
        &self,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<bool>,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Z";
            // 2
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a.into());
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn remove_attachment(
        &self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn recalculate_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@NotNull

    pub fn effective_permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //

    pub fn set_op(&self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        // -1
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::permissions::ServerOperator<'mc>> for Permissible<'mc> {
    fn into(self) -> crate::permissions::ServerOperator<'mc> {
        crate::permissions::ServerOperator::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Permissible into crate::permissions::ServerOperator")
    }
}
/// Represents a unique permission that may be attached to a <a title="interface in org.bukkit.permissions" href="Permissible.html"><code>Permissible</code></a>
pub struct Permission<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Permission<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for Permission<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Permission from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/permissions/Permission")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Permission object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Permission<'mc> {
    pub fn new_with_string(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
        arg1: std::option::Option<impl Into<String>>,
        arg2: std::option::Option<impl Into<crate::permissions::PermissionDefault<'mc>>>,
        arg3: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::permissions::Permission<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg0.into())?,
        ));
        args.push(val_1);
        if let Some(a) = arg1 {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = arg2 {
            sig += "Lorg/bukkit/permissions/PermissionDefault;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        if let Some(a) = arg3 {
            sig += "Ljava/util/Map;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/permissions/Permission");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::permissions::Permission::from_raw(&jni, res)
    }
    //

    pub fn set_default(
        &self,
        arg0: impl Into<crate::permissions::PermissionDefault<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionDefault;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDefault",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@NotNull

    pub fn children(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChildren", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //@NotNull

    pub fn name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getName", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //@NotNull

    pub fn default(
        &self,
    ) -> Result<crate::permissions::PermissionDefault<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/permissions/PermissionDefault;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDefault", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = self.jni_ref().translate_error(variant)?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::permissions::PermissionDefault::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::permissions::PermissionDefault::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
    //@NotNull

    pub fn description(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDescription", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    //

    pub fn recalculate_permissibles(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissibles",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_description(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDescription",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@NotNull

    pub fn permissibles(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPermissibles", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn add_parent_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: bool,
    ) -> Result<crate::permissions::Permission<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Z";
        // 3
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        args.push(val_2);
        sig += ")Lorg/bukkit/permissions/Permission;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addParent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::Permission::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn load_permissions(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
        arg1: impl Into<String>,
        arg2: impl Into<crate::permissions::PermissionDefault<'mc>>,
    ) -> Result<Vec<crate::permissions::Permission<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;Ljava/lang/String;Lorg/bukkit/permissions/PermissionDefault;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let cls = jni.find_class("java/util/List");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "loadPermissions",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&jni, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::permissions::Permission::from_raw(&jni, obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn wait_with_long(
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

impl<'mc> std::string::ToString for Permission<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling Permission.toString: {}", err),
        }
    }
}

/// Holds information on a permission and which <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> provides it
pub struct PermissionAttachmentInfo<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PermissionAttachmentInfo<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for PermissionAttachmentInfo<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PermissionAttachmentInfo from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/permissions/PermissionAttachmentInfo")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PermissionAttachmentInfo object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PermissionAttachmentInfo<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::permissions::Permissible<'mc>>,
        arg1: impl Into<String>,
        arg2: impl Into<crate::permissions::PermissionAttachment<'mc>>,
        arg3: bool,
    ) -> Result<crate::permissions::PermissionAttachmentInfo<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/Permissible;Ljava/lang/String;Lorg/bukkit/permissions/PermissionAttachment;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(arg1.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        // -1
        let val_4 = jni::objects::JValueGen::Bool(arg3.into());
        let cls = jni.find_class("org/bukkit/permissions/PermissionAttachmentInfo");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::permissions::PermissionAttachmentInfo::from_raw(&jni, res)
    }
    //@Nullable

    //@Nullable

    pub fn permission(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPermission", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            self.jni_ref()
                .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
                .to_string_lossy()
                .to_string(),
        ))
    }
    //@Deprecated

    #[deprecated]

    pub fn value(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //@NotNull

    pub fn attachment(
        &self,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/permissions/PermissionAttachment;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachment", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //@NotNull

    pub fn permissible(
        &self,
    ) -> Result<crate::permissions::Permissible<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/permissions/Permissible;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPermissible", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::Permissible::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn wait_with_long(
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

impl<'mc> std::string::ToString for PermissionAttachmentInfo<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PermissionAttachmentInfo.toString: {}", err),
        }
    }
}

/// Holds information about a permission attachment on a <a href="Permissible.html" title="interface in org.bukkit.permissions"><code>Permissible</code></a> object
pub struct PermissionAttachment<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PermissionAttachment<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for PermissionAttachment<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PermissionAttachment from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/permissions/PermissionAttachment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PermissionAttachment object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PermissionAttachment<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::plugin::Plugin<'mc>>,
        arg1: impl Into<crate::permissions::Permissible<'mc>>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;Lorg/bukkit/permissions/Permissible;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/permissions/PermissionAttachment");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::permissions::PermissionAttachment::from_raw(&jni, res)
    }
    //

    pub fn remove(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    //@NotNull

    pub fn permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPermissions", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn set_permission_with_string(
        &self,
        arg0: impl Into<String>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += "Z";
        // 3
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        args.push(val_2);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPermission", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@NotNull

    pub fn plugin(&self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/plugin/Plugin;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getPlugin", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //@Nullable

    pub fn removal_callback(
        &self,
    ) -> Result<
        Option<crate::permissions::PermissionRemovedExecutor<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/permissions/PermissionRemovedExecutor;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRemovalCallback",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            crate::permissions::PermissionRemovedExecutor::from_raw(&self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })?,
        ))
    }
    //

    pub fn set_removal_callback(
        &self,
        arg0: impl Into<crate::permissions::PermissionRemovedExecutor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionRemovedExecutor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRemovalCallback",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //@NotNull

    pub fn permissible(
        &self,
    ) -> Result<crate::permissions::Permissible<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/permissions/Permissible;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPermissible", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::Permissible::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn unset_permission_with_string(
        &self,
        arg0: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(arg0.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "unsetPermission", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn wait_with_long(
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

impl<'mc> std::string::ToString for PermissionAttachment<'mc> {
    fn to_string(&self) -> String {
        match &self.internal_to_string() {
            Ok(a) => a.clone(),
            Err(err) => format!("Error calling PermissionAttachment.toString: {}", err),
        }
    }
}

/// Represents a class which is to be notified when a <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> is removed from a <a title="interface in org.bukkit.permissions" href="Permissible.html"><code>Permissible</code></a>
///
/// This is a representation of an abstract class.
pub struct PermissionRemovedExecutor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PermissionRemovedExecutor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}

impl<'mc> JNIInstantiatable<'mc> for PermissionRemovedExecutor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PermissionRemovedExecutor from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/permissions/PermissionRemovedExecutor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PermissionRemovedExecutor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PermissionRemovedExecutor<'mc> {
    //

    pub fn attachment_removed(
        &self,
        arg0: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "attachmentRemoved",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
