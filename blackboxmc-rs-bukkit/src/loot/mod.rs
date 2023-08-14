#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// LootTables are technical files that represent what items should be in naturally generated containers, what items should be dropped when killing a mob, or what items can be fished. See the <a href="https://minecraft.gamepedia.com/Loot_table"> Minecraft Wiki</a> for more information.
///
/// This is a representation of an abstract class.
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootTable")?;
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
    //

    pub fn populate_loot(
        &mut self,
        arg0: impl Into<blackboxmc_java::JavaRandom<'mc>>,
        arg1: impl Into<crate::loot::LootContext<'mc>>,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/util/Random;Lorg/bukkit/loot/LootContext;)Ljava/util/Collection;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "populateLoot",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    //

    pub fn fill_inventory(
        &mut self,
        arg0: impl Into<crate::inventory::Inventory<'mc>>,
        arg1: impl Into<blackboxmc_java::JavaRandom<'mc>>,
        arg2: impl Into<crate::loot::LootContext<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/inventory/Inventory;Ljava/util/Random;Lorg/bukkit/loot/LootContext;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg1.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg2.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "fillInventory",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn key(&mut self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NamespacedKey;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), vec![]);
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
impl<'mc> Into<crate::Keyed<'mc>> for LootTable<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LootTable into crate::Keyed")
    }
}
#[derive(PartialEq, Eq)]
pub enum LootTablesLootTablesEnum {
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
impl std::fmt::Display for LootTablesLootTablesEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LootTablesLootTablesEnum::Empty => f.write_str("EMPTY"),
            LootTablesLootTablesEnum::AbandonedMineshaft => f.write_str("ABANDONED_MINESHAFT"),
            LootTablesLootTablesEnum::BuriedTreasure => f.write_str("BURIED_TREASURE"),
            LootTablesLootTablesEnum::DesertPyramid => f.write_str("DESERT_PYRAMID"),
            LootTablesLootTablesEnum::EndCityTreasure => f.write_str("END_CITY_TREASURE"),
            LootTablesLootTablesEnum::IglooChest => f.write_str("IGLOO_CHEST"),
            LootTablesLootTablesEnum::JungleTemple => f.write_str("JUNGLE_TEMPLE"),
            LootTablesLootTablesEnum::JungleTempleDispenser => {
                f.write_str("JUNGLE_TEMPLE_DISPENSER")
            }
            LootTablesLootTablesEnum::NetherBridge => f.write_str("NETHER_BRIDGE"),
            LootTablesLootTablesEnum::PillagerOutpost => f.write_str("PILLAGER_OUTPOST"),
            LootTablesLootTablesEnum::BastionTreasure => f.write_str("BASTION_TREASURE"),
            LootTablesLootTablesEnum::BastionOther => f.write_str("BASTION_OTHER"),
            LootTablesLootTablesEnum::BastionBridge => f.write_str("BASTION_BRIDGE"),
            LootTablesLootTablesEnum::BastionHoglinStable => f.write_str("BASTION_HOGLIN_STABLE"),
            LootTablesLootTablesEnum::AncientCity => f.write_str("ANCIENT_CITY"),
            LootTablesLootTablesEnum::AncientCityIceBox => f.write_str("ANCIENT_CITY_ICE_BOX"),
            LootTablesLootTablesEnum::RuinedPortal => f.write_str("RUINED_PORTAL"),
            LootTablesLootTablesEnum::ShipwreckMap => f.write_str("SHIPWRECK_MAP"),
            LootTablesLootTablesEnum::ShipwreckSupply => f.write_str("SHIPWRECK_SUPPLY"),
            LootTablesLootTablesEnum::ShipwreckTreasure => f.write_str("SHIPWRECK_TREASURE"),
            LootTablesLootTablesEnum::SimpleDungeon => f.write_str("SIMPLE_DUNGEON"),
            LootTablesLootTablesEnum::SpawnBonusChest => f.write_str("SPAWN_BONUS_CHEST"),
            LootTablesLootTablesEnum::StrongholdCorridor => f.write_str("STRONGHOLD_CORRIDOR"),
            LootTablesLootTablesEnum::StrongholdCrossing => f.write_str("STRONGHOLD_CROSSING"),
            LootTablesLootTablesEnum::StrongholdLibrary => f.write_str("STRONGHOLD_LIBRARY"),
            LootTablesLootTablesEnum::UnderwaterRuinBig => f.write_str("UNDERWATER_RUIN_BIG"),
            LootTablesLootTablesEnum::UnderwaterRuinSmall => f.write_str("UNDERWATER_RUIN_SMALL"),
            LootTablesLootTablesEnum::VillageArmorer => f.write_str("VILLAGE_ARMORER"),
            LootTablesLootTablesEnum::VillageButcher => f.write_str("VILLAGE_BUTCHER"),
            LootTablesLootTablesEnum::VillageCartographer => f.write_str("VILLAGE_CARTOGRAPHER"),
            LootTablesLootTablesEnum::VillageDesertHouse => f.write_str("VILLAGE_DESERT_HOUSE"),
            LootTablesLootTablesEnum::VillageFisher => f.write_str("VILLAGE_FISHER"),
            LootTablesLootTablesEnum::VillageFletcher => f.write_str("VILLAGE_FLETCHER"),
            LootTablesLootTablesEnum::VillageMason => f.write_str("VILLAGE_MASON"),
            LootTablesLootTablesEnum::VillagePlainsHouse => f.write_str("VILLAGE_PLAINS_HOUSE"),
            LootTablesLootTablesEnum::VillageSavannaHouse => f.write_str("VILLAGE_SAVANNA_HOUSE"),
            LootTablesLootTablesEnum::VillageShepherd => f.write_str("VILLAGE_SHEPHERD"),
            LootTablesLootTablesEnum::VillageSnowyHouse => f.write_str("VILLAGE_SNOWY_HOUSE"),
            LootTablesLootTablesEnum::VillageTaigaHouse => f.write_str("VILLAGE_TAIGA_HOUSE"),
            LootTablesLootTablesEnum::VillageTannery => f.write_str("VILLAGE_TANNERY"),
            LootTablesLootTablesEnum::VillageTemple => f.write_str("VILLAGE_TEMPLE"),
            LootTablesLootTablesEnum::VillageToolsmith => f.write_str("VILLAGE_TOOLSMITH"),
            LootTablesLootTablesEnum::VillageWeaponsmith => f.write_str("VILLAGE_WEAPONSMITH"),
            LootTablesLootTablesEnum::WoodlandMansion => f.write_str("WOODLAND_MANSION"),
            LootTablesLootTablesEnum::ArmorStand => f.write_str("ARMOR_STAND"),
            LootTablesLootTablesEnum::Axolotl => f.write_str("AXOLOTL"),
            LootTablesLootTablesEnum::Bat => f.write_str("BAT"),
            LootTablesLootTablesEnum::Bee => f.write_str("BEE"),
            LootTablesLootTablesEnum::Blaze => f.write_str("BLAZE"),
            LootTablesLootTablesEnum::Cat => f.write_str("CAT"),
            LootTablesLootTablesEnum::CaveSpider => f.write_str("CAVE_SPIDER"),
            LootTablesLootTablesEnum::Chicken => f.write_str("CHICKEN"),
            LootTablesLootTablesEnum::Cod => f.write_str("COD"),
            LootTablesLootTablesEnum::Cow => f.write_str("COW"),
            LootTablesLootTablesEnum::Creeper => f.write_str("CREEPER"),
            LootTablesLootTablesEnum::Dolphin => f.write_str("DOLPHIN"),
            LootTablesLootTablesEnum::Donkey => f.write_str("DONKEY"),
            LootTablesLootTablesEnum::Drowned => f.write_str("DROWNED"),
            LootTablesLootTablesEnum::ElderGuardian => f.write_str("ELDER_GUARDIAN"),
            LootTablesLootTablesEnum::EnderDragon => f.write_str("ENDER_DRAGON"),
            LootTablesLootTablesEnum::Enderman => f.write_str("ENDERMAN"),
            LootTablesLootTablesEnum::Endermite => f.write_str("ENDERMITE"),
            LootTablesLootTablesEnum::Evoker => f.write_str("EVOKER"),
            LootTablesLootTablesEnum::Fox => f.write_str("FOX"),
            LootTablesLootTablesEnum::Ghast => f.write_str("GHAST"),
            LootTablesLootTablesEnum::Giant => f.write_str("GIANT"),
            LootTablesLootTablesEnum::GlowSquid => f.write_str("GLOW_SQUID"),
            LootTablesLootTablesEnum::Goat => f.write_str("GOAT"),
            LootTablesLootTablesEnum::Guardian => f.write_str("GUARDIAN"),
            LootTablesLootTablesEnum::Hoglin => f.write_str("HOGLIN"),
            LootTablesLootTablesEnum::Horse => f.write_str("HORSE"),
            LootTablesLootTablesEnum::Husk => f.write_str("HUSK"),
            LootTablesLootTablesEnum::Illusioner => f.write_str("ILLUSIONER"),
            LootTablesLootTablesEnum::IronGolem => f.write_str("IRON_GOLEM"),
            LootTablesLootTablesEnum::Llama => f.write_str("LLAMA"),
            LootTablesLootTablesEnum::MagmaCube => f.write_str("MAGMA_CUBE"),
            LootTablesLootTablesEnum::Mooshroom => f.write_str("MOOSHROOM"),
            LootTablesLootTablesEnum::Mule => f.write_str("MULE"),
            LootTablesLootTablesEnum::Ocelot => f.write_str("OCELOT"),
            LootTablesLootTablesEnum::Panda => f.write_str("PANDA"),
            LootTablesLootTablesEnum::Parrot => f.write_str("PARROT"),
            LootTablesLootTablesEnum::Phantom => f.write_str("PHANTOM"),
            LootTablesLootTablesEnum::Pig => f.write_str("PIG"),
            LootTablesLootTablesEnum::Piglin => f.write_str("PIGLIN"),
            LootTablesLootTablesEnum::PiglinBrute => f.write_str("PIGLIN_BRUTE"),
            LootTablesLootTablesEnum::Pillager => f.write_str("PILLAGER"),
            LootTablesLootTablesEnum::Player => f.write_str("PLAYER"),
            LootTablesLootTablesEnum::PolarBear => f.write_str("POLAR_BEAR"),
            LootTablesLootTablesEnum::Pufferfish => f.write_str("PUFFERFISH"),
            LootTablesLootTablesEnum::Rabbit => f.write_str("RABBIT"),
            LootTablesLootTablesEnum::Ravager => f.write_str("RAVAGER"),
            LootTablesLootTablesEnum::Salmon => f.write_str("SALMON"),
            LootTablesLootTablesEnum::Shulker => f.write_str("SHULKER"),
            LootTablesLootTablesEnum::Silverfish => f.write_str("SILVERFISH"),
            LootTablesLootTablesEnum::Skeleton => f.write_str("SKELETON"),
            LootTablesLootTablesEnum::SkeletonHorse => f.write_str("SKELETON_HORSE"),
            LootTablesLootTablesEnum::Slime => f.write_str("SLIME"),
            LootTablesLootTablesEnum::SnowGolem => f.write_str("SNOW_GOLEM"),
            LootTablesLootTablesEnum::Spider => f.write_str("SPIDER"),
            LootTablesLootTablesEnum::Squid => f.write_str("SQUID"),
            LootTablesLootTablesEnum::Stray => f.write_str("STRAY"),
            LootTablesLootTablesEnum::Strider => f.write_str("STRIDER"),
            LootTablesLootTablesEnum::TraderLlama => f.write_str("TRADER_LLAMA"),
            LootTablesLootTablesEnum::TropicalFish => f.write_str("TROPICAL_FISH"),
            LootTablesLootTablesEnum::Turtle => f.write_str("TURTLE"),
            LootTablesLootTablesEnum::Vex => f.write_str("VEX"),
            LootTablesLootTablesEnum::Villager => f.write_str("VILLAGER"),
            LootTablesLootTablesEnum::Vindicator => f.write_str("VINDICATOR"),
            LootTablesLootTablesEnum::WanderingTrader => f.write_str("WANDERING_TRADER"),
            LootTablesLootTablesEnum::Witch => f.write_str("WITCH"),
            LootTablesLootTablesEnum::Wither => f.write_str("WITHER"),
            LootTablesLootTablesEnum::WitherSkeleton => f.write_str("WITHER_SKELETON"),
            LootTablesLootTablesEnum::Wolf => f.write_str("WOLF"),
            LootTablesLootTablesEnum::Zoglin => f.write_str("ZOGLIN"),
            LootTablesLootTablesEnum::Zombie => f.write_str("ZOMBIE"),
            LootTablesLootTablesEnum::ZombieHorse => f.write_str("ZOMBIE_HORSE"),
            LootTablesLootTablesEnum::ZombieVillager => f.write_str("ZOMBIE_VILLAGER"),
            LootTablesLootTablesEnum::ZombifiedPiglin => f.write_str("ZOMBIFIED_PIGLIN"),
            LootTablesLootTablesEnum::ArmorerGift => f.write_str("ARMORER_GIFT"),
            LootTablesLootTablesEnum::ButcherGift => f.write_str("BUTCHER_GIFT"),
            LootTablesLootTablesEnum::CartographerGift => f.write_str("CARTOGRAPHER_GIFT"),
            LootTablesLootTablesEnum::CatMorningGift => f.write_str("CAT_MORNING_GIFT"),
            LootTablesLootTablesEnum::ClericGift => f.write_str("CLERIC_GIFT"),
            LootTablesLootTablesEnum::FarmerGift => f.write_str("FARMER_GIFT"),
            LootTablesLootTablesEnum::FishermanGift => f.write_str("FISHERMAN_GIFT"),
            LootTablesLootTablesEnum::Fishing => f.write_str("FISHING"),
            LootTablesLootTablesEnum::FishingFish => f.write_str("FISHING_FISH"),
            LootTablesLootTablesEnum::FishingJunk => f.write_str("FISHING_JUNK"),
            LootTablesLootTablesEnum::FishingTreasure => f.write_str("FISHING_TREASURE"),
            LootTablesLootTablesEnum::FletcherGift => f.write_str("FLETCHER_GIFT"),
            LootTablesLootTablesEnum::LeatherworkerGift => f.write_str("LEATHERWORKER_GIFT"),
            LootTablesLootTablesEnum::LibrarianGift => f.write_str("LIBRARIAN_GIFT"),
            LootTablesLootTablesEnum::MasonGift => f.write_str("MASON_GIFT"),
            LootTablesLootTablesEnum::ShepherdGift => f.write_str("SHEPHERD_GIFT"),
            LootTablesLootTablesEnum::ToolsmithGift => f.write_str("TOOLSMITH_GIFT"),
            LootTablesLootTablesEnum::WeaponsmithGift => f.write_str("WEAPONSMITH_GIFT"),
            LootTablesLootTablesEnum::SnifferDigging => f.write_str("SNIFFER_DIGGING"),
            LootTablesLootTablesEnum::PiglinBartering => f.write_str("PIGLIN_BARTERING"),
            LootTablesLootTablesEnum::DesertWellArchaeology => {
                f.write_str("DESERT_WELL_ARCHAEOLOGY")
            }
            LootTablesLootTablesEnum::DesertPyramidArchaeology => {
                f.write_str("DESERT_PYRAMID_ARCHAEOLOGY")
            }
            LootTablesLootTablesEnum::TrailRuinsArchaeologyCommon => {
                f.write_str("TRAIL_RUINS_ARCHAEOLOGY_COMMON")
            }
            LootTablesLootTablesEnum::TrailRuinsArchaeologyRare => {
                f.write_str("TRAIL_RUINS_ARCHAEOLOGY_RARE")
            }
            LootTablesLootTablesEnum::OceanRuinWarmArchaeology => {
                f.write_str("OCEAN_RUIN_WARM_ARCHAEOLOGY")
            }
            LootTablesLootTablesEnum::OceanRuinColdArchaeology => {
                f.write_str("OCEAN_RUIN_COLD_ARCHAEOLOGY")
            }
            LootTablesLootTablesEnum::Sheep => f.write_str("SHEEP"),
            LootTablesLootTablesEnum::SheepBlack => f.write_str("SHEEP_BLACK"),
            LootTablesLootTablesEnum::SheepBlue => f.write_str("SHEEP_BLUE"),
            LootTablesLootTablesEnum::SheepBrown => f.write_str("SHEEP_BROWN"),
            LootTablesLootTablesEnum::SheepCyan => f.write_str("SHEEP_CYAN"),
            LootTablesLootTablesEnum::SheepGray => f.write_str("SHEEP_GRAY"),
            LootTablesLootTablesEnum::SheepGreen => f.write_str("SHEEP_GREEN"),
            LootTablesLootTablesEnum::SheepLightBlue => f.write_str("SHEEP_LIGHT_BLUE"),
            LootTablesLootTablesEnum::SheepLightGray => f.write_str("SHEEP_LIGHT_GRAY"),
            LootTablesLootTablesEnum::SheepLime => f.write_str("SHEEP_LIME"),
            LootTablesLootTablesEnum::SheepMagenta => f.write_str("SHEEP_MAGENTA"),
            LootTablesLootTablesEnum::SheepOrange => f.write_str("SHEEP_ORANGE"),
            LootTablesLootTablesEnum::SheepPink => f.write_str("SHEEP_PINK"),
            LootTablesLootTablesEnum::SheepPurple => f.write_str("SHEEP_PURPLE"),
            LootTablesLootTablesEnum::SheepRed => f.write_str("SHEEP_RED"),
            LootTablesLootTablesEnum::SheepWhite => f.write_str("SHEEP_WHITE"),
            LootTablesLootTablesEnum::SheepYellow => f.write_str("SHEEP_YELLOW"),
        }
    }
}
pub struct LootTablesLootTables<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub LootTablesLootTablesEnum,
);
impl<'mc> std::ops::Deref for LootTablesLootTables<'mc> {
    type Target = LootTablesLootTablesEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for LootTablesLootTables<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> LootTablesLootTables<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: LootTablesLootTablesEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LootTablesLootTables from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootTables$LootTables")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootTablesLootTables object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const EMPTY: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Empty;
    pub const ABANDONED_MINESHAFT: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::AbandonedMineshaft;
    pub const BURIED_TREASURE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::BuriedTreasure;
    pub const DESERT_PYRAMID: LootTablesLootTablesEnum = LootTablesLootTablesEnum::DesertPyramid;
    pub const END_CITY_TREASURE: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::EndCityTreasure;
    pub const IGLOO_CHEST: LootTablesLootTablesEnum = LootTablesLootTablesEnum::IglooChest;
    pub const JUNGLE_TEMPLE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::JungleTemple;
    pub const JUNGLE_TEMPLE_DISPENSER: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::JungleTempleDispenser;
    pub const NETHER_BRIDGE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::NetherBridge;
    pub const PILLAGER_OUTPOST: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::PillagerOutpost;
    pub const BASTION_TREASURE: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::BastionTreasure;
    pub const BASTION_OTHER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::BastionOther;
    pub const BASTION_BRIDGE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::BastionBridge;
    pub const BASTION_HOGLIN_STABLE: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::BastionHoglinStable;
    pub const ANCIENT_CITY: LootTablesLootTablesEnum = LootTablesLootTablesEnum::AncientCity;
    pub const ANCIENT_CITY_ICE_BOX: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::AncientCityIceBox;
    pub const RUINED_PORTAL: LootTablesLootTablesEnum = LootTablesLootTablesEnum::RuinedPortal;
    pub const SHIPWRECK_MAP: LootTablesLootTablesEnum = LootTablesLootTablesEnum::ShipwreckMap;
    pub const SHIPWRECK_SUPPLY: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::ShipwreckSupply;
    pub const SHIPWRECK_TREASURE: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::ShipwreckTreasure;
    pub const SIMPLE_DUNGEON: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SimpleDungeon;
    pub const SPAWN_BONUS_CHEST: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::SpawnBonusChest;
    pub const STRONGHOLD_CORRIDOR: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::StrongholdCorridor;
    pub const STRONGHOLD_CROSSING: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::StrongholdCrossing;
    pub const STRONGHOLD_LIBRARY: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::StrongholdLibrary;
    pub const UNDERWATER_RUIN_BIG: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::UnderwaterRuinBig;
    pub const UNDERWATER_RUIN_SMALL: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::UnderwaterRuinSmall;
    pub const VILLAGE_ARMORER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::VillageArmorer;
    pub const VILLAGE_BUTCHER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::VillageButcher;
    pub const VILLAGE_CARTOGRAPHER: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::VillageCartographer;
    pub const VILLAGE_DESERT_HOUSE: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::VillageDesertHouse;
    pub const VILLAGE_FISHER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::VillageFisher;
    pub const VILLAGE_FLETCHER: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::VillageFletcher;
    pub const VILLAGE_MASON: LootTablesLootTablesEnum = LootTablesLootTablesEnum::VillageMason;
    pub const VILLAGE_PLAINS_HOUSE: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::VillagePlainsHouse;
    pub const VILLAGE_SAVANNA_HOUSE: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::VillageSavannaHouse;
    pub const VILLAGE_SHEPHERD: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::VillageShepherd;
    pub const VILLAGE_SNOWY_HOUSE: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::VillageSnowyHouse;
    pub const VILLAGE_TAIGA_HOUSE: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::VillageTaigaHouse;
    pub const VILLAGE_TANNERY: LootTablesLootTablesEnum = LootTablesLootTablesEnum::VillageTannery;
    pub const VILLAGE_TEMPLE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::VillageTemple;
    pub const VILLAGE_TOOLSMITH: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::VillageToolsmith;
    pub const VILLAGE_WEAPONSMITH: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::VillageWeaponsmith;
    pub const WOODLAND_MANSION: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::WoodlandMansion;
    pub const ARMOR_STAND: LootTablesLootTablesEnum = LootTablesLootTablesEnum::ArmorStand;
    pub const AXOLOTL: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Axolotl;
    pub const BAT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Bat;
    pub const BEE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Bee;
    pub const BLAZE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Blaze;
    pub const CAT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Cat;
    pub const CAVE_SPIDER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::CaveSpider;
    pub const CHICKEN: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Chicken;
    pub const COD: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Cod;
    pub const COW: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Cow;
    pub const CREEPER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Creeper;
    pub const DOLPHIN: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Dolphin;
    pub const DONKEY: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Donkey;
    pub const DROWNED: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Drowned;
    pub const ELDER_GUARDIAN: LootTablesLootTablesEnum = LootTablesLootTablesEnum::ElderGuardian;
    pub const ENDER_DRAGON: LootTablesLootTablesEnum = LootTablesLootTablesEnum::EnderDragon;
    pub const ENDERMAN: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Enderman;
    pub const ENDERMITE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Endermite;
    pub const EVOKER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Evoker;
    pub const FOX: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Fox;
    pub const GHAST: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Ghast;
    pub const GIANT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Giant;
    pub const GLOW_SQUID: LootTablesLootTablesEnum = LootTablesLootTablesEnum::GlowSquid;
    pub const GOAT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Goat;
    pub const GUARDIAN: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Guardian;
    pub const HOGLIN: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Hoglin;
    pub const HORSE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Horse;
    pub const HUSK: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Husk;
    pub const ILLUSIONER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Illusioner;
    pub const IRON_GOLEM: LootTablesLootTablesEnum = LootTablesLootTablesEnum::IronGolem;
    pub const LLAMA: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Llama;
    pub const MAGMA_CUBE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::MagmaCube;
    pub const MOOSHROOM: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Mooshroom;
    pub const MULE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Mule;
    pub const OCELOT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Ocelot;
    pub const PANDA: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Panda;
    pub const PARROT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Parrot;
    pub const PHANTOM: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Phantom;
    pub const PIG: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Pig;
    pub const PIGLIN: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Piglin;
    pub const PIGLIN_BRUTE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::PiglinBrute;
    pub const PILLAGER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Pillager;
    pub const PLAYER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Player;
    pub const POLAR_BEAR: LootTablesLootTablesEnum = LootTablesLootTablesEnum::PolarBear;
    pub const PUFFERFISH: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Pufferfish;
    pub const RABBIT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Rabbit;
    pub const RAVAGER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Ravager;
    pub const SALMON: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Salmon;
    pub const SHULKER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Shulker;
    pub const SILVERFISH: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Silverfish;
    pub const SKELETON: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Skeleton;
    pub const SKELETON_HORSE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SkeletonHorse;
    pub const SLIME: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Slime;
    pub const SNOW_GOLEM: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SnowGolem;
    pub const SPIDER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Spider;
    pub const SQUID: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Squid;
    pub const STRAY: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Stray;
    pub const STRIDER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Strider;
    pub const TRADER_LLAMA: LootTablesLootTablesEnum = LootTablesLootTablesEnum::TraderLlama;
    pub const TROPICAL_FISH: LootTablesLootTablesEnum = LootTablesLootTablesEnum::TropicalFish;
    pub const TURTLE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Turtle;
    pub const VEX: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Vex;
    pub const VILLAGER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Villager;
    pub const VINDICATOR: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Vindicator;
    pub const WANDERING_TRADER: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::WanderingTrader;
    pub const WITCH: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Witch;
    pub const WITHER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Wither;
    pub const WITHER_SKELETON: LootTablesLootTablesEnum = LootTablesLootTablesEnum::WitherSkeleton;
    pub const WOLF: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Wolf;
    pub const ZOGLIN: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Zoglin;
    pub const ZOMBIE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Zombie;
    pub const ZOMBIE_HORSE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::ZombieHorse;
    pub const ZOMBIE_VILLAGER: LootTablesLootTablesEnum = LootTablesLootTablesEnum::ZombieVillager;
    pub const ZOMBIFIED_PIGLIN: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::ZombifiedPiglin;
    pub const ARMORER_GIFT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::ArmorerGift;
    pub const BUTCHER_GIFT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::ButcherGift;
    pub const CARTOGRAPHER_GIFT: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::CartographerGift;
    pub const CAT_MORNING_GIFT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::CatMorningGift;
    pub const CLERIC_GIFT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::ClericGift;
    pub const FARMER_GIFT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::FarmerGift;
    pub const FISHERMAN_GIFT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::FishermanGift;
    pub const FISHING: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Fishing;
    pub const FISHING_FISH: LootTablesLootTablesEnum = LootTablesLootTablesEnum::FishingFish;
    pub const FISHING_JUNK: LootTablesLootTablesEnum = LootTablesLootTablesEnum::FishingJunk;
    pub const FISHING_TREASURE: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::FishingTreasure;
    pub const FLETCHER_GIFT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::FletcherGift;
    pub const LEATHERWORKER_GIFT: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::LeatherworkerGift;
    pub const LIBRARIAN_GIFT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::LibrarianGift;
    pub const MASON_GIFT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::MasonGift;
    pub const SHEPHERD_GIFT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::ShepherdGift;
    pub const TOOLSMITH_GIFT: LootTablesLootTablesEnum = LootTablesLootTablesEnum::ToolsmithGift;
    pub const WEAPONSMITH_GIFT: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::WeaponsmithGift;
    pub const SNIFFER_DIGGING: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SnifferDigging;
    pub const PIGLIN_BARTERING: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::PiglinBartering;
    pub const DESERT_WELL_ARCHAEOLOGY: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::DesertWellArchaeology;
    pub const DESERT_PYRAMID_ARCHAEOLOGY: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::DesertPyramidArchaeology;
    pub const TRAIL_RUINS_ARCHAEOLOGY_COMMON: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::TrailRuinsArchaeologyCommon;
    pub const TRAIL_RUINS_ARCHAEOLOGY_RARE: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::TrailRuinsArchaeologyRare;
    pub const OCEAN_RUIN_WARM_ARCHAEOLOGY: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::OceanRuinWarmArchaeology;
    pub const OCEAN_RUIN_COLD_ARCHAEOLOGY: LootTablesLootTablesEnum =
        LootTablesLootTablesEnum::OceanRuinColdArchaeology;
    pub const SHEEP: LootTablesLootTablesEnum = LootTablesLootTablesEnum::Sheep;
    pub const SHEEP_BLACK: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepBlack;
    pub const SHEEP_BLUE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepBlue;
    pub const SHEEP_BROWN: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepBrown;
    pub const SHEEP_CYAN: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepCyan;
    pub const SHEEP_GRAY: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepGray;
    pub const SHEEP_GREEN: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepGreen;
    pub const SHEEP_LIGHT_BLUE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepLightBlue;
    pub const SHEEP_LIGHT_GRAY: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepLightGray;
    pub const SHEEP_LIME: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepLime;
    pub const SHEEP_MAGENTA: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepMagenta;
    pub const SHEEP_ORANGE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepOrange;
    pub const SHEEP_PINK: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepPink;
    pub const SHEEP_PURPLE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepPurple;
    pub const SHEEP_RED: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepRed;
    pub const SHEEP_WHITE: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepWhite;
    pub const SHEEP_YELLOW: LootTablesLootTablesEnum = LootTablesLootTablesEnum::SheepYellow;
    pub fn from_string(str: String) -> std::option::Option<LootTablesLootTablesEnum> {
        match str.as_str() {
            "EMPTY" => Some(LootTablesLootTablesEnum::Empty),
            "ABANDONED_MINESHAFT" => Some(LootTablesLootTablesEnum::AbandonedMineshaft),
            "BURIED_TREASURE" => Some(LootTablesLootTablesEnum::BuriedTreasure),
            "DESERT_PYRAMID" => Some(LootTablesLootTablesEnum::DesertPyramid),
            "END_CITY_TREASURE" => Some(LootTablesLootTablesEnum::EndCityTreasure),
            "IGLOO_CHEST" => Some(LootTablesLootTablesEnum::IglooChest),
            "JUNGLE_TEMPLE" => Some(LootTablesLootTablesEnum::JungleTemple),
            "JUNGLE_TEMPLE_DISPENSER" => Some(LootTablesLootTablesEnum::JungleTempleDispenser),
            "NETHER_BRIDGE" => Some(LootTablesLootTablesEnum::NetherBridge),
            "PILLAGER_OUTPOST" => Some(LootTablesLootTablesEnum::PillagerOutpost),
            "BASTION_TREASURE" => Some(LootTablesLootTablesEnum::BastionTreasure),
            "BASTION_OTHER" => Some(LootTablesLootTablesEnum::BastionOther),
            "BASTION_BRIDGE" => Some(LootTablesLootTablesEnum::BastionBridge),
            "BASTION_HOGLIN_STABLE" => Some(LootTablesLootTablesEnum::BastionHoglinStable),
            "ANCIENT_CITY" => Some(LootTablesLootTablesEnum::AncientCity),
            "ANCIENT_CITY_ICE_BOX" => Some(LootTablesLootTablesEnum::AncientCityIceBox),
            "RUINED_PORTAL" => Some(LootTablesLootTablesEnum::RuinedPortal),
            "SHIPWRECK_MAP" => Some(LootTablesLootTablesEnum::ShipwreckMap),
            "SHIPWRECK_SUPPLY" => Some(LootTablesLootTablesEnum::ShipwreckSupply),
            "SHIPWRECK_TREASURE" => Some(LootTablesLootTablesEnum::ShipwreckTreasure),
            "SIMPLE_DUNGEON" => Some(LootTablesLootTablesEnum::SimpleDungeon),
            "SPAWN_BONUS_CHEST" => Some(LootTablesLootTablesEnum::SpawnBonusChest),
            "STRONGHOLD_CORRIDOR" => Some(LootTablesLootTablesEnum::StrongholdCorridor),
            "STRONGHOLD_CROSSING" => Some(LootTablesLootTablesEnum::StrongholdCrossing),
            "STRONGHOLD_LIBRARY" => Some(LootTablesLootTablesEnum::StrongholdLibrary),
            "UNDERWATER_RUIN_BIG" => Some(LootTablesLootTablesEnum::UnderwaterRuinBig),
            "UNDERWATER_RUIN_SMALL" => Some(LootTablesLootTablesEnum::UnderwaterRuinSmall),
            "VILLAGE_ARMORER" => Some(LootTablesLootTablesEnum::VillageArmorer),
            "VILLAGE_BUTCHER" => Some(LootTablesLootTablesEnum::VillageButcher),
            "VILLAGE_CARTOGRAPHER" => Some(LootTablesLootTablesEnum::VillageCartographer),
            "VILLAGE_DESERT_HOUSE" => Some(LootTablesLootTablesEnum::VillageDesertHouse),
            "VILLAGE_FISHER" => Some(LootTablesLootTablesEnum::VillageFisher),
            "VILLAGE_FLETCHER" => Some(LootTablesLootTablesEnum::VillageFletcher),
            "VILLAGE_MASON" => Some(LootTablesLootTablesEnum::VillageMason),
            "VILLAGE_PLAINS_HOUSE" => Some(LootTablesLootTablesEnum::VillagePlainsHouse),
            "VILLAGE_SAVANNA_HOUSE" => Some(LootTablesLootTablesEnum::VillageSavannaHouse),
            "VILLAGE_SHEPHERD" => Some(LootTablesLootTablesEnum::VillageShepherd),
            "VILLAGE_SNOWY_HOUSE" => Some(LootTablesLootTablesEnum::VillageSnowyHouse),
            "VILLAGE_TAIGA_HOUSE" => Some(LootTablesLootTablesEnum::VillageTaigaHouse),
            "VILLAGE_TANNERY" => Some(LootTablesLootTablesEnum::VillageTannery),
            "VILLAGE_TEMPLE" => Some(LootTablesLootTablesEnum::VillageTemple),
            "VILLAGE_TOOLSMITH" => Some(LootTablesLootTablesEnum::VillageToolsmith),
            "VILLAGE_WEAPONSMITH" => Some(LootTablesLootTablesEnum::VillageWeaponsmith),
            "WOODLAND_MANSION" => Some(LootTablesLootTablesEnum::WoodlandMansion),
            "ARMOR_STAND" => Some(LootTablesLootTablesEnum::ArmorStand),
            "AXOLOTL" => Some(LootTablesLootTablesEnum::Axolotl),
            "BAT" => Some(LootTablesLootTablesEnum::Bat),
            "BEE" => Some(LootTablesLootTablesEnum::Bee),
            "BLAZE" => Some(LootTablesLootTablesEnum::Blaze),
            "CAT" => Some(LootTablesLootTablesEnum::Cat),
            "CAVE_SPIDER" => Some(LootTablesLootTablesEnum::CaveSpider),
            "CHICKEN" => Some(LootTablesLootTablesEnum::Chicken),
            "COD" => Some(LootTablesLootTablesEnum::Cod),
            "COW" => Some(LootTablesLootTablesEnum::Cow),
            "CREEPER" => Some(LootTablesLootTablesEnum::Creeper),
            "DOLPHIN" => Some(LootTablesLootTablesEnum::Dolphin),
            "DONKEY" => Some(LootTablesLootTablesEnum::Donkey),
            "DROWNED" => Some(LootTablesLootTablesEnum::Drowned),
            "ELDER_GUARDIAN" => Some(LootTablesLootTablesEnum::ElderGuardian),
            "ENDER_DRAGON" => Some(LootTablesLootTablesEnum::EnderDragon),
            "ENDERMAN" => Some(LootTablesLootTablesEnum::Enderman),
            "ENDERMITE" => Some(LootTablesLootTablesEnum::Endermite),
            "EVOKER" => Some(LootTablesLootTablesEnum::Evoker),
            "FOX" => Some(LootTablesLootTablesEnum::Fox),
            "GHAST" => Some(LootTablesLootTablesEnum::Ghast),
            "GIANT" => Some(LootTablesLootTablesEnum::Giant),
            "GLOW_SQUID" => Some(LootTablesLootTablesEnum::GlowSquid),
            "GOAT" => Some(LootTablesLootTablesEnum::Goat),
            "GUARDIAN" => Some(LootTablesLootTablesEnum::Guardian),
            "HOGLIN" => Some(LootTablesLootTablesEnum::Hoglin),
            "HORSE" => Some(LootTablesLootTablesEnum::Horse),
            "HUSK" => Some(LootTablesLootTablesEnum::Husk),
            "ILLUSIONER" => Some(LootTablesLootTablesEnum::Illusioner),
            "IRON_GOLEM" => Some(LootTablesLootTablesEnum::IronGolem),
            "LLAMA" => Some(LootTablesLootTablesEnum::Llama),
            "MAGMA_CUBE" => Some(LootTablesLootTablesEnum::MagmaCube),
            "MOOSHROOM" => Some(LootTablesLootTablesEnum::Mooshroom),
            "MULE" => Some(LootTablesLootTablesEnum::Mule),
            "OCELOT" => Some(LootTablesLootTablesEnum::Ocelot),
            "PANDA" => Some(LootTablesLootTablesEnum::Panda),
            "PARROT" => Some(LootTablesLootTablesEnum::Parrot),
            "PHANTOM" => Some(LootTablesLootTablesEnum::Phantom),
            "PIG" => Some(LootTablesLootTablesEnum::Pig),
            "PIGLIN" => Some(LootTablesLootTablesEnum::Piglin),
            "PIGLIN_BRUTE" => Some(LootTablesLootTablesEnum::PiglinBrute),
            "PILLAGER" => Some(LootTablesLootTablesEnum::Pillager),
            "PLAYER" => Some(LootTablesLootTablesEnum::Player),
            "POLAR_BEAR" => Some(LootTablesLootTablesEnum::PolarBear),
            "PUFFERFISH" => Some(LootTablesLootTablesEnum::Pufferfish),
            "RABBIT" => Some(LootTablesLootTablesEnum::Rabbit),
            "RAVAGER" => Some(LootTablesLootTablesEnum::Ravager),
            "SALMON" => Some(LootTablesLootTablesEnum::Salmon),
            "SHULKER" => Some(LootTablesLootTablesEnum::Shulker),
            "SILVERFISH" => Some(LootTablesLootTablesEnum::Silverfish),
            "SKELETON" => Some(LootTablesLootTablesEnum::Skeleton),
            "SKELETON_HORSE" => Some(LootTablesLootTablesEnum::SkeletonHorse),
            "SLIME" => Some(LootTablesLootTablesEnum::Slime),
            "SNOW_GOLEM" => Some(LootTablesLootTablesEnum::SnowGolem),
            "SPIDER" => Some(LootTablesLootTablesEnum::Spider),
            "SQUID" => Some(LootTablesLootTablesEnum::Squid),
            "STRAY" => Some(LootTablesLootTablesEnum::Stray),
            "STRIDER" => Some(LootTablesLootTablesEnum::Strider),
            "TRADER_LLAMA" => Some(LootTablesLootTablesEnum::TraderLlama),
            "TROPICAL_FISH" => Some(LootTablesLootTablesEnum::TropicalFish),
            "TURTLE" => Some(LootTablesLootTablesEnum::Turtle),
            "VEX" => Some(LootTablesLootTablesEnum::Vex),
            "VILLAGER" => Some(LootTablesLootTablesEnum::Villager),
            "VINDICATOR" => Some(LootTablesLootTablesEnum::Vindicator),
            "WANDERING_TRADER" => Some(LootTablesLootTablesEnum::WanderingTrader),
            "WITCH" => Some(LootTablesLootTablesEnum::Witch),
            "WITHER" => Some(LootTablesLootTablesEnum::Wither),
            "WITHER_SKELETON" => Some(LootTablesLootTablesEnum::WitherSkeleton),
            "WOLF" => Some(LootTablesLootTablesEnum::Wolf),
            "ZOGLIN" => Some(LootTablesLootTablesEnum::Zoglin),
            "ZOMBIE" => Some(LootTablesLootTablesEnum::Zombie),
            "ZOMBIE_HORSE" => Some(LootTablesLootTablesEnum::ZombieHorse),
            "ZOMBIE_VILLAGER" => Some(LootTablesLootTablesEnum::ZombieVillager),
            "ZOMBIFIED_PIGLIN" => Some(LootTablesLootTablesEnum::ZombifiedPiglin),
            "ARMORER_GIFT" => Some(LootTablesLootTablesEnum::ArmorerGift),
            "BUTCHER_GIFT" => Some(LootTablesLootTablesEnum::ButcherGift),
            "CARTOGRAPHER_GIFT" => Some(LootTablesLootTablesEnum::CartographerGift),
            "CAT_MORNING_GIFT" => Some(LootTablesLootTablesEnum::CatMorningGift),
            "CLERIC_GIFT" => Some(LootTablesLootTablesEnum::ClericGift),
            "FARMER_GIFT" => Some(LootTablesLootTablesEnum::FarmerGift),
            "FISHERMAN_GIFT" => Some(LootTablesLootTablesEnum::FishermanGift),
            "FISHING" => Some(LootTablesLootTablesEnum::Fishing),
            "FISHING_FISH" => Some(LootTablesLootTablesEnum::FishingFish),
            "FISHING_JUNK" => Some(LootTablesLootTablesEnum::FishingJunk),
            "FISHING_TREASURE" => Some(LootTablesLootTablesEnum::FishingTreasure),
            "FLETCHER_GIFT" => Some(LootTablesLootTablesEnum::FletcherGift),
            "LEATHERWORKER_GIFT" => Some(LootTablesLootTablesEnum::LeatherworkerGift),
            "LIBRARIAN_GIFT" => Some(LootTablesLootTablesEnum::LibrarianGift),
            "MASON_GIFT" => Some(LootTablesLootTablesEnum::MasonGift),
            "SHEPHERD_GIFT" => Some(LootTablesLootTablesEnum::ShepherdGift),
            "TOOLSMITH_GIFT" => Some(LootTablesLootTablesEnum::ToolsmithGift),
            "WEAPONSMITH_GIFT" => Some(LootTablesLootTablesEnum::WeaponsmithGift),
            "SNIFFER_DIGGING" => Some(LootTablesLootTablesEnum::SnifferDigging),
            "PIGLIN_BARTERING" => Some(LootTablesLootTablesEnum::PiglinBartering),
            "DESERT_WELL_ARCHAEOLOGY" => Some(LootTablesLootTablesEnum::DesertWellArchaeology),
            "DESERT_PYRAMID_ARCHAEOLOGY" => {
                Some(LootTablesLootTablesEnum::DesertPyramidArchaeology)
            }
            "TRAIL_RUINS_ARCHAEOLOGY_COMMON" => {
                Some(LootTablesLootTablesEnum::TrailRuinsArchaeologyCommon)
            }
            "TRAIL_RUINS_ARCHAEOLOGY_RARE" => {
                Some(LootTablesLootTablesEnum::TrailRuinsArchaeologyRare)
            }
            "OCEAN_RUIN_WARM_ARCHAEOLOGY" => {
                Some(LootTablesLootTablesEnum::OceanRuinWarmArchaeology)
            }
            "OCEAN_RUIN_COLD_ARCHAEOLOGY" => {
                Some(LootTablesLootTablesEnum::OceanRuinColdArchaeology)
            }
            "SHEEP" => Some(LootTablesLootTablesEnum::Sheep),
            "SHEEP_BLACK" => Some(LootTablesLootTablesEnum::SheepBlack),
            "SHEEP_BLUE" => Some(LootTablesLootTablesEnum::SheepBlue),
            "SHEEP_BROWN" => Some(LootTablesLootTablesEnum::SheepBrown),
            "SHEEP_CYAN" => Some(LootTablesLootTablesEnum::SheepCyan),
            "SHEEP_GRAY" => Some(LootTablesLootTablesEnum::SheepGray),
            "SHEEP_GREEN" => Some(LootTablesLootTablesEnum::SheepGreen),
            "SHEEP_LIGHT_BLUE" => Some(LootTablesLootTablesEnum::SheepLightBlue),
            "SHEEP_LIGHT_GRAY" => Some(LootTablesLootTablesEnum::SheepLightGray),
            "SHEEP_LIME" => Some(LootTablesLootTablesEnum::SheepLime),
            "SHEEP_MAGENTA" => Some(LootTablesLootTablesEnum::SheepMagenta),
            "SHEEP_ORANGE" => Some(LootTablesLootTablesEnum::SheepOrange),
            "SHEEP_PINK" => Some(LootTablesLootTablesEnum::SheepPink),
            "SHEEP_PURPLE" => Some(LootTablesLootTablesEnum::SheepPurple),
            "SHEEP_RED" => Some(LootTablesLootTablesEnum::SheepRed),
            "SHEEP_WHITE" => Some(LootTablesLootTablesEnum::SheepWhite),
            "SHEEP_YELLOW" => Some(LootTablesLootTablesEnum::SheepYellow),
            _ => None,
        }
    }

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<LootTablesLootTables<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/loot/LootTables$LootTables");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/loot/LootTables$LootTables;",
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
        LootTablesLootTables::from_raw(
            &jni,
            raw_obj,
            LootTablesLootTables::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Represents a <a href="../block/Container.html" title="interface in org.bukkit.block"><code>Container</code></a> or a <a href="../entity/Mob.html" title="interface in org.bukkit.entity"><code>Mob</code></a> that can have a loot table.
///
/// Container loot will only generate upon opening, and only when the container is <i>first</i> opened.
///
/// Entities will only generate loot upon death.
///
/// This is a representation of an abstract class.
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/Lootable")?;
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
    //

    pub fn seed(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    //

    /// Set the seed used when this Loot Table generates loot.
    pub fn set_seed(&mut self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn set_loot_table(
        &mut self,
        arg0: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/loot/LootTable;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    //

    pub fn loot_table(
        &mut self,
    ) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootTable;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootTable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
#[derive(PartialEq, Eq)]
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
        match self {
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootTables")?;
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

    pub fn value_of(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<LootTables<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into())?);
        let cls = jni.find_class("org/bukkit/loot/LootTables");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/loot/LootTables;",
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
        LootTables::from_raw(
            &jni,
            raw_obj,
            LootTables::from_string(variant_str)
                .ok_or(eyre::eyre!("String gaven for variant was invalid"))?,
        )
    }
}
/// Represents additional information a <a title="interface in org.bukkit.loot" href="LootTable.html"><code>LootTable</code></a> can use to modify it's generated loot.
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootContext$Builder")?;
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
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<crate::Location<'mc>>,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/loot/LootContext$Builder");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::loot::LootContextBuilder::from_raw(&jni, res)
    }
    //

    pub fn build(&mut self) -> Result<crate::loot::LootContext<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootContext;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "build", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContext::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //@NotNull

    pub fn luck(
        &mut self,
        arg0: f32,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(F)Lorg/bukkit/loot/LootContext$Builder;");
        let val_1 = jni::objects::JValueGen::Float(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "luck",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn killer(
        &mut self,
        arg0: impl Into<crate::entity::HumanEntity<'mc>>,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/HumanEntity;)Lorg/bukkit/loot/LootContext$Builder;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "killer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //@NotNull

    pub fn looting_modifier(
        &mut self,
        arg0: i32,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/loot/LootContext$Builder;");
        let val_1 = jni::objects::JValueGen::Int(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lootingModifier",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn looted_entity(
        &mut self,
        arg0: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Lorg/bukkit/loot/LootContext$Builder;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(arg0.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lootedEntity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootContext")?;
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
    //

    pub fn killer(
        &mut self,
    ) -> Result<crate::entity::HumanEntity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKiller", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::HumanEntity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn luck(&mut self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLuck", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    //

    pub fn looting_modifier(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootingModifier",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    //

    pub fn looted_entity(
        &mut self,
    ) -> Result<crate::entity::Entity<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootedEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::entity::Entity::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    //

    pub fn location(&mut self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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
