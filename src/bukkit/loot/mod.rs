use crate::JNIRaw;
/// An instantiatable struct that implements LootTable. Needed for returning it from Java.
pub struct LootTable<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> LootTable<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LootTable from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("LootTable") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootTable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn fill_inventory(
        &mut self,
        arg0: crate::bukkit::inventory::Inventory<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: crate::bukkit::loot::LootContext<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let val_1 = arg1;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg2.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "fillInventory",
            "(Lorg/bukkit/inventory/Inventory;Ljava/util/Random;Lorg/bukkit/loot/LootContext;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        Ok(())
    }
    pub fn key(&mut self) -> Result<crate::bukkit::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKey",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        )?;
        let ret = {
            crate::bukkit::NamespacedKey(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for LootTable<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// An instantiatable struct that implements Lootable. Needed for returning it from Java.
pub struct Lootable<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Lootable<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Lootable from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("Lootable") {
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
            .call_method(&self.jni_object(), "getSeed", "()J", &[])?;
        Ok(res.j().unwrap())
    }
    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.into());
        self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            "(J)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn set_loot_table(
        &mut self,
        arg0: crate::bukkit::loot::LootTable<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            "(Lorg/bukkit/loot/LootTable;)V",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        Ok(())
    }
    pub fn loot_table(
        &mut self,
    ) -> Result<crate::bukkit::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        )?;
        let ret = {
            crate::bukkit::loot::LootTable(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
impl<'mc> crate::JNIRaw<'mc> for Lootable<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
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
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub LootTablesEnum,
);
impl<'mc> std::ops::Deref for LootTables<'mc> {
    type Target = LootTablesEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> crate::JNIRaw<'mc> for LootTables<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LootTables<'mc> {
    pub const EMPTY: LootTablesEnum = LootTablesEnum::Empty;
    pub const ABANDONEDMINESHAFT: LootTablesEnum = LootTablesEnum::AbandonedMineshaft;
    pub const BURIEDTREASURE: LootTablesEnum = LootTablesEnum::BuriedTreasure;
    pub const DESERTPYRAMID: LootTablesEnum = LootTablesEnum::DesertPyramid;
    pub const ENDCITYTREASURE: LootTablesEnum = LootTablesEnum::EndCityTreasure;
    pub const IGLOOCHEST: LootTablesEnum = LootTablesEnum::IglooChest;
    pub const JUNGLETEMPLE: LootTablesEnum = LootTablesEnum::JungleTemple;
    pub const JUNGLETEMPLEDISPENSER: LootTablesEnum = LootTablesEnum::JungleTempleDispenser;
    pub const NETHERBRIDGE: LootTablesEnum = LootTablesEnum::NetherBridge;
    pub const PILLAGEROUTPOST: LootTablesEnum = LootTablesEnum::PillagerOutpost;
    pub const BASTIONTREASURE: LootTablesEnum = LootTablesEnum::BastionTreasure;
    pub const BASTIONOTHER: LootTablesEnum = LootTablesEnum::BastionOther;
    pub const BASTIONBRIDGE: LootTablesEnum = LootTablesEnum::BastionBridge;
    pub const BASTIONHOGLINSTABLE: LootTablesEnum = LootTablesEnum::BastionHoglinStable;
    pub const ANCIENTCITY: LootTablesEnum = LootTablesEnum::AncientCity;
    pub const ANCIENTCITYICEBOX: LootTablesEnum = LootTablesEnum::AncientCityIceBox;
    pub const RUINEDPORTAL: LootTablesEnum = LootTablesEnum::RuinedPortal;
    pub const SHIPWRECKMAP: LootTablesEnum = LootTablesEnum::ShipwreckMap;
    pub const SHIPWRECKSUPPLY: LootTablesEnum = LootTablesEnum::ShipwreckSupply;
    pub const SHIPWRECKTREASURE: LootTablesEnum = LootTablesEnum::ShipwreckTreasure;
    pub const SIMPLEDUNGEON: LootTablesEnum = LootTablesEnum::SimpleDungeon;
    pub const SPAWNBONUSCHEST: LootTablesEnum = LootTablesEnum::SpawnBonusChest;
    pub const STRONGHOLDCORRIDOR: LootTablesEnum = LootTablesEnum::StrongholdCorridor;
    pub const STRONGHOLDCROSSING: LootTablesEnum = LootTablesEnum::StrongholdCrossing;
    pub const STRONGHOLDLIBRARY: LootTablesEnum = LootTablesEnum::StrongholdLibrary;
    pub const UNDERWATERRUINBIG: LootTablesEnum = LootTablesEnum::UnderwaterRuinBig;
    pub const UNDERWATERRUINSMALL: LootTablesEnum = LootTablesEnum::UnderwaterRuinSmall;
    pub const VILLAGEARMORER: LootTablesEnum = LootTablesEnum::VillageArmorer;
    pub const VILLAGEBUTCHER: LootTablesEnum = LootTablesEnum::VillageButcher;
    pub const VILLAGECARTOGRAPHER: LootTablesEnum = LootTablesEnum::VillageCartographer;
    pub const VILLAGEDESERTHOUSE: LootTablesEnum = LootTablesEnum::VillageDesertHouse;
    pub const VILLAGEFISHER: LootTablesEnum = LootTablesEnum::VillageFisher;
    pub const VILLAGEFLETCHER: LootTablesEnum = LootTablesEnum::VillageFletcher;
    pub const VILLAGEMASON: LootTablesEnum = LootTablesEnum::VillageMason;
    pub const VILLAGEPLAINSHOUSE: LootTablesEnum = LootTablesEnum::VillagePlainsHouse;
    pub const VILLAGESAVANNAHOUSE: LootTablesEnum = LootTablesEnum::VillageSavannaHouse;
    pub const VILLAGESHEPHERD: LootTablesEnum = LootTablesEnum::VillageShepherd;
    pub const VILLAGESNOWYHOUSE: LootTablesEnum = LootTablesEnum::VillageSnowyHouse;
    pub const VILLAGETAIGAHOUSE: LootTablesEnum = LootTablesEnum::VillageTaigaHouse;
    pub const VILLAGETANNERY: LootTablesEnum = LootTablesEnum::VillageTannery;
    pub const VILLAGETEMPLE: LootTablesEnum = LootTablesEnum::VillageTemple;
    pub const VILLAGETOOLSMITH: LootTablesEnum = LootTablesEnum::VillageToolsmith;
    pub const VILLAGEWEAPONSMITH: LootTablesEnum = LootTablesEnum::VillageWeaponsmith;
    pub const WOODLANDMANSION: LootTablesEnum = LootTablesEnum::WoodlandMansion;
    pub const ARMORSTAND: LootTablesEnum = LootTablesEnum::ArmorStand;
    pub const AXOLOTL: LootTablesEnum = LootTablesEnum::Axolotl;
    pub const BAT: LootTablesEnum = LootTablesEnum::Bat;
    pub const BEE: LootTablesEnum = LootTablesEnum::Bee;
    pub const BLAZE: LootTablesEnum = LootTablesEnum::Blaze;
    pub const CAT: LootTablesEnum = LootTablesEnum::Cat;
    pub const CAVESPIDER: LootTablesEnum = LootTablesEnum::CaveSpider;
    pub const CHICKEN: LootTablesEnum = LootTablesEnum::Chicken;
    pub const COD: LootTablesEnum = LootTablesEnum::Cod;
    pub const COW: LootTablesEnum = LootTablesEnum::Cow;
    pub const CREEPER: LootTablesEnum = LootTablesEnum::Creeper;
    pub const DOLPHIN: LootTablesEnum = LootTablesEnum::Dolphin;
    pub const DONKEY: LootTablesEnum = LootTablesEnum::Donkey;
    pub const DROWNED: LootTablesEnum = LootTablesEnum::Drowned;
    pub const ELDERGUARDIAN: LootTablesEnum = LootTablesEnum::ElderGuardian;
    pub const ENDERDRAGON: LootTablesEnum = LootTablesEnum::EnderDragon;
    pub const ENDERMAN: LootTablesEnum = LootTablesEnum::Enderman;
    pub const ENDERMITE: LootTablesEnum = LootTablesEnum::Endermite;
    pub const EVOKER: LootTablesEnum = LootTablesEnum::Evoker;
    pub const FOX: LootTablesEnum = LootTablesEnum::Fox;
    pub const GHAST: LootTablesEnum = LootTablesEnum::Ghast;
    pub const GIANT: LootTablesEnum = LootTablesEnum::Giant;
    pub const GLOWSQUID: LootTablesEnum = LootTablesEnum::GlowSquid;
    pub const GOAT: LootTablesEnum = LootTablesEnum::Goat;
    pub const GUARDIAN: LootTablesEnum = LootTablesEnum::Guardian;
    pub const HOGLIN: LootTablesEnum = LootTablesEnum::Hoglin;
    pub const HORSE: LootTablesEnum = LootTablesEnum::Horse;
    pub const HUSK: LootTablesEnum = LootTablesEnum::Husk;
    pub const ILLUSIONER: LootTablesEnum = LootTablesEnum::Illusioner;
    pub const IRONGOLEM: LootTablesEnum = LootTablesEnum::IronGolem;
    pub const LLAMA: LootTablesEnum = LootTablesEnum::Llama;
    pub const MAGMACUBE: LootTablesEnum = LootTablesEnum::MagmaCube;
    pub const MOOSHROOM: LootTablesEnum = LootTablesEnum::Mooshroom;
    pub const MULE: LootTablesEnum = LootTablesEnum::Mule;
    pub const OCELOT: LootTablesEnum = LootTablesEnum::Ocelot;
    pub const PANDA: LootTablesEnum = LootTablesEnum::Panda;
    pub const PARROT: LootTablesEnum = LootTablesEnum::Parrot;
    pub const PHANTOM: LootTablesEnum = LootTablesEnum::Phantom;
    pub const PIG: LootTablesEnum = LootTablesEnum::Pig;
    pub const PIGLIN: LootTablesEnum = LootTablesEnum::Piglin;
    pub const PIGLINBRUTE: LootTablesEnum = LootTablesEnum::PiglinBrute;
    pub const PILLAGER: LootTablesEnum = LootTablesEnum::Pillager;
    pub const PLAYER: LootTablesEnum = LootTablesEnum::Player;
    pub const POLARBEAR: LootTablesEnum = LootTablesEnum::PolarBear;
    pub const PUFFERFISH: LootTablesEnum = LootTablesEnum::Pufferfish;
    pub const RABBIT: LootTablesEnum = LootTablesEnum::Rabbit;
    pub const RAVAGER: LootTablesEnum = LootTablesEnum::Ravager;
    pub const SALMON: LootTablesEnum = LootTablesEnum::Salmon;
    pub const SHULKER: LootTablesEnum = LootTablesEnum::Shulker;
    pub const SILVERFISH: LootTablesEnum = LootTablesEnum::Silverfish;
    pub const SKELETON: LootTablesEnum = LootTablesEnum::Skeleton;
    pub const SKELETONHORSE: LootTablesEnum = LootTablesEnum::SkeletonHorse;
    pub const SLIME: LootTablesEnum = LootTablesEnum::Slime;
    pub const SNOWGOLEM: LootTablesEnum = LootTablesEnum::SnowGolem;
    pub const SPIDER: LootTablesEnum = LootTablesEnum::Spider;
    pub const SQUID: LootTablesEnum = LootTablesEnum::Squid;
    pub const STRAY: LootTablesEnum = LootTablesEnum::Stray;
    pub const STRIDER: LootTablesEnum = LootTablesEnum::Strider;
    pub const TRADERLLAMA: LootTablesEnum = LootTablesEnum::TraderLlama;
    pub const TROPICALFISH: LootTablesEnum = LootTablesEnum::TropicalFish;
    pub const TURTLE: LootTablesEnum = LootTablesEnum::Turtle;
    pub const VEX: LootTablesEnum = LootTablesEnum::Vex;
    pub const VILLAGER: LootTablesEnum = LootTablesEnum::Villager;
    pub const VINDICATOR: LootTablesEnum = LootTablesEnum::Vindicator;
    pub const WANDERINGTRADER: LootTablesEnum = LootTablesEnum::WanderingTrader;
    pub const WITCH: LootTablesEnum = LootTablesEnum::Witch;
    pub const WITHER: LootTablesEnum = LootTablesEnum::Wither;
    pub const WITHERSKELETON: LootTablesEnum = LootTablesEnum::WitherSkeleton;
    pub const WOLF: LootTablesEnum = LootTablesEnum::Wolf;
    pub const ZOGLIN: LootTablesEnum = LootTablesEnum::Zoglin;
    pub const ZOMBIE: LootTablesEnum = LootTablesEnum::Zombie;
    pub const ZOMBIEHORSE: LootTablesEnum = LootTablesEnum::ZombieHorse;
    pub const ZOMBIEVILLAGER: LootTablesEnum = LootTablesEnum::ZombieVillager;
    pub const ZOMBIFIEDPIGLIN: LootTablesEnum = LootTablesEnum::ZombifiedPiglin;
    pub const ARMORERGIFT: LootTablesEnum = LootTablesEnum::ArmorerGift;
    pub const BUTCHERGIFT: LootTablesEnum = LootTablesEnum::ButcherGift;
    pub const CARTOGRAPHERGIFT: LootTablesEnum = LootTablesEnum::CartographerGift;
    pub const CATMORNINGGIFT: LootTablesEnum = LootTablesEnum::CatMorningGift;
    pub const CLERICGIFT: LootTablesEnum = LootTablesEnum::ClericGift;
    pub const FARMERGIFT: LootTablesEnum = LootTablesEnum::FarmerGift;
    pub const FISHERMANGIFT: LootTablesEnum = LootTablesEnum::FishermanGift;
    pub const FISHING: LootTablesEnum = LootTablesEnum::Fishing;
    pub const FISHINGFISH: LootTablesEnum = LootTablesEnum::FishingFish;
    pub const FISHINGJUNK: LootTablesEnum = LootTablesEnum::FishingJunk;
    pub const FISHINGTREASURE: LootTablesEnum = LootTablesEnum::FishingTreasure;
    pub const FLETCHERGIFT: LootTablesEnum = LootTablesEnum::FletcherGift;
    pub const LEATHERWORKERGIFT: LootTablesEnum = LootTablesEnum::LeatherworkerGift;
    pub const LIBRARIANGIFT: LootTablesEnum = LootTablesEnum::LibrarianGift;
    pub const MASONGIFT: LootTablesEnum = LootTablesEnum::MasonGift;
    pub const SHEPHERDGIFT: LootTablesEnum = LootTablesEnum::ShepherdGift;
    pub const TOOLSMITHGIFT: LootTablesEnum = LootTablesEnum::ToolsmithGift;
    pub const WEAPONSMITHGIFT: LootTablesEnum = LootTablesEnum::WeaponsmithGift;
    pub const SNIFFERDIGGING: LootTablesEnum = LootTablesEnum::SnifferDigging;
    pub const PIGLINBARTERING: LootTablesEnum = LootTablesEnum::PiglinBartering;
    pub const DESERTWELLARCHAEOLOGY: LootTablesEnum = LootTablesEnum::DesertWellArchaeology;
    pub const DESERTPYRAMIDARCHAEOLOGY: LootTablesEnum = LootTablesEnum::DesertPyramidArchaeology;
    pub const TRAILRUINSARCHAEOLOGYCOMMON: LootTablesEnum =
        LootTablesEnum::TrailRuinsArchaeologyCommon;
    pub const TRAILRUINSARCHAEOLOGYRARE: LootTablesEnum = LootTablesEnum::TrailRuinsArchaeologyRare;
    pub const OCEANRUINWARMARCHAEOLOGY: LootTablesEnum = LootTablesEnum::OceanRuinWarmArchaeology;
    pub const OCEANRUINCOLDARCHAEOLOGY: LootTablesEnum = LootTablesEnum::OceanRuinColdArchaeology;
    pub const SHEEP: LootTablesEnum = LootTablesEnum::Sheep;
    pub const SHEEPBLACK: LootTablesEnum = LootTablesEnum::SheepBlack;
    pub const SHEEPBLUE: LootTablesEnum = LootTablesEnum::SheepBlue;
    pub const SHEEPBROWN: LootTablesEnum = LootTablesEnum::SheepBrown;
    pub const SHEEPCYAN: LootTablesEnum = LootTablesEnum::SheepCyan;
    pub const SHEEPGRAY: LootTablesEnum = LootTablesEnum::SheepGray;
    pub const SHEEPGREEN: LootTablesEnum = LootTablesEnum::SheepGreen;
    pub const SHEEPLIGHTBLUE: LootTablesEnum = LootTablesEnum::SheepLightBlue;
    pub const SHEEPLIGHTGRAY: LootTablesEnum = LootTablesEnum::SheepLightGray;
    pub const SHEEPLIME: LootTablesEnum = LootTablesEnum::SheepLime;
    pub const SHEEPMAGENTA: LootTablesEnum = LootTablesEnum::SheepMagenta;
    pub const SHEEPORANGE: LootTablesEnum = LootTablesEnum::SheepOrange;
    pub const SHEEPPINK: LootTablesEnum = LootTablesEnum::SheepPink;
    pub const SHEEPPURPLE: LootTablesEnum = LootTablesEnum::SheepPurple;
    pub const SHEEPRED: LootTablesEnum = LootTablesEnum::SheepRed;
    pub const SHEEPWHITE: LootTablesEnum = LootTablesEnum::SheepWhite;
    pub const SHEEPYELLOW: LootTablesEnum = LootTablesEnum::SheepYellow;
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
    pub fn value_of(
        mut jni: crate::SharedJNIEnv<'mc>,
        arg0: String,
    ) -> Result<crate::bukkit::loot::LootTables<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0).unwrap());
        let cls = &jni.find_class("org/bukkit/loot/LootTables")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/loot/LootTables;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            let obj = res.l()?;
            let raw_obj = obj;
            let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
            let variant_str = jni
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            crate::bukkit::loot::LootTables(
                jni,
                raw_obj,
                crate::bukkit::loot::LootTables::from_string(variant_str).unwrap(),
            )
        };
        Ok(ret)
    }
    pub fn key(&mut self) -> Result<crate::bukkit::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKey",
            "()Lorg/bukkit/NamespacedKey;",
            &[],
        )?;
        let ret = {
            crate::bukkit::NamespacedKey(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn loot_table(
        &mut self,
    ) -> Result<crate::bukkit::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootTable",
            "()Lorg/bukkit/loot/LootTable;",
            &[],
        )?;
        let ret = {
            crate::bukkit::loot::LootTable(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
}
pub struct LootContext<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
pub struct LootContextBuilder<'mc>(
    pub(crate) crate::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> crate::JNIRaw<'mc> for LootContextBuilder<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LootContextBuilder<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LootContextBuilder from null object.").into(),
            );
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("LootContextBuilder") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootContextBuilder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn build(
        &mut self,
    ) -> Result<crate::bukkit::loot::LootContext<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "build",
            "()Lorg/bukkit/loot/LootContext;",
            &[],
        )?;
        let ret = {
            crate::bukkit::loot::LootContext(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn luck(
        &mut self,
        arg0: f32,
    ) -> Result<crate::bukkit::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "luck",
            "(F)Lorg/bukkit/loot/LootContext$Builder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::loot::LootContextBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn looted_entity(
        &mut self,
        arg0: crate::bukkit::entity::Entity<'mc>,
    ) -> Result<crate::bukkit::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lootedEntity",
            "(Lorg/bukkit/entity/Entity;)Lorg/bukkit/loot/LootContext$Builder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::loot::LootContextBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn looting_modifier(
        &mut self,
        arg0: i32,
    ) -> Result<crate::bukkit::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lootingModifier",
            "(I)Lorg/bukkit/loot/LootContext$Builder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::loot::LootContextBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn killer(
        &mut self,
        arg0: crate::bukkit::entity::HumanEntity<'mc>,
    ) -> Result<crate::bukkit::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let val_0 = unsafe { jni::objects::JObject::from_raw(arg0.1.clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "killer",
            "(Lorg/bukkit/entity/HumanEntity;)Lorg/bukkit/loot/LootContext$Builder;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let ret = {
            crate::bukkit::loot::LootContextBuilder(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
impl<'mc> crate::JNIRaw<'mc> for LootContext<'mc> {
    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LootContext<'mc> {
    pub fn from_raw(
        env: &crate::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LootContext from null object.").into());
        }
        let cls = env.jni.borrow().get_object_class(&obj)?;
        let name_raw = env.call_method(cls, "getName", "()Ljava/lang/String;", &[])?;
        let oh = name_raw.l()?.into();
        let what = env.get_string(&oh)?;
        let name = what.to_string_lossy();
        if !name.ends_with("LootContext") {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootContext object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn location(&mut self) -> Result<crate::bukkit::Location<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLocation",
            "()Lorg/bukkit/Location;",
            &[],
        )?;
        let ret = {
            crate::bukkit::Location(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn killer(
        &mut self,
    ) -> Result<crate::bukkit::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKiller",
            "()Lorg/bukkit/entity/HumanEntity;",
            &[],
        )?;
        let ret = {
            crate::bukkit::entity::HumanEntity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
    }
    pub fn luck(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLuck", "()F", &[])?;
        Ok(res.f().unwrap())
    }
    pub fn looting_modifier(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootingModifier", "()I", &[])?;
        Ok(res.i().unwrap())
    }
    pub fn looted_entity(
        &mut self,
    ) -> Result<crate::bukkit::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootedEntity",
            "()Lorg/bukkit/entity/Entity;",
            &[],
        )?;
        let ret = {
            crate::bukkit::entity::Entity(self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })
        };
        Ok(ret)
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
