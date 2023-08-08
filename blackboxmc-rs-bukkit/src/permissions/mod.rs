#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Represents an object that may become a server operator, such as a <a title="interface in org.bukkit.entity" href="../entity/Player.html"><code>Player</code></a>
///
/// This is a representation of an abstract class.
pub struct ServerOperator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ServerOperator<'mc> {
    pub fn from_raw(
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
    /// Checks if this object is a server operator
    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Sets the operator status of this object
    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for ServerOperator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Base Permissible for use in any Permissible object via proxy or extension
pub struct PermissibleBase<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PermissibleBase<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PermissibleBase<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::permissions::ServerOperator<'mc>>,
    ) -> Result<crate::permissions::PermissibleBase<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/permissions/PermissibleBase")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/permissions/ServerOperator;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::permissions::PermissibleBase::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Permissible.html#isPermissionSet(java.lang.String)">Permissible</a></code></span>
    /// Checks if this object contains an override for the specified permission, by fully qualified name
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Permissible.html#isPermissionSet(org.bukkit.permissions.Permission)">Permissible</a></code></span>
    /// Checks if this object contains an override for the specified <a href="Permission.html" title="class in org.bukkit.permissions"><code>Permission</code></a>
    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPermissionSet",
            "(Lorg/bukkit/permissions/Permission;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Permissible.html#hasPermission(java.lang.String)">Permissible</a></code></span>
    /// Gets the value of the specified permission, if set.
    /// <p>If a permission override is not set on this object, the default value of the permission will be returned.</p>
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Permissible.html#hasPermission(org.bukkit.permissions.Permission)">Permissible</a></code></span>
    /// Gets the value of the specified permission, if set.
    /// <p>If a permission override is not set on this object, the default value of the permission will be returned</p>
    pub fn has_permission_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasPermission",
            "(Lorg/bukkit/permissions/Permission;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Permissible.html#addAttachment(org.bukkit.plugin.Plugin,java.lang.String,boolean)">Permissible</a></code></span>
    /// Adds a new <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> with a single permission by name and value
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Permissible.html#addAttachment(org.bukkit.plugin.Plugin)">Permissible</a></code></span>
    /// Adds a new empty <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> to this object
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Permissible.html#addAttachment(org.bukkit.plugin.Plugin,java.lang.String,boolean,int)">Permissible</a></code></span>
    /// Temporarily adds a new <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> with a single permission by name and value
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Permissible.html#addAttachment(org.bukkit.plugin.Plugin,int)">Permissible</a></code></span>
    /// Temporarily adds a new empty <a title="class in org.bukkit.permissions" href="PermissionAttachment.html"><code>PermissionAttachment</code></a> to this object
    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        // 3
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(&self.jni_object(),"addAttachment","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;ZI)Lorg/bukkit/permissions/PermissionAttachment;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Permissible.html#removeAttachment(org.bukkit.permissions.PermissionAttachment)">Permissible</a></code></span>
    /// Removes the given <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> from this object
    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<&'mc crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            "(Lorg/bukkit/permissions/PermissionAttachment;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Permissible.html#recalculatePermissions()">Permissible</a></code></span>
    /// Recalculates the permissions for this object, if the attachments have changed values.
    /// <p>This should very rarely need to be called from a plugin.</p>
    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "recalculatePermissions", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Permissible.html#getEffectivePermissions()">Permissible</a></code></span>
    /// Gets a set containing all of the permissions currently in effect by this object
    pub fn effective_permissions(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ServerOperator.html#isOp()">ServerOperator</a></code></span>
    /// Checks if this object is a server operator
    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="ServerOperator.html#setOp(boolean)">ServerOperator</a></code></span>
    /// Sets the operator status of this object
    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clearPermissions", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
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
impl<'mc> Into<crate::permissions::Permissible<'mc>> for PermissibleBase<'mc> {
    fn into(self) -> crate::permissions::Permissible<'mc> {
        crate::permissions::Permissible::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum PermissionDefaultEnum {
    VariantTrue,
    VariantFalse,
    Op,
    NotOp,
}
impl std::fmt::Display for PermissionDefaultEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PermissionDefaultEnum::VariantTrue => f.write_str("TRUE"),
            PermissionDefaultEnum::VariantFalse => f.write_str("FALSE"),
            PermissionDefaultEnum::Op => f.write_str("OP"),
            PermissionDefaultEnum::NotOp => f.write_str("NOT_OP"),
        }
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
impl<'mc> PermissionDefault<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: PermissionDefaultEnum,
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
}
/// Represents an object that may be assigned permissions
///
/// This is a representation of an abstract class.
pub struct Permissible<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Permissible<'mc> {
    pub fn from_raw(
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
    /// Checks if this object contains an override for the specified permission, by fully qualified name
    /// Checks if this object contains an override for the specified <a href="Permission.html" title="class in org.bukkit.permissions"><code>Permission</code></a>
    pub fn is_permission_set_with_string(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::permissions::Permission<'mc>>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPermissionSet",
            "(Lorg/bukkit/permissions/Permission;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the value of the specified permission, if set.
    /// <p>If a permission override is not set on this object, the default value of the permission will be returned.</p>
    /// Gets the value of the specified permission, if set.
    /// <p>If a permission override is not set on this object, the default value of the permission will be returned</p>
    pub fn has_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "hasPermission",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Adds a new <a title="class in org.bukkit.permissions" href="PermissionAttachment.html"><code>PermissionAttachment</code></a> with a single permission by name and value
    /// Adds a new empty <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> to this object
    /// Temporarily adds a new <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> with a single permission by name and value
    /// Temporarily adds a new empty <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> to this object
    pub fn add_attachment_with_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc String>,
        arg2: bool,
        arg3: std::option::Option<i32>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        // 3
        let val_3 = jni::objects::JValueGen::Bool(arg2.into());
        let val_4 = jni::objects::JValueGen::Int(arg3.unwrap().into());
        let res = self.jni_ref().call_method(&self.jni_object(),"addAttachment","(Lorg/bukkit/plugin/Plugin;Ljava/lang/String;ZI)Lorg/bukkit/permissions/PermissionAttachment;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Removes the given <a title="class in org.bukkit.permissions" href="PermissionAttachment.html"><code>PermissionAttachment</code></a> from this object
    pub fn remove_attachment(
        &mut self,
        arg0: impl Into<&'mc crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            "(Lorg/bukkit/permissions/PermissionAttachment;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Recalculates the permissions for this object, if the attachments have changed values.
    /// <p>This should very rarely need to be called from a plugin.</p>
    pub fn recalculate_permissions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "recalculatePermissions", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a set containing all of the permissions currently in effect by this object
    pub fn effective_permissions(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_op(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_op(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Permissible<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::permissions::ServerOperator<'mc>> for Permissible<'mc> {
    fn into(self) -> crate::permissions::ServerOperator<'mc> {
        crate::permissions::ServerOperator::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a unique permission that may be attached to a <a href="Permissible.html" title="interface in org.bukkit.permissions"><code>Permissible</code></a>
pub struct Permission<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Permission<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Permission<'mc> {
    pub fn from_raw(
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
    pub fn new_with_string(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc String>,
        arg2: impl Into<&'mc crate::permissions::PermissionDefault<'mc>>,
        arg3: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
    ) -> Result<crate::permissions::Permission<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 =
            unsafe { jni::objects::JObject::from_raw(arg3.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/permissions/Permission")?;
        let res = jni.new_object(cls,
"(Ljava/lang/String;Ljava/lang/String;Lorg/bukkit/permissions/PermissionDefault;Ljava/util/Map;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
        crate::permissions::Permission::from_raw(&jni, res)
    }
    /// Sets the default value of this permission.
    /// <p>This will not be saved to disk, and is a temporary operation until the server reloads permissions. Changing this default will cause all <a href="Permissible.html" title="interface in org.bukkit.permissions"><code>Permissible</code></a>s that contain this permission to recalculate their permissions</p>
    pub fn set_default(
        &mut self,
        arg0: impl Into<&'mc crate::permissions::PermissionDefault<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDefault",
            "(Lorg/bukkit/permissions/PermissionDefault;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the children of this permission.
    /// <p>If you change this map in any form, you must call <a href="#recalculatePermissibles()"><code>recalculatePermissibles()</code></a> to recalculate all <a title="interface in org.bukkit.permissions" href="Permissible.html"><code>Permissible</code></a>s</p>
    pub fn children(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChildren", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a brief description of this permission, may be empty
    pub fn description(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDescription",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Recalculates all <a title="interface in org.bukkit.permissions" href="Permissible.html"><code>Permissible</code></a>s that contain this permission.
    /// <p>This should be called after modifying the children, and is automatically called after modifying the default value</p>
    pub fn recalculate_permissibles(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "recalculatePermissibles", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Sets the description of this permission.
    /// <p>This will not be saved to disk, and is a temporary operation until the server reloads permissions.</p>
    pub fn set_description(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDescription",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a set containing every <a href="Permissible.html" title="interface in org.bukkit.permissions"><code>Permissible</code></a> that has this permission.
    /// <p>This set cannot be modified.</p>
    pub fn permissibles(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermissibles",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Adds this permission to the specified parent permission.
    /// <p>If the parent permission does not exist, it will be created and registered.</p>
    /// Adds this permission to the specified parent permission.
    pub fn add_parent_with_string(
        &mut self,
        arg0: impl Into<&'mc crate::permissions::Permission<'mc>>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        // 1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addParent",
            "(Lorg/bukkit/permissions/Permission;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Loads a list of Permissions from a map of data, usually used from retrieval from a yaml file.
    /// <p>The data may contain a list of name:data, where the data contains the following keys:</p>
    /// <ul>
    /// <li>default: Boolean true or false. If not specified, false.</li>
    /// <li>children: <code>Map&lt;String, Boolean&gt;</code> of child permissions. If not specified, empty list.</li>
    /// <li>description: Short string containing a very small description of this description. If not specified, empty string.</li>
    /// </ul>
    pub fn load_permissions(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc blackboxmc_java::JavaMap<'mc>>,
        arg1: impl Into<&'mc String>,
        arg2: impl Into<&'mc crate::permissions::PermissionDefault<'mc>>,
    ) -> Result<Vec<crate::permissions::Permission<'mc>>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let cls = &jni.find_class("java/util/List")?;
        let res = jni.call_static_method(cls,"loadPermissions",
"(Ljava/util/Map;Ljava/lang/String;Lorg/bukkit/permissions/PermissionDefault;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)])?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&jni, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::permissions::Permission::from_raw(&jni, obj)?);
        }
        Ok(new_vec)
    }
    /// Loads a list of Permissions from a map of data, usually used from retrieval from a yaml file.
    /// <p>The data may contain a list of name:data, where the data contains the following keys:</p>
    /// <ul>
    /// <li>default: Boolean true or false. If not specified, false.</li>
    /// <li>children: <code>Map&lt;String, Boolean&gt;</code> of child permissions. If not specified, empty list.</li>
    /// <li>description: Short string containing a very small description of this description. If not specified, empty string.</li>
    /// </ul>
    /// Loads a Permission from a map of data, usually used from retrieval from a yaml file.
    /// <p>The data may contain the following keys:</p>
    /// <ul>
    /// <li>default: Boolean true or false. If not specified, false.</li>
    /// <li>children: <code>Map&lt;String, Boolean&gt;</code> of child permissions. If not specified, empty list.</li>
    /// <li>description: Short string containing a very small description of this description. If not specified, empty string.</li>
    /// </ul>
    /// Loads a Permission from a map of data, usually used from retrieval from a yaml file.
    /// <p>The data may contain the following keys:</p>
    /// <ul>
    /// <li>default: Boolean true or false. If not specified, false.</li>
    /// <li>children: <code>Map&lt;String, Boolean&gt;</code> of child permissions. If not specified, empty list.</li>
    /// <li>description: Short string containing a very small description of this description. If not specified, empty string.</li>
    /// </ul>
    pub fn load_permission_with_string(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<impl Into<&'mc blackboxmc_java::JavaMap<'mc>>>,
        arg2: std::option::Option<impl Into<&'mc crate::permissions::PermissionDefault<'mc>>>,
        arg3: std::option::Option<Vec<impl Into<crate::permissions::Permission<'mc>>>>,
    ) -> Result<crate::permissions::Permission<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let val_2 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let val_3 =
            unsafe { jni::objects::JObject::from_raw(arg2.unwrap().into().jni_object().clone()) };
        let raw_val_4 = jni.new_object("java/util/ArrayList", "()V", &[]).unwrap();
        for v in arg3.unwrap() {
            let map_val_0 =
                unsafe { jni::objects::JObject::from_raw(v.into().jni_object().clone()) };
            jni.call_method(
                &raw_val_4,
                "add",
                "(Ljava/Lang/Object)V",
                &[jni::objects::JValueGen::from(&map_val_0)],
            )?;
        }
        let val_4 = jni::objects::JValueGen::Object(raw_val_4);
        let cls = &jni.find_class("org/bukkit/permissions/Permission")?;
        let res = jni.call_static_method(cls,"loadPermission",
"(Ljava/lang/String;Ljava/util/Map;Lorg/bukkit/permissions/PermissionDefault;Ljava/util/List;)Lorg/bukkit/permissions/Permission;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
        let obj = res.l()?;
        crate::permissions::Permission::from_raw(&jni, obj)
    }
    /// Returns the unique fully qualified name of this Permission
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the default value of this permission.
    pub fn default(
        &mut self,
    ) -> Result<crate::permissions::PermissionDefault<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefault",
            "()Lorg/bukkit/permissions/PermissionDefault;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::permissions::PermissionDefault::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::permissions::PermissionDefault::from_string(variant_str).unwrap(),
        )
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
/// Holds information on a permission and which <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> provides it
pub struct PermissionAttachmentInfo<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PermissionAttachmentInfo<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PermissionAttachmentInfo<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::permissions::Permissible<'mc>>,
        arg1: impl Into<&'mc String>,
        arg2: impl Into<&'mc crate::permissions::PermissionAttachment<'mc>>,
        arg3: bool,
    ) -> Result<crate::permissions::PermissionAttachmentInfo<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        // -2
        let val_4 = jni::objects::JValueGen::Bool(arg3.into());
        let cls = &jni.find_class("org/bukkit/permissions/PermissionAttachmentInfo")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/permissions/Permissible;Ljava/lang/String;Lorg/bukkit/permissions/PermissionAttachment;Z)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
        crate::permissions::PermissionAttachmentInfo::from_raw(&jni, res)
    }
    /// Gets the permission being set
    pub fn permission(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermission",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the attachment providing this permission. This may be null for default permissions (usually parent permissions).
    pub fn attachment(
        &mut self,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAttachment",
            "()Lorg/bukkit/permissions/PermissionAttachment;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionAttachment::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the permissible this is attached to
    pub fn permissible(
        &mut self,
    ) -> Result<crate::permissions::Permissible<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermissible",
            "()Lorg/bukkit/permissions/Permissible;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::Permissible::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the value of this permission
    pub fn value(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
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
/// Holds information about a permission attachment on a <a href="Permissible.html" title="interface in org.bukkit.permissions"><code>Permissible</code></a> object
pub struct PermissionAttachment<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PermissionAttachment<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PermissionAttachment<'mc> {
    pub fn from_raw(
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
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
        arg1: impl Into<&'mc crate::permissions::Permissible<'mc>>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/permissions/PermissionAttachment")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/plugin/Plugin;Lorg/bukkit/permissions/Permissible;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        crate::permissions::PermissionAttachment::from_raw(&jni, res)
    }
    /// Sets a permission to the given value, by its fully qualified name
    /// Sets a permission to the given value
    pub fn set_permission_with_permission(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        // 1
        let val_2 = jni::objects::JValueGen::Bool(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setPermission",
            "(Ljava/lang/String;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the plugin responsible for this attachment
    pub fn plugin(&mut self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlugin",
            "()Lorg/bukkit/plugin/Plugin;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the class that was previously set to be called when this attachment was removed from a <a href="Permissible.html" title="interface in org.bukkit.permissions"><code>Permissible</code></a>. May be null.
    pub fn removal_callback(
        &mut self,
    ) -> Result<crate::permissions::PermissionRemovedExecutor<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRemovalCallback",
            "()Lorg/bukkit/permissions/PermissionRemovedExecutor;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionRemovedExecutor::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets an object to be called for when this attachment is removed from a <a href="Permissible.html" title="interface in org.bukkit.permissions"><code>Permissible</code></a>. May be null.
    pub fn set_removal_callback(
        &mut self,
        arg0: impl Into<&'mc crate::permissions::PermissionRemovedExecutor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRemovalCallback",
            "(Lorg/bukkit/permissions/PermissionRemovedExecutor;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the Permissible that this is attached to
    pub fn permissible(
        &mut self,
    ) -> Result<crate::permissions::Permissible<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermissible",
            "()Lorg/bukkit/permissions/Permissible;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::Permissible::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Removes the specified permission from this attachment.
    /// <p>If the permission does not exist in this attachment, nothing will happen.</p>
    /// Removes the specified permission from this attachment.
    /// <p>If the permission does not exist in this attachment, nothing will happen.</p>
    pub fn unset_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "unsetPermission",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Removes this attachment from its registered <a title="interface in org.bukkit.permissions" href="Permissible.html"><code>Permissible</code></a>
    pub fn remove(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets a copy of all set permissions and values contained within this attachment.
    /// <p>This map may be modified but will not affect the attachment, as it is a copy.</p>
    pub fn permissions(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermissions",
            "()Ljava/util/Map;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
/// Represents a class which is to be notified when a <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> is removed from a <a title="interface in org.bukkit.permissions" href="Permissible.html"><code>Permissible</code></a>
///
/// This is a representation of an abstract class.
pub struct PermissionRemovedExecutor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PermissionRemovedExecutor<'mc> {
    pub fn from_raw(
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
    /// Called when a <a href="PermissionAttachment.html" title="class in org.bukkit.permissions"><code>PermissionAttachment</code></a> is removed from a <a title="interface in org.bukkit.permissions" href="Permissible.html"><code>Permissible</code></a>
    pub fn attachment_removed(
        &mut self,
        arg0: impl Into<&'mc crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "attachmentRemoved",
            "(Lorg/bukkit/permissions/PermissionAttachment;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for PermissionRemovedExecutor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
