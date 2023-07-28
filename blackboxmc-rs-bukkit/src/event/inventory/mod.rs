#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub struct InventoryOpenEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryOpenEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryOpenEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryOpenEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "InventoryOpenEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryOpenEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryOpenEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryOpenEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct PrepareItemCraftEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareItemCraftEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareItemCraftEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PrepareItemCraftEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "PrepareItemCraftEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PrepareItemCraftEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareItemCraftEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct PrepareAnvilEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareAnvilEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareAnvilEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PrepareAnvilEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "PrepareAnvilEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PrepareAnvilEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>>
    for PrepareAnvilEvent<'mc>
{
    fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
        crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub enum DragTypeEnum {
    Single,
    Even,
}
impl std::fmt::Display for DragTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DragTypeEnum::Single => f.write_str("SINGLE"),
            DragTypeEnum::Even => f.write_str("EVEN"),
        }
    }
}
pub struct DragType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub DragTypeEnum,
);
impl<'mc> std::ops::Deref for DragType<'mc> {
    type Target = DragTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for DragType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> DragType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: DragTypeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate DragType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "DragType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a DragType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const SINGLE: DragTypeEnum = DragTypeEnum::Single;
    pub const EVEN: DragTypeEnum = DragTypeEnum::Even;
    pub fn from_string(str: String) -> std::option::Option<DragTypeEnum> {
        match str.as_str() {
            "SINGLE" => Some(DragTypeEnum::Single),
            "EVEN" => Some(DragTypeEnum::Even),
            _ => None,
        }
    }
}
pub struct BrewEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BrewEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BrewEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate BrewEvent from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "BrewEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BrewEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BrewEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BrewEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct InventoryMoveItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryMoveItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryMoveItemEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryMoveItemEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "InventoryMoveItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryMoveItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryMoveItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryMoveItemEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct FurnaceSmeltEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceSmeltEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceSmeltEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FurnaceSmeltEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "FurnaceSmeltEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceSmeltEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::block::BlockCookEvent<'mc>> for FurnaceSmeltEvent<'mc> {
    fn into(self) -> crate::event::block::BlockCookEvent<'mc> {
        crate::event::block::BlockCookEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct SmithItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for SmithItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> SmithItemEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate SmithItemEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "SmithItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a SmithItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for SmithItemEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
        crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum ClickTypeEnum {
    Left,
    ShiftLeft,
    Right,
    ShiftRight,
    WindowBorderLeft,
    WindowBorderRight,
    Middle,
    NumberKey,
    DoubleClick,
    Drop,
    ControlDrop,
    Creative,
    SwapOffhand,
    Unknown,
}
impl std::fmt::Display for ClickTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ClickTypeEnum::Left => f.write_str("LEFT"),
            ClickTypeEnum::ShiftLeft => f.write_str("SHIFT_LEFT"),
            ClickTypeEnum::Right => f.write_str("RIGHT"),
            ClickTypeEnum::ShiftRight => f.write_str("SHIFT_RIGHT"),
            ClickTypeEnum::WindowBorderLeft => f.write_str("WINDOW_BORDER_LEFT"),
            ClickTypeEnum::WindowBorderRight => f.write_str("WINDOW_BORDER_RIGHT"),
            ClickTypeEnum::Middle => f.write_str("MIDDLE"),
            ClickTypeEnum::NumberKey => f.write_str("NUMBER_KEY"),
            ClickTypeEnum::DoubleClick => f.write_str("DOUBLE_CLICK"),
            ClickTypeEnum::Drop => f.write_str("DROP"),
            ClickTypeEnum::ControlDrop => f.write_str("CONTROL_DROP"),
            ClickTypeEnum::Creative => f.write_str("CREATIVE"),
            ClickTypeEnum::SwapOffhand => f.write_str("SWAP_OFFHAND"),
            ClickTypeEnum::Unknown => f.write_str("UNKNOWN"),
        }
    }
}
pub struct ClickType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub ClickTypeEnum,
);
impl<'mc> std::ops::Deref for ClickType<'mc> {
    type Target = ClickTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for ClickType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ClickType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: ClickTypeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate ClickType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "ClickType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ClickType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const LEFT: ClickTypeEnum = ClickTypeEnum::Left;
    pub const SHIFT_LEFT: ClickTypeEnum = ClickTypeEnum::ShiftLeft;
    pub const RIGHT: ClickTypeEnum = ClickTypeEnum::Right;
    pub const SHIFT_RIGHT: ClickTypeEnum = ClickTypeEnum::ShiftRight;
    pub const WINDOW_BORDER_LEFT: ClickTypeEnum = ClickTypeEnum::WindowBorderLeft;
    pub const WINDOW_BORDER_RIGHT: ClickTypeEnum = ClickTypeEnum::WindowBorderRight;
    pub const MIDDLE: ClickTypeEnum = ClickTypeEnum::Middle;
    pub const NUMBER_KEY: ClickTypeEnum = ClickTypeEnum::NumberKey;
    pub const DOUBLE_CLICK: ClickTypeEnum = ClickTypeEnum::DoubleClick;
    pub const DROP: ClickTypeEnum = ClickTypeEnum::Drop;
    pub const CONTROL_DROP: ClickTypeEnum = ClickTypeEnum::ControlDrop;
    pub const CREATIVE: ClickTypeEnum = ClickTypeEnum::Creative;
    pub const SWAP_OFFHAND: ClickTypeEnum = ClickTypeEnum::SwapOffhand;
    pub const UNKNOWN: ClickTypeEnum = ClickTypeEnum::Unknown;
    pub fn from_string(str: String) -> std::option::Option<ClickTypeEnum> {
        match str.as_str() {
            "LEFT" => Some(ClickTypeEnum::Left),
            "SHIFT_LEFT" => Some(ClickTypeEnum::ShiftLeft),
            "RIGHT" => Some(ClickTypeEnum::Right),
            "SHIFT_RIGHT" => Some(ClickTypeEnum::ShiftRight),
            "WINDOW_BORDER_LEFT" => Some(ClickTypeEnum::WindowBorderLeft),
            "WINDOW_BORDER_RIGHT" => Some(ClickTypeEnum::WindowBorderRight),
            "MIDDLE" => Some(ClickTypeEnum::Middle),
            "NUMBER_KEY" => Some(ClickTypeEnum::NumberKey),
            "DOUBLE_CLICK" => Some(ClickTypeEnum::DoubleClick),
            "DROP" => Some(ClickTypeEnum::Drop),
            "CONTROL_DROP" => Some(ClickTypeEnum::ControlDrop),
            "CREATIVE" => Some(ClickTypeEnum::Creative),
            "SWAP_OFFHAND" => Some(ClickTypeEnum::SwapOffhand),
            "UNKNOWN" => Some(ClickTypeEnum::Unknown),
            _ => None,
        }
    }
}
pub struct BrewingStandFuelEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for BrewingStandFuelEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> BrewingStandFuelEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate BrewingStandFuelEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "BrewingStandFuelEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a BrewingStandFuelEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for BrewingStandFuelEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for BrewingStandFuelEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct PrepareGrindstoneEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareGrindstoneEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareGrindstoneEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PrepareGrindstoneEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "PrepareGrindstoneEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PrepareGrindstoneEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>>
    for PrepareGrindstoneEvent<'mc>
{
    fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
        crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub struct HopperInventorySearchEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct HopperInventorySearchEventContainerType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for HopperInventorySearchEventContainerType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HopperInventorySearchEventContainerType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate HopperInventorySearchEventContainerType from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "HopperInventorySearchEventContainerType")?;
        if !valid {
            Err(eyre::eyre!(
        "Invalid argument passed. Expected a HopperInventorySearchEventContainerType object, got {}",
        name
    )
    .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> blackboxmc_general::JNIRaw<'mc> for HopperInventorySearchEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> HopperInventorySearchEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate HopperInventorySearchEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "HopperInventorySearchEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a HopperInventorySearchEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for HopperInventorySearchEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct CraftItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for CraftItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> CraftItemEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate CraftItemEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "CraftItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a CraftItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for CraftItemEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
        crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct FurnaceStartSmeltEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceStartSmeltEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceStartSmeltEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate FurnaceStartSmeltEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "FurnaceStartSmeltEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceStartSmeltEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::block::InventoryBlockStartEvent<'mc>> for FurnaceStartSmeltEvent<'mc> {
    fn into(self) -> crate::event::block::InventoryBlockStartEvent<'mc> {
        crate::event::block::InventoryBlockStartEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct InventoryPickupItemEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryPickupItemEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryPickupItemEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryPickupItemEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "InventoryPickupItemEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryPickupItemEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryPickupItemEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryPickupItemEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct InventoryInteractEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryInteractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryInteractEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryInteractEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "InventoryInteractEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryInteractEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for InventoryInteractEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryInteractEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct InventoryEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "InventoryEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Event<'mc>> for InventoryEvent<'mc> {
    fn into(self) -> crate::event::Event<'mc> {
        crate::event::Event::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct InventoryCreativeEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryCreativeEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryCreativeEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryCreativeEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "InventoryCreativeEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryCreativeEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::InventoryClickEvent<'mc>> for InventoryCreativeEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryClickEvent<'mc> {
        crate::event::inventory::InventoryClickEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct FurnaceExtractEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceExtractEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceExtractEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FurnaceExtractEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "FurnaceExtractEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceExtractEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::block::BlockExpEvent<'mc>> for FurnaceExtractEvent<'mc> {
    fn into(self) -> crate::event::block::BlockExpEvent<'mc> {
        crate::event::block::BlockExpEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum InventoryTypeEnum {
    Chest,
    Dispenser,
    Dropper,
    Furnace,
    Workbench,
    Crafting,
    Enchanting,
    Brewing,
    Player,
    Creative,
    Merchant,
    EnderChest,
    Anvil,
    Smithing,
    Beacon,
    Hopper,
    ShulkerBox,
    Barrel,
    BlastFurnace,
    Lectern,
    Smoker,
    Loom,
    Cartography,
    Grindstone,
    Stonecutter,
    Composter,
    ChiseledBookshelf,
    Jukebox,
    #[deprecated]
    SmithingNew,
}
impl std::fmt::Display for InventoryTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InventoryTypeEnum::Chest => f.write_str("CHEST"),
            InventoryTypeEnum::Dispenser => f.write_str("DISPENSER"),
            InventoryTypeEnum::Dropper => f.write_str("DROPPER"),
            InventoryTypeEnum::Furnace => f.write_str("FURNACE"),
            InventoryTypeEnum::Workbench => f.write_str("WORKBENCH"),
            InventoryTypeEnum::Crafting => f.write_str("CRAFTING"),
            InventoryTypeEnum::Enchanting => f.write_str("ENCHANTING"),
            InventoryTypeEnum::Brewing => f.write_str("BREWING"),
            InventoryTypeEnum::Player => f.write_str("PLAYER"),
            InventoryTypeEnum::Creative => f.write_str("CREATIVE"),
            InventoryTypeEnum::Merchant => f.write_str("MERCHANT"),
            InventoryTypeEnum::EnderChest => f.write_str("ENDER_CHEST"),
            InventoryTypeEnum::Anvil => f.write_str("ANVIL"),
            InventoryTypeEnum::Smithing => f.write_str("SMITHING"),
            InventoryTypeEnum::Beacon => f.write_str("BEACON"),
            InventoryTypeEnum::Hopper => f.write_str("HOPPER"),
            InventoryTypeEnum::ShulkerBox => f.write_str("SHULKER_BOX"),
            InventoryTypeEnum::Barrel => f.write_str("BARREL"),
            InventoryTypeEnum::BlastFurnace => f.write_str("BLAST_FURNACE"),
            InventoryTypeEnum::Lectern => f.write_str("LECTERN"),
            InventoryTypeEnum::Smoker => f.write_str("SMOKER"),
            InventoryTypeEnum::Loom => f.write_str("LOOM"),
            InventoryTypeEnum::Cartography => f.write_str("CARTOGRAPHY"),
            InventoryTypeEnum::Grindstone => f.write_str("GRINDSTONE"),
            InventoryTypeEnum::Stonecutter => f.write_str("STONECUTTER"),
            InventoryTypeEnum::Composter => f.write_str("COMPOSTER"),
            InventoryTypeEnum::ChiseledBookshelf => f.write_str("CHISELED_BOOKSHELF"),
            InventoryTypeEnum::Jukebox => f.write_str("JUKEBOX"),
            InventoryTypeEnum::SmithingNew => f.write_str("SMITHING_NEW"),
        }
    }
}
pub struct InventoryType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub InventoryTypeEnum,
);
impl<'mc> std::ops::Deref for InventoryType<'mc> {
    type Target = InventoryTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for InventoryType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub struct InventoryTypeSlotType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryTypeSlotType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryTypeSlotType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate InventoryTypeSlotType from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "InventoryTypeSlotType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryTypeSlotType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> InventoryType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: InventoryTypeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate InventoryType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "InventoryType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const CHEST: InventoryTypeEnum = InventoryTypeEnum::Chest;
    pub const DISPENSER: InventoryTypeEnum = InventoryTypeEnum::Dispenser;
    pub const DROPPER: InventoryTypeEnum = InventoryTypeEnum::Dropper;
    pub const FURNACE: InventoryTypeEnum = InventoryTypeEnum::Furnace;
    pub const WORKBENCH: InventoryTypeEnum = InventoryTypeEnum::Workbench;
    pub const CRAFTING: InventoryTypeEnum = InventoryTypeEnum::Crafting;
    pub const ENCHANTING: InventoryTypeEnum = InventoryTypeEnum::Enchanting;
    pub const BREWING: InventoryTypeEnum = InventoryTypeEnum::Brewing;
    pub const PLAYER: InventoryTypeEnum = InventoryTypeEnum::Player;
    pub const CREATIVE: InventoryTypeEnum = InventoryTypeEnum::Creative;
    pub const MERCHANT: InventoryTypeEnum = InventoryTypeEnum::Merchant;
    pub const ENDER_CHEST: InventoryTypeEnum = InventoryTypeEnum::EnderChest;
    pub const ANVIL: InventoryTypeEnum = InventoryTypeEnum::Anvil;
    pub const SMITHING: InventoryTypeEnum = InventoryTypeEnum::Smithing;
    pub const BEACON: InventoryTypeEnum = InventoryTypeEnum::Beacon;
    pub const HOPPER: InventoryTypeEnum = InventoryTypeEnum::Hopper;
    pub const SHULKER_BOX: InventoryTypeEnum = InventoryTypeEnum::ShulkerBox;
    pub const BARREL: InventoryTypeEnum = InventoryTypeEnum::Barrel;
    pub const BLAST_FURNACE: InventoryTypeEnum = InventoryTypeEnum::BlastFurnace;
    pub const LECTERN: InventoryTypeEnum = InventoryTypeEnum::Lectern;
    pub const SMOKER: InventoryTypeEnum = InventoryTypeEnum::Smoker;
    pub const LOOM: InventoryTypeEnum = InventoryTypeEnum::Loom;
    pub const CARTOGRAPHY: InventoryTypeEnum = InventoryTypeEnum::Cartography;
    pub const GRINDSTONE: InventoryTypeEnum = InventoryTypeEnum::Grindstone;
    pub const STONECUTTER: InventoryTypeEnum = InventoryTypeEnum::Stonecutter;
    pub const COMPOSTER: InventoryTypeEnum = InventoryTypeEnum::Composter;
    pub const CHISELED_BOOKSHELF: InventoryTypeEnum = InventoryTypeEnum::ChiseledBookshelf;
    pub const JUKEBOX: InventoryTypeEnum = InventoryTypeEnum::Jukebox;
    pub const SMITHING_NEW: InventoryTypeEnum = InventoryTypeEnum::SmithingNew;
    pub fn from_string(str: String) -> std::option::Option<InventoryTypeEnum> {
        match str.as_str() {
            "CHEST" => Some(InventoryTypeEnum::Chest),
            "DISPENSER" => Some(InventoryTypeEnum::Dispenser),
            "DROPPER" => Some(InventoryTypeEnum::Dropper),
            "FURNACE" => Some(InventoryTypeEnum::Furnace),
            "WORKBENCH" => Some(InventoryTypeEnum::Workbench),
            "CRAFTING" => Some(InventoryTypeEnum::Crafting),
            "ENCHANTING" => Some(InventoryTypeEnum::Enchanting),
            "BREWING" => Some(InventoryTypeEnum::Brewing),
            "PLAYER" => Some(InventoryTypeEnum::Player),
            "CREATIVE" => Some(InventoryTypeEnum::Creative),
            "MERCHANT" => Some(InventoryTypeEnum::Merchant),
            "ENDER_CHEST" => Some(InventoryTypeEnum::EnderChest),
            "ANVIL" => Some(InventoryTypeEnum::Anvil),
            "SMITHING" => Some(InventoryTypeEnum::Smithing),
            "BEACON" => Some(InventoryTypeEnum::Beacon),
            "HOPPER" => Some(InventoryTypeEnum::Hopper),
            "SHULKER_BOX" => Some(InventoryTypeEnum::ShulkerBox),
            "BARREL" => Some(InventoryTypeEnum::Barrel),
            "BLAST_FURNACE" => Some(InventoryTypeEnum::BlastFurnace),
            "LECTERN" => Some(InventoryTypeEnum::Lectern),
            "SMOKER" => Some(InventoryTypeEnum::Smoker),
            "LOOM" => Some(InventoryTypeEnum::Loom),
            "CARTOGRAPHY" => Some(InventoryTypeEnum::Cartography),
            "GRINDSTONE" => Some(InventoryTypeEnum::Grindstone),
            "STONECUTTER" => Some(InventoryTypeEnum::Stonecutter),
            "COMPOSTER" => Some(InventoryTypeEnum::Composter),
            "CHISELED_BOOKSHELF" => Some(InventoryTypeEnum::ChiseledBookshelf),
            "JUKEBOX" => Some(InventoryTypeEnum::Jukebox),
            "SMITHING_NEW" => Some(InventoryTypeEnum::SmithingNew),
            _ => None,
        }
    }
}
pub struct PrepareSmithingEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareSmithingEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareSmithingEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PrepareSmithingEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "PrepareSmithingEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PrepareSmithingEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::PrepareInventoryResultEvent<'mc>>
    for PrepareSmithingEvent<'mc>
{
    fn into(self) -> crate::event::inventory::PrepareInventoryResultEvent<'mc> {
        crate::event::inventory::PrepareInventoryResultEvent::from_raw(&self.jni_ref(), self.1)
            .unwrap()
    }
}
pub struct TradeSelectEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TradeSelectEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TradeSelectEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate TradeSelectEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "TradeSelectEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TradeSelectEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for TradeSelectEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
        crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub enum InventoryActionEnum {
    Nothing,
    PickupAll,
    PickupSome,
    PickupHalf,
    PickupOne,
    PlaceAll,
    PlaceSome,
    PlaceOne,
    SwapWithCursor,
    DropAllCursor,
    DropOneCursor,
    DropAllSlot,
    DropOneSlot,
    MoveToOtherInventory,
    HotbarMoveAndReadd,
    HotbarSwap,
    CloneStack,
    CollectToCursor,
    Unknown,
}
impl std::fmt::Display for InventoryActionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InventoryActionEnum::Nothing => f.write_str("NOTHING"),
            InventoryActionEnum::PickupAll => f.write_str("PICKUP_ALL"),
            InventoryActionEnum::PickupSome => f.write_str("PICKUP_SOME"),
            InventoryActionEnum::PickupHalf => f.write_str("PICKUP_HALF"),
            InventoryActionEnum::PickupOne => f.write_str("PICKUP_ONE"),
            InventoryActionEnum::PlaceAll => f.write_str("PLACE_ALL"),
            InventoryActionEnum::PlaceSome => f.write_str("PLACE_SOME"),
            InventoryActionEnum::PlaceOne => f.write_str("PLACE_ONE"),
            InventoryActionEnum::SwapWithCursor => f.write_str("SWAP_WITH_CURSOR"),
            InventoryActionEnum::DropAllCursor => f.write_str("DROP_ALL_CURSOR"),
            InventoryActionEnum::DropOneCursor => f.write_str("DROP_ONE_CURSOR"),
            InventoryActionEnum::DropAllSlot => f.write_str("DROP_ALL_SLOT"),
            InventoryActionEnum::DropOneSlot => f.write_str("DROP_ONE_SLOT"),
            InventoryActionEnum::MoveToOtherInventory => f.write_str("MOVE_TO_OTHER_INVENTORY"),
            InventoryActionEnum::HotbarMoveAndReadd => f.write_str("HOTBAR_MOVE_AND_READD"),
            InventoryActionEnum::HotbarSwap => f.write_str("HOTBAR_SWAP"),
            InventoryActionEnum::CloneStack => f.write_str("CLONE_STACK"),
            InventoryActionEnum::CollectToCursor => f.write_str("COLLECT_TO_CURSOR"),
            InventoryActionEnum::Unknown => f.write_str("UNKNOWN"),
        }
    }
}
pub struct InventoryAction<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub InventoryActionEnum,
);
impl<'mc> std::ops::Deref for InventoryAction<'mc> {
    type Target = InventoryActionEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for InventoryAction<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryAction<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: InventoryActionEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryAction from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "InventoryAction")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryAction object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const NOTHING: InventoryActionEnum = InventoryActionEnum::Nothing;
    pub const PICKUP_ALL: InventoryActionEnum = InventoryActionEnum::PickupAll;
    pub const PICKUP_SOME: InventoryActionEnum = InventoryActionEnum::PickupSome;
    pub const PICKUP_HALF: InventoryActionEnum = InventoryActionEnum::PickupHalf;
    pub const PICKUP_ONE: InventoryActionEnum = InventoryActionEnum::PickupOne;
    pub const PLACE_ALL: InventoryActionEnum = InventoryActionEnum::PlaceAll;
    pub const PLACE_SOME: InventoryActionEnum = InventoryActionEnum::PlaceSome;
    pub const PLACE_ONE: InventoryActionEnum = InventoryActionEnum::PlaceOne;
    pub const SWAP_WITH_CURSOR: InventoryActionEnum = InventoryActionEnum::SwapWithCursor;
    pub const DROP_ALL_CURSOR: InventoryActionEnum = InventoryActionEnum::DropAllCursor;
    pub const DROP_ONE_CURSOR: InventoryActionEnum = InventoryActionEnum::DropOneCursor;
    pub const DROP_ALL_SLOT: InventoryActionEnum = InventoryActionEnum::DropAllSlot;
    pub const DROP_ONE_SLOT: InventoryActionEnum = InventoryActionEnum::DropOneSlot;
    pub const MOVE_TO_OTHER_INVENTORY: InventoryActionEnum =
        InventoryActionEnum::MoveToOtherInventory;
    pub const HOTBAR_MOVE_AND_READD: InventoryActionEnum = InventoryActionEnum::HotbarMoveAndReadd;
    pub const HOTBAR_SWAP: InventoryActionEnum = InventoryActionEnum::HotbarSwap;
    pub const CLONE_STACK: InventoryActionEnum = InventoryActionEnum::CloneStack;
    pub const COLLECT_TO_CURSOR: InventoryActionEnum = InventoryActionEnum::CollectToCursor;
    pub const UNKNOWN: InventoryActionEnum = InventoryActionEnum::Unknown;
    pub fn from_string(str: String) -> std::option::Option<InventoryActionEnum> {
        match str.as_str() {
            "NOTHING" => Some(InventoryActionEnum::Nothing),
            "PICKUP_ALL" => Some(InventoryActionEnum::PickupAll),
            "PICKUP_SOME" => Some(InventoryActionEnum::PickupSome),
            "PICKUP_HALF" => Some(InventoryActionEnum::PickupHalf),
            "PICKUP_ONE" => Some(InventoryActionEnum::PickupOne),
            "PLACE_ALL" => Some(InventoryActionEnum::PlaceAll),
            "PLACE_SOME" => Some(InventoryActionEnum::PlaceSome),
            "PLACE_ONE" => Some(InventoryActionEnum::PlaceOne),
            "SWAP_WITH_CURSOR" => Some(InventoryActionEnum::SwapWithCursor),
            "DROP_ALL_CURSOR" => Some(InventoryActionEnum::DropAllCursor),
            "DROP_ONE_CURSOR" => Some(InventoryActionEnum::DropOneCursor),
            "DROP_ALL_SLOT" => Some(InventoryActionEnum::DropAllSlot),
            "DROP_ONE_SLOT" => Some(InventoryActionEnum::DropOneSlot),
            "MOVE_TO_OTHER_INVENTORY" => Some(InventoryActionEnum::MoveToOtherInventory),
            "HOTBAR_MOVE_AND_READD" => Some(InventoryActionEnum::HotbarMoveAndReadd),
            "HOTBAR_SWAP" => Some(InventoryActionEnum::HotbarSwap),
            "CLONE_STACK" => Some(InventoryActionEnum::CloneStack),
            "COLLECT_TO_CURSOR" => Some(InventoryActionEnum::CollectToCursor),
            "UNKNOWN" => Some(InventoryActionEnum::Unknown),
            _ => None,
        }
    }
}
pub struct InventoryClickEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryClickEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryClickEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryClickEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "InventoryClickEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryClickEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for InventoryClickEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
        crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct InventoryDragEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryDragEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryDragEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryDragEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "InventoryDragEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryDragEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::InventoryInteractEvent<'mc>> for InventoryDragEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryInteractEvent<'mc> {
        crate::event::inventory::InventoryInteractEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct FurnaceBurnEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for FurnaceBurnEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> FurnaceBurnEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FurnaceBurnEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "FurnaceBurnEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FurnaceBurnEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::Cancellable<'mc>> for FurnaceBurnEvent<'mc> {
    fn into(self) -> crate::event::Cancellable<'mc> {
        crate::event::Cancellable::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
impl<'mc> Into<crate::event::block::BlockEvent<'mc>> for FurnaceBurnEvent<'mc> {
    fn into(self) -> crate::event::block::BlockEvent<'mc> {
        crate::event::block::BlockEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct InventoryCloseEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for InventoryCloseEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> InventoryCloseEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate InventoryCloseEvent from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "InventoryCloseEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a InventoryCloseEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for InventoryCloseEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub struct PrepareInventoryResultEvent<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PrepareInventoryResultEvent<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PrepareInventoryResultEvent<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PrepareInventoryResultEvent from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "PrepareInventoryResultEvent")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PrepareInventoryResultEvent object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> Into<crate::event::inventory::InventoryEvent<'mc>> for PrepareInventoryResultEvent<'mc> {
    fn into(self) -> crate::event::inventory::InventoryEvent<'mc> {
        crate::event::inventory::InventoryEvent::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
