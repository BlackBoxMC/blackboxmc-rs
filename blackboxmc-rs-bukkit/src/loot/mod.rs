#![allow(deprecated)]
#![feature(anonymous_lifetime_in_impl_trait)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// An instantiatable struct that implements LootTable. Needed for returning it from Java.
pub struct LootTable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LootTable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LootTable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "LootTable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootTable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn key(&mut self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKey",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for LootTable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements Lootable. Needed for returning it from Java.
pub struct Lootable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Lootable<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Lootable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Lootable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Lootable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<&'mc crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for Lootable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
pub enum LootTablesEnum {
    Empty,
    AbandonedMineshaft,
    BuriedTreasure,
    DesertPyramid,
    EndCityTreasure,
    IglooChest,
    JungleTemple,
    JungleTempleDispenser,
    NetherBridge,
    PillagerOutpost,
    BastionTreasure,
    BastionOther,
    BastionBridge,
    BastionHoglinStable,
    AncientCity,
    AncientCityIceBox,
    RuinedPortal,
    ShipwreckMap,
    ShipwreckSupply,
    ShipwreckTreasure,
    SimpleDungeon,
    SpawnBonusChest,
    StrongholdCorridor,
    StrongholdCrossing,
    StrongholdLibrary,
    UnderwaterRuinBig,
    UnderwaterRuinSmall,
    VillageArmorer,
    VillageButcher,
    VillageCartographer,
    VillageDesertHouse,
    VillageFisher,
    VillageFletcher,
    VillageMason,
    VillagePlainsHouse,
    VillageSavannaHouse,
    VillageShepherd,
    VillageSnowyHouse,
    VillageTaigaHouse,
    VillageTannery,
    VillageTemple,
    VillageToolsmith,
    VillageWeaponsmith,
    WoodlandMansion,
    ArmorStand,
    Axolotl,
    Bat,
    Bee,
    Blaze,
    Cat,
    CaveSpider,
    Chicken,
    Cod,
    Cow,
    Creeper,
    Dolphin,
    Donkey,
    Drowned,
    ElderGuardian,
    EnderDragon,
    Enderman,
    Endermite,
    Evoker,
    Fox,
    Ghast,
    Giant,
    GlowSquid,
    Goat,
    Guardian,
    Hoglin,
    Horse,
    Husk,
    Illusioner,
    IronGolem,
    Llama,
    MagmaCube,
    Mooshroom,
    Mule,
    Ocelot,
    Panda,
    Parrot,
    Phantom,
    Pig,
    Piglin,
    PiglinBrute,
    Pillager,
    Player,
    PolarBear,
    Pufferfish,
    Rabbit,
    Ravager,
    Salmon,
    Shulker,
    Silverfish,
    Skeleton,
    SkeletonHorse,
    Slime,
    SnowGolem,
    Spider,
    Squid,
    Stray,
    Strider,
    TraderLlama,
    TropicalFish,
    Turtle,
    Vex,
    Villager,
    Vindicator,
    WanderingTrader,
    Witch,
    Wither,
    WitherSkeleton,
    Wolf,
    Zoglin,
    Zombie,
    ZombieHorse,
    ZombieVillager,
    ZombifiedPiglin,
    ArmorerGift,
    ButcherGift,
    CartographerGift,
    CatMorningGift,
    ClericGift,
    FarmerGift,
    FishermanGift,
    Fishing,
    FishingFish,
    FishingJunk,
    FishingTreasure,
    FletcherGift,
    LeatherworkerGift,
    LibrarianGift,
    MasonGift,
    ShepherdGift,
    ToolsmithGift,
    WeaponsmithGift,
    SnifferDigging,
    PiglinBartering,
    DesertWellArchaeology,
    DesertPyramidArchaeology,
    TrailRuinsArchaeologyCommon,
    TrailRuinsArchaeologyRare,
    OceanRuinWarmArchaeology,
    OceanRuinColdArchaeology,
    Sheep,
    SheepBlack,
    SheepBlue,
    SheepBrown,
    SheepCyan,
    SheepGray,
    SheepGreen,
    SheepLightBlue,
    SheepLightGray,
    SheepLime,
    SheepMagenta,
    SheepOrange,
    SheepPink,
    SheepPurple,
    SheepRed,
    SheepWhite,
    SheepYellow,
}
impl std::fmt::Display for LootTablesEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LootTablesEnum::Empty => f.write_str("EMPTY"),
            LootTablesEnum::AbandonedMineshaft => f.write_str("ABANDONED_MINESHAFT"),
            LootTablesEnum::BuriedTreasure => f.write_str("BURIED_TREASURE"),
            LootTablesEnum::DesertPyramid => f.write_str("DESERT_PYRAMID"),
            LootTablesEnum::EndCityTreasure => f.write_str("END_CITY_TREASURE"),
            LootTablesEnum::IglooChest => f.write_str("IGLOO_CHEST"),
            LootTablesEnum::JungleTemple => f.write_str("JUNGLE_TEMPLE"),
            LootTablesEnum::JungleTempleDispenser => f.write_str("JUNGLE_TEMPLE_DISPENSER"),
            LootTablesEnum::NetherBridge => f.write_str("NETHER_BRIDGE"),
            LootTablesEnum::PillagerOutpost => f.write_str("PILLAGER_OUTPOST"),
            LootTablesEnum::BastionTreasure => f.write_str("BASTION_TREASURE"),
            LootTablesEnum::BastionOther => f.write_str("BASTION_OTHER"),
            LootTablesEnum::BastionBridge => f.write_str("BASTION_BRIDGE"),
            LootTablesEnum::BastionHoglinStable => f.write_str("BASTION_HOGLIN_STABLE"),
            LootTablesEnum::AncientCity => f.write_str("ANCIENT_CITY"),
            LootTablesEnum::AncientCityIceBox => f.write_str("ANCIENT_CITY_ICE_BOX"),
            LootTablesEnum::RuinedPortal => f.write_str("RUINED_PORTAL"),
            LootTablesEnum::ShipwreckMap => f.write_str("SHIPWRECK_MAP"),
            LootTablesEnum::ShipwreckSupply => f.write_str("SHIPWRECK_SUPPLY"),
            LootTablesEnum::ShipwreckTreasure => f.write_str("SHIPWRECK_TREASURE"),
            LootTablesEnum::SimpleDungeon => f.write_str("SIMPLE_DUNGEON"),
            LootTablesEnum::SpawnBonusChest => f.write_str("SPAWN_BONUS_CHEST"),
            LootTablesEnum::StrongholdCorridor => f.write_str("STRONGHOLD_CORRIDOR"),
            LootTablesEnum::StrongholdCrossing => f.write_str("STRONGHOLD_CROSSING"),
            LootTablesEnum::StrongholdLibrary => f.write_str("STRONGHOLD_LIBRARY"),
            LootTablesEnum::UnderwaterRuinBig => f.write_str("UNDERWATER_RUIN_BIG"),
            LootTablesEnum::UnderwaterRuinSmall => f.write_str("UNDERWATER_RUIN_SMALL"),
            LootTablesEnum::VillageArmorer => f.write_str("VILLAGE_ARMORER"),
            LootTablesEnum::VillageButcher => f.write_str("VILLAGE_BUTCHER"),
            LootTablesEnum::VillageCartographer => f.write_str("VILLAGE_CARTOGRAPHER"),
            LootTablesEnum::VillageDesertHouse => f.write_str("VILLAGE_DESERT_HOUSE"),
            LootTablesEnum::VillageFisher => f.write_str("VILLAGE_FISHER"),
            LootTablesEnum::VillageFletcher => f.write_str("VILLAGE_FLETCHER"),
            LootTablesEnum::VillageMason => f.write_str("VILLAGE_MASON"),
            LootTablesEnum::VillagePlainsHouse => f.write_str("VILLAGE_PLAINS_HOUSE"),
            LootTablesEnum::VillageSavannaHouse => f.write_str("VILLAGE_SAVANNA_HOUSE"),
            LootTablesEnum::VillageShepherd => f.write_str("VILLAGE_SHEPHERD"),
            LootTablesEnum::VillageSnowyHouse => f.write_str("VILLAGE_SNOWY_HOUSE"),
            LootTablesEnum::VillageTaigaHouse => f.write_str("VILLAGE_TAIGA_HOUSE"),
            LootTablesEnum::VillageTannery => f.write_str("VILLAGE_TANNERY"),
            LootTablesEnum::VillageTemple => f.write_str("VILLAGE_TEMPLE"),
            LootTablesEnum::VillageToolsmith => f.write_str("VILLAGE_TOOLSMITH"),
            LootTablesEnum::VillageWeaponsmith => f.write_str("VILLAGE_WEAPONSMITH"),
            LootTablesEnum::WoodlandMansion => f.write_str("WOODLAND_MANSION"),
            LootTablesEnum::ArmorStand => f.write_str("ARMOR_STAND"),
            LootTablesEnum::Axolotl => f.write_str("AXOLOTL"),
            LootTablesEnum::Bat => f.write_str("BAT"),
            LootTablesEnum::Bee => f.write_str("BEE"),
            LootTablesEnum::Blaze => f.write_str("BLAZE"),
            LootTablesEnum::Cat => f.write_str("CAT"),
            LootTablesEnum::CaveSpider => f.write_str("CAVE_SPIDER"),
            LootTablesEnum::Chicken => f.write_str("CHICKEN"),
            LootTablesEnum::Cod => f.write_str("COD"),
            LootTablesEnum::Cow => f.write_str("COW"),
            LootTablesEnum::Creeper => f.write_str("CREEPER"),
            LootTablesEnum::Dolphin => f.write_str("DOLPHIN"),
            LootTablesEnum::Donkey => f.write_str("DONKEY"),
            LootTablesEnum::Drowned => f.write_str("DROWNED"),
            LootTablesEnum::ElderGuardian => f.write_str("ELDER_GUARDIAN"),
            LootTablesEnum::EnderDragon => f.write_str("ENDER_DRAGON"),
            LootTablesEnum::Enderman => f.write_str("ENDERMAN"),
            LootTablesEnum::Endermite => f.write_str("ENDERMITE"),
            LootTablesEnum::Evoker => f.write_str("EVOKER"),
            LootTablesEnum::Fox => f.write_str("FOX"),
            LootTablesEnum::Ghast => f.write_str("GHAST"),
            LootTablesEnum::Giant => f.write_str("GIANT"),
            LootTablesEnum::GlowSquid => f.write_str("GLOW_SQUID"),
            LootTablesEnum::Goat => f.write_str("GOAT"),
            LootTablesEnum::Guardian => f.write_str("GUARDIAN"),
            LootTablesEnum::Hoglin => f.write_str("HOGLIN"),
            LootTablesEnum::Horse => f.write_str("HORSE"),
            LootTablesEnum::Husk => f.write_str("HUSK"),
            LootTablesEnum::Illusioner => f.write_str("ILLUSIONER"),
            LootTablesEnum::IronGolem => f.write_str("IRON_GOLEM"),
            LootTablesEnum::Llama => f.write_str("LLAMA"),
            LootTablesEnum::MagmaCube => f.write_str("MAGMA_CUBE"),
            LootTablesEnum::Mooshroom => f.write_str("MOOSHROOM"),
            LootTablesEnum::Mule => f.write_str("MULE"),
            LootTablesEnum::Ocelot => f.write_str("OCELOT"),
            LootTablesEnum::Panda => f.write_str("PANDA"),
            LootTablesEnum::Parrot => f.write_str("PARROT"),
            LootTablesEnum::Phantom => f.write_str("PHANTOM"),
            LootTablesEnum::Pig => f.write_str("PIG"),
            LootTablesEnum::Piglin => f.write_str("PIGLIN"),
            LootTablesEnum::PiglinBrute => f.write_str("PIGLIN_BRUTE"),
            LootTablesEnum::Pillager => f.write_str("PILLAGER"),
            LootTablesEnum::Player => f.write_str("PLAYER"),
            LootTablesEnum::PolarBear => f.write_str("POLAR_BEAR"),
            LootTablesEnum::Pufferfish => f.write_str("PUFFERFISH"),
            LootTablesEnum::Rabbit => f.write_str("RABBIT"),
            LootTablesEnum::Ravager => f.write_str("RAVAGER"),
            LootTablesEnum::Salmon => f.write_str("SALMON"),
            LootTablesEnum::Shulker => f.write_str("SHULKER"),
            LootTablesEnum::Silverfish => f.write_str("SILVERFISH"),
            LootTablesEnum::Skeleton => f.write_str("SKELETON"),
            LootTablesEnum::SkeletonHorse => f.write_str("SKELETON_HORSE"),
            LootTablesEnum::Slime => f.write_str("SLIME"),
            LootTablesEnum::SnowGolem => f.write_str("SNOW_GOLEM"),
            LootTablesEnum::Spider => f.write_str("SPIDER"),
            LootTablesEnum::Squid => f.write_str("SQUID"),
            LootTablesEnum::Stray => f.write_str("STRAY"),
            LootTablesEnum::Strider => f.write_str("STRIDER"),
            LootTablesEnum::TraderLlama => f.write_str("TRADER_LLAMA"),
            LootTablesEnum::TropicalFish => f.write_str("TROPICAL_FISH"),
            LootTablesEnum::Turtle => f.write_str("TURTLE"),
            LootTablesEnum::Vex => f.write_str("VEX"),
            LootTablesEnum::Villager => f.write_str("VILLAGER"),
            LootTablesEnum::Vindicator => f.write_str("VINDICATOR"),
            LootTablesEnum::WanderingTrader => f.write_str("WANDERING_TRADER"),
            LootTablesEnum::Witch => f.write_str("WITCH"),
            LootTablesEnum::Wither => f.write_str("WITHER"),
            LootTablesEnum::WitherSkeleton => f.write_str("WITHER_SKELETON"),
            LootTablesEnum::Wolf => f.write_str("WOLF"),
            LootTablesEnum::Zoglin => f.write_str("ZOGLIN"),
            LootTablesEnum::Zombie => f.write_str("ZOMBIE"),
            LootTablesEnum::ZombieHorse => f.write_str("ZOMBIE_HORSE"),
            LootTablesEnum::ZombieVillager => f.write_str("ZOMBIE_VILLAGER"),
            LootTablesEnum::ZombifiedPiglin => f.write_str("ZOMBIFIED_PIGLIN"),
            LootTablesEnum::ArmorerGift => f.write_str("ARMORER_GIFT"),
            LootTablesEnum::ButcherGift => f.write_str("BUTCHER_GIFT"),
            LootTablesEnum::CartographerGift => f.write_str("CARTOGRAPHER_GIFT"),
            LootTablesEnum::CatMorningGift => f.write_str("CAT_MORNING_GIFT"),
            LootTablesEnum::ClericGift => f.write_str("CLERIC_GIFT"),
            LootTablesEnum::FarmerGift => f.write_str("FARMER_GIFT"),
            LootTablesEnum::FishermanGift => f.write_str("FISHERMAN_GIFT"),
            LootTablesEnum::Fishing => f.write_str("FISHING"),
            LootTablesEnum::FishingFish => f.write_str("FISHING_FISH"),
            LootTablesEnum::FishingJunk => f.write_str("FISHING_JUNK"),
            LootTablesEnum::FishingTreasure => f.write_str("FISHING_TREASURE"),
            LootTablesEnum::FletcherGift => f.write_str("FLETCHER_GIFT"),
            LootTablesEnum::LeatherworkerGift => f.write_str("LEATHERWORKER_GIFT"),
            LootTablesEnum::LibrarianGift => f.write_str("LIBRARIAN_GIFT"),
            LootTablesEnum::MasonGift => f.write_str("MASON_GIFT"),
            LootTablesEnum::ShepherdGift => f.write_str("SHEPHERD_GIFT"),
            LootTablesEnum::ToolsmithGift => f.write_str("TOOLSMITH_GIFT"),
            LootTablesEnum::WeaponsmithGift => f.write_str("WEAPONSMITH_GIFT"),
            LootTablesEnum::SnifferDigging => f.write_str("SNIFFER_DIGGING"),
            LootTablesEnum::PiglinBartering => f.write_str("PIGLIN_BARTERING"),
            LootTablesEnum::DesertWellArchaeology => f.write_str("DESERT_WELL_ARCHAEOLOGY"),
            LootTablesEnum::DesertPyramidArchaeology => f.write_str("DESERT_PYRAMID_ARCHAEOLOGY"),
            LootTablesEnum::TrailRuinsArchaeologyCommon => {
                f.write_str("TRAIL_RUINS_ARCHAEOLOGY_COMMON")
            }
            LootTablesEnum::TrailRuinsArchaeologyRare => {
                f.write_str("TRAIL_RUINS_ARCHAEOLOGY_RARE")
            }
            LootTablesEnum::OceanRuinWarmArchaeology => f.write_str("OCEAN_RUIN_WARM_ARCHAEOLOGY"),
            LootTablesEnum::OceanRuinColdArchaeology => f.write_str("OCEAN_RUIN_COLD_ARCHAEOLOGY"),
            LootTablesEnum::Sheep => f.write_str("SHEEP"),
            LootTablesEnum::SheepBlack => f.write_str("SHEEP_BLACK"),
            LootTablesEnum::SheepBlue => f.write_str("SHEEP_BLUE"),
            LootTablesEnum::SheepBrown => f.write_str("SHEEP_BROWN"),
            LootTablesEnum::SheepCyan => f.write_str("SHEEP_CYAN"),
            LootTablesEnum::SheepGray => f.write_str("SHEEP_GRAY"),
            LootTablesEnum::SheepGreen => f.write_str("SHEEP_GREEN"),
            LootTablesEnum::SheepLightBlue => f.write_str("SHEEP_LIGHT_BLUE"),
            LootTablesEnum::SheepLightGray => f.write_str("SHEEP_LIGHT_GRAY"),
            LootTablesEnum::SheepLime => f.write_str("SHEEP_LIME"),
            LootTablesEnum::SheepMagenta => f.write_str("SHEEP_MAGENTA"),
            LootTablesEnum::SheepOrange => f.write_str("SHEEP_ORANGE"),
            LootTablesEnum::SheepPink => f.write_str("SHEEP_PINK"),
            LootTablesEnum::SheepPurple => f.write_str("SHEEP_PURPLE"),
            LootTablesEnum::SheepRed => f.write_str("SHEEP_RED"),
            LootTablesEnum::SheepWhite => f.write_str("SHEEP_WHITE"),
            LootTablesEnum::SheepYellow => f.write_str("SHEEP_YELLOW"),
        }
    }
}
pub struct LootTables<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub LootTablesEnum,
);
impl<'mc> std::ops::Deref for LootTables<'mc> {
    type Target = LootTablesEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for LootTables<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LootTables<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: LootTablesEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LootTables from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "LootTables")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootTables object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const EMPTY: LootTablesEnum = LootTablesEnum::Empty;
    pub const ABANDONED_MINESHAFT: LootTablesEnum = LootTablesEnum::AbandonedMineshaft;
    pub const BURIED_TREASURE: LootTablesEnum = LootTablesEnum::BuriedTreasure;
    pub const DESERT_PYRAMID: LootTablesEnum = LootTablesEnum::DesertPyramid;
    pub const END_CITY_TREASURE: LootTablesEnum = LootTablesEnum::EndCityTreasure;
    pub const IGLOO_CHEST: LootTablesEnum = LootTablesEnum::IglooChest;
    pub const JUNGLE_TEMPLE: LootTablesEnum = LootTablesEnum::JungleTemple;
    pub const JUNGLE_TEMPLE_DISPENSER: LootTablesEnum = LootTablesEnum::JungleTempleDispenser;
    pub const NETHER_BRIDGE: LootTablesEnum = LootTablesEnum::NetherBridge;
    pub const PILLAGER_OUTPOST: LootTablesEnum = LootTablesEnum::PillagerOutpost;
    pub const BASTION_TREASURE: LootTablesEnum = LootTablesEnum::BastionTreasure;
    pub const BASTION_OTHER: LootTablesEnum = LootTablesEnum::BastionOther;
    pub const BASTION_BRIDGE: LootTablesEnum = LootTablesEnum::BastionBridge;
    pub const BASTION_HOGLIN_STABLE: LootTablesEnum = LootTablesEnum::BastionHoglinStable;
    pub const ANCIENT_CITY: LootTablesEnum = LootTablesEnum::AncientCity;
    pub const ANCIENT_CITY_ICE_BOX: LootTablesEnum = LootTablesEnum::AncientCityIceBox;
    pub const RUINED_PORTAL: LootTablesEnum = LootTablesEnum::RuinedPortal;
    pub const SHIPWRECK_MAP: LootTablesEnum = LootTablesEnum::ShipwreckMap;
    pub const SHIPWRECK_SUPPLY: LootTablesEnum = LootTablesEnum::ShipwreckSupply;
    pub const SHIPWRECK_TREASURE: LootTablesEnum = LootTablesEnum::ShipwreckTreasure;
    pub const SIMPLE_DUNGEON: LootTablesEnum = LootTablesEnum::SimpleDungeon;
    pub const SPAWN_BONUS_CHEST: LootTablesEnum = LootTablesEnum::SpawnBonusChest;
    pub const STRONGHOLD_CORRIDOR: LootTablesEnum = LootTablesEnum::StrongholdCorridor;
    pub const STRONGHOLD_CROSSING: LootTablesEnum = LootTablesEnum::StrongholdCrossing;
    pub const STRONGHOLD_LIBRARY: LootTablesEnum = LootTablesEnum::StrongholdLibrary;
    pub const UNDERWATER_RUIN_BIG: LootTablesEnum = LootTablesEnum::UnderwaterRuinBig;
    pub const UNDERWATER_RUIN_SMALL: LootTablesEnum = LootTablesEnum::UnderwaterRuinSmall;
    pub const VILLAGE_ARMORER: LootTablesEnum = LootTablesEnum::VillageArmorer;
    pub const VILLAGE_BUTCHER: LootTablesEnum = LootTablesEnum::VillageButcher;
    pub const VILLAGE_CARTOGRAPHER: LootTablesEnum = LootTablesEnum::VillageCartographer;
    pub const VILLAGE_DESERT_HOUSE: LootTablesEnum = LootTablesEnum::VillageDesertHouse;
    pub const VILLAGE_FISHER: LootTablesEnum = LootTablesEnum::VillageFisher;
    pub const VILLAGE_FLETCHER: LootTablesEnum = LootTablesEnum::VillageFletcher;
    pub const VILLAGE_MASON: LootTablesEnum = LootTablesEnum::VillageMason;
    pub const VILLAGE_PLAINS_HOUSE: LootTablesEnum = LootTablesEnum::VillagePlainsHouse;
    pub const VILLAGE_SAVANNA_HOUSE: LootTablesEnum = LootTablesEnum::VillageSavannaHouse;
    pub const VILLAGE_SHEPHERD: LootTablesEnum = LootTablesEnum::VillageShepherd;
    pub const VILLAGE_SNOWY_HOUSE: LootTablesEnum = LootTablesEnum::VillageSnowyHouse;
    pub const VILLAGE_TAIGA_HOUSE: LootTablesEnum = LootTablesEnum::VillageTaigaHouse;
    pub const VILLAGE_TANNERY: LootTablesEnum = LootTablesEnum::VillageTannery;
    pub const VILLAGE_TEMPLE: LootTablesEnum = LootTablesEnum::VillageTemple;
    pub const VILLAGE_TOOLSMITH: LootTablesEnum = LootTablesEnum::VillageToolsmith;
    pub const VILLAGE_WEAPONSMITH: LootTablesEnum = LootTablesEnum::VillageWeaponsmith;
    pub const WOODLAND_MANSION: LootTablesEnum = LootTablesEnum::WoodlandMansion;
    pub const ARMOR_STAND: LootTablesEnum = LootTablesEnum::ArmorStand;
    pub const AXOLOTL: LootTablesEnum = LootTablesEnum::Axolotl;
    pub const BAT: LootTablesEnum = LootTablesEnum::Bat;
    pub const BEE: LootTablesEnum = LootTablesEnum::Bee;
    pub const BLAZE: LootTablesEnum = LootTablesEnum::Blaze;
    pub const CAT: LootTablesEnum = LootTablesEnum::Cat;
    pub const CAVE_SPIDER: LootTablesEnum = LootTablesEnum::CaveSpider;
    pub const CHICKEN: LootTablesEnum = LootTablesEnum::Chicken;
    pub const COD: LootTablesEnum = LootTablesEnum::Cod;
    pub const COW: LootTablesEnum = LootTablesEnum::Cow;
    pub const CREEPER: LootTablesEnum = LootTablesEnum::Creeper;
    pub const DOLPHIN: LootTablesEnum = LootTablesEnum::Dolphin;
    pub const DONKEY: LootTablesEnum = LootTablesEnum::Donkey;
    pub const DROWNED: LootTablesEnum = LootTablesEnum::Drowned;
    pub const ELDER_GUARDIAN: LootTablesEnum = LootTablesEnum::ElderGuardian;
    pub const ENDER_DRAGON: LootTablesEnum = LootTablesEnum::EnderDragon;
    pub const ENDERMAN: LootTablesEnum = LootTablesEnum::Enderman;
    pub const ENDERMITE: LootTablesEnum = LootTablesEnum::Endermite;
    pub const EVOKER: LootTablesEnum = LootTablesEnum::Evoker;
    pub const FOX: LootTablesEnum = LootTablesEnum::Fox;
    pub const GHAST: LootTablesEnum = LootTablesEnum::Ghast;
    pub const GIANT: LootTablesEnum = LootTablesEnum::Giant;
    pub const GLOW_SQUID: LootTablesEnum = LootTablesEnum::GlowSquid;
    pub const GOAT: LootTablesEnum = LootTablesEnum::Goat;
    pub const GUARDIAN: LootTablesEnum = LootTablesEnum::Guardian;
    pub const HOGLIN: LootTablesEnum = LootTablesEnum::Hoglin;
    pub const HORSE: LootTablesEnum = LootTablesEnum::Horse;
    pub const HUSK: LootTablesEnum = LootTablesEnum::Husk;
    pub const ILLUSIONER: LootTablesEnum = LootTablesEnum::Illusioner;
    pub const IRON_GOLEM: LootTablesEnum = LootTablesEnum::IronGolem;
    pub const LLAMA: LootTablesEnum = LootTablesEnum::Llama;
    pub const MAGMA_CUBE: LootTablesEnum = LootTablesEnum::MagmaCube;
    pub const MOOSHROOM: LootTablesEnum = LootTablesEnum::Mooshroom;
    pub const MULE: LootTablesEnum = LootTablesEnum::Mule;
    pub const OCELOT: LootTablesEnum = LootTablesEnum::Ocelot;
    pub const PANDA: LootTablesEnum = LootTablesEnum::Panda;
    pub const PARROT: LootTablesEnum = LootTablesEnum::Parrot;
    pub const PHANTOM: LootTablesEnum = LootTablesEnum::Phantom;
    pub const PIG: LootTablesEnum = LootTablesEnum::Pig;
    pub const PIGLIN: LootTablesEnum = LootTablesEnum::Piglin;
    pub const PIGLIN_BRUTE: LootTablesEnum = LootTablesEnum::PiglinBrute;
    pub const PILLAGER: LootTablesEnum = LootTablesEnum::Pillager;
    pub const PLAYER: LootTablesEnum = LootTablesEnum::Player;
    pub const POLAR_BEAR: LootTablesEnum = LootTablesEnum::PolarBear;
    pub const PUFFERFISH: LootTablesEnum = LootTablesEnum::Pufferfish;
    pub const RABBIT: LootTablesEnum = LootTablesEnum::Rabbit;
    pub const RAVAGER: LootTablesEnum = LootTablesEnum::Ravager;
    pub const SALMON: LootTablesEnum = LootTablesEnum::Salmon;
    pub const SHULKER: LootTablesEnum = LootTablesEnum::Shulker;
    pub const SILVERFISH: LootTablesEnum = LootTablesEnum::Silverfish;
    pub const SKELETON: LootTablesEnum = LootTablesEnum::Skeleton;
    pub const SKELETON_HORSE: LootTablesEnum = LootTablesEnum::SkeletonHorse;
    pub const SLIME: LootTablesEnum = LootTablesEnum::Slime;
    pub const SNOW_GOLEM: LootTablesEnum = LootTablesEnum::SnowGolem;
    pub const SPIDER: LootTablesEnum = LootTablesEnum::Spider;
    pub const SQUID: LootTablesEnum = LootTablesEnum::Squid;
    pub const STRAY: LootTablesEnum = LootTablesEnum::Stray;
    pub const STRIDER: LootTablesEnum = LootTablesEnum::Strider;
    pub const TRADER_LLAMA: LootTablesEnum = LootTablesEnum::TraderLlama;
    pub const TROPICAL_FISH: LootTablesEnum = LootTablesEnum::TropicalFish;
    pub const TURTLE: LootTablesEnum = LootTablesEnum::Turtle;
    pub const VEX: LootTablesEnum = LootTablesEnum::Vex;
    pub const VILLAGER: LootTablesEnum = LootTablesEnum::Villager;
    pub const VINDICATOR: LootTablesEnum = LootTablesEnum::Vindicator;
    pub const WANDERING_TRADER: LootTablesEnum = LootTablesEnum::WanderingTrader;
    pub const WITCH: LootTablesEnum = LootTablesEnum::Witch;
    pub const WITHER: LootTablesEnum = LootTablesEnum::Wither;
    pub const WITHER_SKELETON: LootTablesEnum = LootTablesEnum::WitherSkeleton;
    pub const WOLF: LootTablesEnum = LootTablesEnum::Wolf;
    pub const ZOGLIN: LootTablesEnum = LootTablesEnum::Zoglin;
    pub const ZOMBIE: LootTablesEnum = LootTablesEnum::Zombie;
    pub const ZOMBIE_HORSE: LootTablesEnum = LootTablesEnum::ZombieHorse;
    pub const ZOMBIE_VILLAGER: LootTablesEnum = LootTablesEnum::ZombieVillager;
    pub const ZOMBIFIED_PIGLIN: LootTablesEnum = LootTablesEnum::ZombifiedPiglin;
    pub const ARMORER_GIFT: LootTablesEnum = LootTablesEnum::ArmorerGift;
    pub const BUTCHER_GIFT: LootTablesEnum = LootTablesEnum::ButcherGift;
    pub const CARTOGRAPHER_GIFT: LootTablesEnum = LootTablesEnum::CartographerGift;
    pub const CAT_MORNING_GIFT: LootTablesEnum = LootTablesEnum::CatMorningGift;
    pub const CLERIC_GIFT: LootTablesEnum = LootTablesEnum::ClericGift;
    pub const FARMER_GIFT: LootTablesEnum = LootTablesEnum::FarmerGift;
    pub const FISHERMAN_GIFT: LootTablesEnum = LootTablesEnum::FishermanGift;
    pub const FISHING: LootTablesEnum = LootTablesEnum::Fishing;
    pub const FISHING_FISH: LootTablesEnum = LootTablesEnum::FishingFish;
    pub const FISHING_JUNK: LootTablesEnum = LootTablesEnum::FishingJunk;
    pub const FISHING_TREASURE: LootTablesEnum = LootTablesEnum::FishingTreasure;
    pub const FLETCHER_GIFT: LootTablesEnum = LootTablesEnum::FletcherGift;
    pub const LEATHERWORKER_GIFT: LootTablesEnum = LootTablesEnum::LeatherworkerGift;
    pub const LIBRARIAN_GIFT: LootTablesEnum = LootTablesEnum::LibrarianGift;
    pub const MASON_GIFT: LootTablesEnum = LootTablesEnum::MasonGift;
    pub const SHEPHERD_GIFT: LootTablesEnum = LootTablesEnum::ShepherdGift;
    pub const TOOLSMITH_GIFT: LootTablesEnum = LootTablesEnum::ToolsmithGift;
    pub const WEAPONSMITH_GIFT: LootTablesEnum = LootTablesEnum::WeaponsmithGift;
    pub const SNIFFER_DIGGING: LootTablesEnum = LootTablesEnum::SnifferDigging;
    pub const PIGLIN_BARTERING: LootTablesEnum = LootTablesEnum::PiglinBartering;
    pub const DESERT_WELL_ARCHAEOLOGY: LootTablesEnum = LootTablesEnum::DesertWellArchaeology;
    pub const DESERT_PYRAMID_ARCHAEOLOGY: LootTablesEnum = LootTablesEnum::DesertPyramidArchaeology;
    pub const TRAIL_RUINS_ARCHAEOLOGY_COMMON: LootTablesEnum =
        LootTablesEnum::TrailRuinsArchaeologyCommon;
    pub const TRAIL_RUINS_ARCHAEOLOGY_RARE: LootTablesEnum =
        LootTablesEnum::TrailRuinsArchaeologyRare;
    pub const OCEAN_RUIN_WARM_ARCHAEOLOGY: LootTablesEnum =
        LootTablesEnum::OceanRuinWarmArchaeology;
    pub const OCEAN_RUIN_COLD_ARCHAEOLOGY: LootTablesEnum =
        LootTablesEnum::OceanRuinColdArchaeology;
    pub const SHEEP: LootTablesEnum = LootTablesEnum::Sheep;
    pub const SHEEP_BLACK: LootTablesEnum = LootTablesEnum::SheepBlack;
    pub const SHEEP_BLUE: LootTablesEnum = LootTablesEnum::SheepBlue;
    pub const SHEEP_BROWN: LootTablesEnum = LootTablesEnum::SheepBrown;
    pub const SHEEP_CYAN: LootTablesEnum = LootTablesEnum::SheepCyan;
    pub const SHEEP_GRAY: LootTablesEnum = LootTablesEnum::SheepGray;
    pub const SHEEP_GREEN: LootTablesEnum = LootTablesEnum::SheepGreen;
    pub const SHEEP_LIGHT_BLUE: LootTablesEnum = LootTablesEnum::SheepLightBlue;
    pub const SHEEP_LIGHT_GRAY: LootTablesEnum = LootTablesEnum::SheepLightGray;
    pub const SHEEP_LIME: LootTablesEnum = LootTablesEnum::SheepLime;
    pub const SHEEP_MAGENTA: LootTablesEnum = LootTablesEnum::SheepMagenta;
    pub const SHEEP_ORANGE: LootTablesEnum = LootTablesEnum::SheepOrange;
    pub const SHEEP_PINK: LootTablesEnum = LootTablesEnum::SheepPink;
    pub const SHEEP_PURPLE: LootTablesEnum = LootTablesEnum::SheepPurple;
    pub const SHEEP_RED: LootTablesEnum = LootTablesEnum::SheepRed;
    pub const SHEEP_WHITE: LootTablesEnum = LootTablesEnum::SheepWhite;
    pub const SHEEP_YELLOW: LootTablesEnum = LootTablesEnum::SheepYellow;
    pub fn from_string(str: String) -> std::option::Option<LootTablesEnum> {
        match str.as_str() {
            "EMPTY" => Some(LootTablesEnum::Empty),
            "ABANDONED_MINESHAFT" => Some(LootTablesEnum::AbandonedMineshaft),
            "BURIED_TREASURE" => Some(LootTablesEnum::BuriedTreasure),
            "DESERT_PYRAMID" => Some(LootTablesEnum::DesertPyramid),
            "END_CITY_TREASURE" => Some(LootTablesEnum::EndCityTreasure),
            "IGLOO_CHEST" => Some(LootTablesEnum::IglooChest),
            "JUNGLE_TEMPLE" => Some(LootTablesEnum::JungleTemple),
            "JUNGLE_TEMPLE_DISPENSER" => Some(LootTablesEnum::JungleTempleDispenser),
            "NETHER_BRIDGE" => Some(LootTablesEnum::NetherBridge),
            "PILLAGER_OUTPOST" => Some(LootTablesEnum::PillagerOutpost),
            "BASTION_TREASURE" => Some(LootTablesEnum::BastionTreasure),
            "BASTION_OTHER" => Some(LootTablesEnum::BastionOther),
            "BASTION_BRIDGE" => Some(LootTablesEnum::BastionBridge),
            "BASTION_HOGLIN_STABLE" => Some(LootTablesEnum::BastionHoglinStable),
            "ANCIENT_CITY" => Some(LootTablesEnum::AncientCity),
            "ANCIENT_CITY_ICE_BOX" => Some(LootTablesEnum::AncientCityIceBox),
            "RUINED_PORTAL" => Some(LootTablesEnum::RuinedPortal),
            "SHIPWRECK_MAP" => Some(LootTablesEnum::ShipwreckMap),
            "SHIPWRECK_SUPPLY" => Some(LootTablesEnum::ShipwreckSupply),
            "SHIPWRECK_TREASURE" => Some(LootTablesEnum::ShipwreckTreasure),
            "SIMPLE_DUNGEON" => Some(LootTablesEnum::SimpleDungeon),
            "SPAWN_BONUS_CHEST" => Some(LootTablesEnum::SpawnBonusChest),
            "STRONGHOLD_CORRIDOR" => Some(LootTablesEnum::StrongholdCorridor),
            "STRONGHOLD_CROSSING" => Some(LootTablesEnum::StrongholdCrossing),
            "STRONGHOLD_LIBRARY" => Some(LootTablesEnum::StrongholdLibrary),
            "UNDERWATER_RUIN_BIG" => Some(LootTablesEnum::UnderwaterRuinBig),
            "UNDERWATER_RUIN_SMALL" => Some(LootTablesEnum::UnderwaterRuinSmall),
            "VILLAGE_ARMORER" => Some(LootTablesEnum::VillageArmorer),
            "VILLAGE_BUTCHER" => Some(LootTablesEnum::VillageButcher),
            "VILLAGE_CARTOGRAPHER" => Some(LootTablesEnum::VillageCartographer),
            "VILLAGE_DESERT_HOUSE" => Some(LootTablesEnum::VillageDesertHouse),
            "VILLAGE_FISHER" => Some(LootTablesEnum::VillageFisher),
            "VILLAGE_FLETCHER" => Some(LootTablesEnum::VillageFletcher),
            "VILLAGE_MASON" => Some(LootTablesEnum::VillageMason),
            "VILLAGE_PLAINS_HOUSE" => Some(LootTablesEnum::VillagePlainsHouse),
            "VILLAGE_SAVANNA_HOUSE" => Some(LootTablesEnum::VillageSavannaHouse),
            "VILLAGE_SHEPHERD" => Some(LootTablesEnum::VillageShepherd),
            "VILLAGE_SNOWY_HOUSE" => Some(LootTablesEnum::VillageSnowyHouse),
            "VILLAGE_TAIGA_HOUSE" => Some(LootTablesEnum::VillageTaigaHouse),
            "VILLAGE_TANNERY" => Some(LootTablesEnum::VillageTannery),
            "VILLAGE_TEMPLE" => Some(LootTablesEnum::VillageTemple),
            "VILLAGE_TOOLSMITH" => Some(LootTablesEnum::VillageToolsmith),
            "VILLAGE_WEAPONSMITH" => Some(LootTablesEnum::VillageWeaponsmith),
            "WOODLAND_MANSION" => Some(LootTablesEnum::WoodlandMansion),
            "ARMOR_STAND" => Some(LootTablesEnum::ArmorStand),
            "AXOLOTL" => Some(LootTablesEnum::Axolotl),
            "BAT" => Some(LootTablesEnum::Bat),
            "BEE" => Some(LootTablesEnum::Bee),
            "BLAZE" => Some(LootTablesEnum::Blaze),
            "CAT" => Some(LootTablesEnum::Cat),
            "CAVE_SPIDER" => Some(LootTablesEnum::CaveSpider),
            "CHICKEN" => Some(LootTablesEnum::Chicken),
            "COD" => Some(LootTablesEnum::Cod),
            "COW" => Some(LootTablesEnum::Cow),
            "CREEPER" => Some(LootTablesEnum::Creeper),
            "DOLPHIN" => Some(LootTablesEnum::Dolphin),
            "DONKEY" => Some(LootTablesEnum::Donkey),
            "DROWNED" => Some(LootTablesEnum::Drowned),
            "ELDER_GUARDIAN" => Some(LootTablesEnum::ElderGuardian),
            "ENDER_DRAGON" => Some(LootTablesEnum::EnderDragon),
            "ENDERMAN" => Some(LootTablesEnum::Enderman),
            "ENDERMITE" => Some(LootTablesEnum::Endermite),
            "EVOKER" => Some(LootTablesEnum::Evoker),
            "FOX" => Some(LootTablesEnum::Fox),
            "GHAST" => Some(LootTablesEnum::Ghast),
            "GIANT" => Some(LootTablesEnum::Giant),
            "GLOW_SQUID" => Some(LootTablesEnum::GlowSquid),
            "GOAT" => Some(LootTablesEnum::Goat),
            "GUARDIAN" => Some(LootTablesEnum::Guardian),
            "HOGLIN" => Some(LootTablesEnum::Hoglin),
            "HORSE" => Some(LootTablesEnum::Horse),
            "HUSK" => Some(LootTablesEnum::Husk),
            "ILLUSIONER" => Some(LootTablesEnum::Illusioner),
            "IRON_GOLEM" => Some(LootTablesEnum::IronGolem),
            "LLAMA" => Some(LootTablesEnum::Llama),
            "MAGMA_CUBE" => Some(LootTablesEnum::MagmaCube),
            "MOOSHROOM" => Some(LootTablesEnum::Mooshroom),
            "MULE" => Some(LootTablesEnum::Mule),
            "OCELOT" => Some(LootTablesEnum::Ocelot),
            "PANDA" => Some(LootTablesEnum::Panda),
            "PARROT" => Some(LootTablesEnum::Parrot),
            "PHANTOM" => Some(LootTablesEnum::Phantom),
            "PIG" => Some(LootTablesEnum::Pig),
            "PIGLIN" => Some(LootTablesEnum::Piglin),
            "PIGLIN_BRUTE" => Some(LootTablesEnum::PiglinBrute),
            "PILLAGER" => Some(LootTablesEnum::Pillager),
            "PLAYER" => Some(LootTablesEnum::Player),
            "POLAR_BEAR" => Some(LootTablesEnum::PolarBear),
            "PUFFERFISH" => Some(LootTablesEnum::Pufferfish),
            "RABBIT" => Some(LootTablesEnum::Rabbit),
            "RAVAGER" => Some(LootTablesEnum::Ravager),
            "SALMON" => Some(LootTablesEnum::Salmon),
            "SHULKER" => Some(LootTablesEnum::Shulker),
            "SILVERFISH" => Some(LootTablesEnum::Silverfish),
            "SKELETON" => Some(LootTablesEnum::Skeleton),
            "SKELETON_HORSE" => Some(LootTablesEnum::SkeletonHorse),
            "SLIME" => Some(LootTablesEnum::Slime),
            "SNOW_GOLEM" => Some(LootTablesEnum::SnowGolem),
            "SPIDER" => Some(LootTablesEnum::Spider),
            "SQUID" => Some(LootTablesEnum::Squid),
            "STRAY" => Some(LootTablesEnum::Stray),
            "STRIDER" => Some(LootTablesEnum::Strider),
            "TRADER_LLAMA" => Some(LootTablesEnum::TraderLlama),
            "TROPICAL_FISH" => Some(LootTablesEnum::TropicalFish),
            "TURTLE" => Some(LootTablesEnum::Turtle),
            "VEX" => Some(LootTablesEnum::Vex),
            "VILLAGER" => Some(LootTablesEnum::Villager),
            "VINDICATOR" => Some(LootTablesEnum::Vindicator),
            "WANDERING_TRADER" => Some(LootTablesEnum::WanderingTrader),
            "WITCH" => Some(LootTablesEnum::Witch),
            "WITHER" => Some(LootTablesEnum::Wither),
            "WITHER_SKELETON" => Some(LootTablesEnum::WitherSkeleton),
            "WOLF" => Some(LootTablesEnum::Wolf),
            "ZOGLIN" => Some(LootTablesEnum::Zoglin),
            "ZOMBIE" => Some(LootTablesEnum::Zombie),
            "ZOMBIE_HORSE" => Some(LootTablesEnum::ZombieHorse),
            "ZOMBIE_VILLAGER" => Some(LootTablesEnum::ZombieVillager),
            "ZOMBIFIED_PIGLIN" => Some(LootTablesEnum::ZombifiedPiglin),
            "ARMORER_GIFT" => Some(LootTablesEnum::ArmorerGift),
            "BUTCHER_GIFT" => Some(LootTablesEnum::ButcherGift),
            "CARTOGRAPHER_GIFT" => Some(LootTablesEnum::CartographerGift),
            "CAT_MORNING_GIFT" => Some(LootTablesEnum::CatMorningGift),
            "CLERIC_GIFT" => Some(LootTablesEnum::ClericGift),
            "FARMER_GIFT" => Some(LootTablesEnum::FarmerGift),
            "FISHERMAN_GIFT" => Some(LootTablesEnum::FishermanGift),
            "FISHING" => Some(LootTablesEnum::Fishing),
            "FISHING_FISH" => Some(LootTablesEnum::FishingFish),
            "FISHING_JUNK" => Some(LootTablesEnum::FishingJunk),
            "FISHING_TREASURE" => Some(LootTablesEnum::FishingTreasure),
            "FLETCHER_GIFT" => Some(LootTablesEnum::FletcherGift),
            "LEATHERWORKER_GIFT" => Some(LootTablesEnum::LeatherworkerGift),
            "LIBRARIAN_GIFT" => Some(LootTablesEnum::LibrarianGift),
            "MASON_GIFT" => Some(LootTablesEnum::MasonGift),
            "SHEPHERD_GIFT" => Some(LootTablesEnum::ShepherdGift),
            "TOOLSMITH_GIFT" => Some(LootTablesEnum::ToolsmithGift),
            "WEAPONSMITH_GIFT" => Some(LootTablesEnum::WeaponsmithGift),
            "SNIFFER_DIGGING" => Some(LootTablesEnum::SnifferDigging),
            "PIGLIN_BARTERING" => Some(LootTablesEnum::PiglinBartering),
            "DESERT_WELL_ARCHAEOLOGY" => Some(LootTablesEnum::DesertWellArchaeology),
            "DESERT_PYRAMID_ARCHAEOLOGY" => Some(LootTablesEnum::DesertPyramidArchaeology),
            "TRAIL_RUINS_ARCHAEOLOGY_COMMON" => Some(LootTablesEnum::TrailRuinsArchaeologyCommon),
            "TRAIL_RUINS_ARCHAEOLOGY_RARE" => Some(LootTablesEnum::TrailRuinsArchaeologyRare),
            "OCEAN_RUIN_WARM_ARCHAEOLOGY" => Some(LootTablesEnum::OceanRuinWarmArchaeology),
            "OCEAN_RUIN_COLD_ARCHAEOLOGY" => Some(LootTablesEnum::OceanRuinColdArchaeology),
            "SHEEP" => Some(LootTablesEnum::Sheep),
            "SHEEP_BLACK" => Some(LootTablesEnum::SheepBlack),
            "SHEEP_BLUE" => Some(LootTablesEnum::SheepBlue),
            "SHEEP_BROWN" => Some(LootTablesEnum::SheepBrown),
            "SHEEP_CYAN" => Some(LootTablesEnum::SheepCyan),
            "SHEEP_GRAY" => Some(LootTablesEnum::SheepGray),
            "SHEEP_GREEN" => Some(LootTablesEnum::SheepGreen),
            "SHEEP_LIGHT_BLUE" => Some(LootTablesEnum::SheepLightBlue),
            "SHEEP_LIGHT_GRAY" => Some(LootTablesEnum::SheepLightGray),
            "SHEEP_LIME" => Some(LootTablesEnum::SheepLime),
            "SHEEP_MAGENTA" => Some(LootTablesEnum::SheepMagenta),
            "SHEEP_ORANGE" => Some(LootTablesEnum::SheepOrange),
            "SHEEP_PINK" => Some(LootTablesEnum::SheepPink),
            "SHEEP_PURPLE" => Some(LootTablesEnum::SheepPurple),
            "SHEEP_RED" => Some(LootTablesEnum::SheepRed),
            "SHEEP_WHITE" => Some(LootTablesEnum::SheepWhite),
            "SHEEP_YELLOW" => Some(LootTablesEnum::SheepYellow),
            _ => None,
        }
    }
}
pub struct LootContext<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct LootContextBuilder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for LootContextBuilder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LootContextBuilder<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LootContextBuilder from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "LootContextBuilder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootContextBuilder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::Location<'mc>>,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/loot/LootContext$Builder")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/Location;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::loot::LootContextBuilder::from_raw(&jni, res)
    }
    pub fn build(&mut self) -> Result<crate::loot::LootContext<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "build",
            "()Lorg/bukkit/loot/LootContext;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContext::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn luck(
        &mut self,
        arg0: f32,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "luck",
            "(F)Lorg/bukkit/loot/LootContext$Builder;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn killer(
        &mut self,
        arg0: impl Into<&'mc crate::entity::HumanEntity<'mc>>,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "killer",
            "(Lorg/bukkit/entity/HumanEntity;)Lorg/bukkit/loot/LootContext$Builder;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn looting_modifier(
        &mut self,
        arg0: i32,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lootingModifier",
            "(I)Lorg/bukkit/loot/LootContext$Builder;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn looted_entity(
        &mut self,
        arg0: impl Into<&'mc crate::entity::Entity<'mc>>,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lootedEntity",
            "(Lorg/bukkit/entity/Entity;)Lorg/bukkit/loot/LootContext$Builder;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
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
impl<'mc> blackboxmc_general::JNIRaw<'mc> for LootContext<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LootContext<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LootContext from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "LootContext")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootContext object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn killer(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKiller",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn luck(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLuck", "()F", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f().unwrap())
    }
    pub fn looting_modifier(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLootingModifier", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn looted_entity(
        &mut self,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootedEntity",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
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
