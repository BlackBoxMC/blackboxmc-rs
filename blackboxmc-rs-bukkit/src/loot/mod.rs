#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// LootTables are technical files that represent what items should be in naturally generated containers, what items should be dropped when killing a mob, or what items can be fished. See the <a href="https://minecraft.gamepedia.com/Loot_table"> Minecraft Wiki</a> for more information.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct LootTable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LootTable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootTable<'mc> {
    fn from_raw(
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
}

impl<'mc> LootTable<'mc> {
    pub fn populate_loot(
        &self,
        arg0: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
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
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }

    pub fn fill_inventory(
        &self,
        arg0: impl Into<crate::inventory::Inventory<'mc>>,
        arg1: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for LootTable<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LootTable into crate::Keyed")
    }
}
pub enum LootTablesLootTables<'mc> {
    Empty {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    AbandonedMineshaft {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    BuriedTreasure {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    DesertPyramid {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    EndCityTreasure {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    IglooChest {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    JungleTemple {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    JungleTempleDispenser {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    NetherBridge {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    PillagerOutpost {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    BastionTreasure {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    BastionOther {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    BastionBridge {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    BastionHoglinStable {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    AncientCity {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    AncientCityIceBox {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    RuinedPortal {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ShipwreckMap {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ShipwreckSupply {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ShipwreckTreasure {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SimpleDungeon {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SpawnBonusChest {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    StrongholdCorridor {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    StrongholdCrossing {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    StrongholdLibrary {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    UnderwaterRuinBig {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    UnderwaterRuinSmall {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageArmorer {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageButcher {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageCartographer {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageDesertHouse {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageFisher {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageFletcher {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageMason {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillagePlainsHouse {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageSavannaHouse {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageShepherd {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageSnowyHouse {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageTaigaHouse {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageTannery {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageTemple {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageToolsmith {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    VillageWeaponsmith {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    WoodlandMansion {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ArmorStand {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Axolotl {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Bat {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Bee {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Blaze {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Cat {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    CaveSpider {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Chicken {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Cod {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Cow {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Creeper {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Dolphin {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Donkey {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Drowned {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ElderGuardian {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    EnderDragon {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Enderman {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Endermite {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Evoker {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Fox {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Ghast {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Giant {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    GlowSquid {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Goat {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Guardian {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Hoglin {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Horse {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Husk {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Illusioner {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    IronGolem {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Llama {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    MagmaCube {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Mooshroom {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Mule {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Ocelot {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Panda {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Parrot {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Phantom {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Pig {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Piglin {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    PiglinBrute {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Pillager {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Player {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    PolarBear {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Pufferfish {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Rabbit {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Ravager {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Salmon {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Shulker {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Silverfish {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Skeleton {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SkeletonHorse {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Slime {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SnowGolem {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Spider {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Squid {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Stray {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Strider {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    TraderLlama {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    TropicalFish {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Turtle {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Vex {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Villager {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Vindicator {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    WanderingTrader {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Witch {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Wither {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    WitherSkeleton {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Wolf {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Zoglin {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Zombie {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ZombieHorse {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ZombieVillager {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ZombifiedPiglin {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ArmorerGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ButcherGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    CartographerGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    CatMorningGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ClericGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    FarmerGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    FishermanGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Fishing {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    FishingFish {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    FishingJunk {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    FishingTreasure {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    FletcherGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    LeatherworkerGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    LibrarianGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    MasonGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ShepherdGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    ToolsmithGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    WeaponsmithGift {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SnifferDigging {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    PiglinBartering {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    DesertWellArchaeology {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    DesertPyramidArchaeology {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    TrailRuinsArchaeologyCommon {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    TrailRuinsArchaeologyRare {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    OceanRuinWarmArchaeology {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    OceanRuinColdArchaeology {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    Sheep {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepBlack {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepBlue {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepBrown {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepCyan {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepGray {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepGreen {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepLightBlue {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepLightGray {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepLime {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepMagenta {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepOrange {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepPink {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepPurple {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepRed {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepWhite {
        inner: LootTablesLootTablesStruct<'mc>,
    },
    SheepYellow {
        inner: LootTablesLootTablesStruct<'mc>,
    },
}
impl<'mc> std::fmt::Display for LootTablesLootTables<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LootTablesLootTables::Empty { .. } => f.write_str("EMPTY"),
            LootTablesLootTables::AbandonedMineshaft { .. } => f.write_str("ABANDONED_MINESHAFT"),
            LootTablesLootTables::BuriedTreasure { .. } => f.write_str("BURIED_TREASURE"),
            LootTablesLootTables::DesertPyramid { .. } => f.write_str("DESERT_PYRAMID"),
            LootTablesLootTables::EndCityTreasure { .. } => f.write_str("END_CITY_TREASURE"),
            LootTablesLootTables::IglooChest { .. } => f.write_str("IGLOO_CHEST"),
            LootTablesLootTables::JungleTemple { .. } => f.write_str("JUNGLE_TEMPLE"),
            LootTablesLootTables::JungleTempleDispenser { .. } => {
                f.write_str("JUNGLE_TEMPLE_DISPENSER")
            }
            LootTablesLootTables::NetherBridge { .. } => f.write_str("NETHER_BRIDGE"),
            LootTablesLootTables::PillagerOutpost { .. } => f.write_str("PILLAGER_OUTPOST"),
            LootTablesLootTables::BastionTreasure { .. } => f.write_str("BASTION_TREASURE"),
            LootTablesLootTables::BastionOther { .. } => f.write_str("BASTION_OTHER"),
            LootTablesLootTables::BastionBridge { .. } => f.write_str("BASTION_BRIDGE"),
            LootTablesLootTables::BastionHoglinStable { .. } => {
                f.write_str("BASTION_HOGLIN_STABLE")
            }
            LootTablesLootTables::AncientCity { .. } => f.write_str("ANCIENT_CITY"),
            LootTablesLootTables::AncientCityIceBox { .. } => f.write_str("ANCIENT_CITY_ICE_BOX"),
            LootTablesLootTables::RuinedPortal { .. } => f.write_str("RUINED_PORTAL"),
            LootTablesLootTables::ShipwreckMap { .. } => f.write_str("SHIPWRECK_MAP"),
            LootTablesLootTables::ShipwreckSupply { .. } => f.write_str("SHIPWRECK_SUPPLY"),
            LootTablesLootTables::ShipwreckTreasure { .. } => f.write_str("SHIPWRECK_TREASURE"),
            LootTablesLootTables::SimpleDungeon { .. } => f.write_str("SIMPLE_DUNGEON"),
            LootTablesLootTables::SpawnBonusChest { .. } => f.write_str("SPAWN_BONUS_CHEST"),
            LootTablesLootTables::StrongholdCorridor { .. } => f.write_str("STRONGHOLD_CORRIDOR"),
            LootTablesLootTables::StrongholdCrossing { .. } => f.write_str("STRONGHOLD_CROSSING"),
            LootTablesLootTables::StrongholdLibrary { .. } => f.write_str("STRONGHOLD_LIBRARY"),
            LootTablesLootTables::UnderwaterRuinBig { .. } => f.write_str("UNDERWATER_RUIN_BIG"),
            LootTablesLootTables::UnderwaterRuinSmall { .. } => {
                f.write_str("UNDERWATER_RUIN_SMALL")
            }
            LootTablesLootTables::VillageArmorer { .. } => f.write_str("VILLAGE_ARMORER"),
            LootTablesLootTables::VillageButcher { .. } => f.write_str("VILLAGE_BUTCHER"),
            LootTablesLootTables::VillageCartographer { .. } => f.write_str("VILLAGE_CARTOGRAPHER"),
            LootTablesLootTables::VillageDesertHouse { .. } => f.write_str("VILLAGE_DESERT_HOUSE"),
            LootTablesLootTables::VillageFisher { .. } => f.write_str("VILLAGE_FISHER"),
            LootTablesLootTables::VillageFletcher { .. } => f.write_str("VILLAGE_FLETCHER"),
            LootTablesLootTables::VillageMason { .. } => f.write_str("VILLAGE_MASON"),
            LootTablesLootTables::VillagePlainsHouse { .. } => f.write_str("VILLAGE_PLAINS_HOUSE"),
            LootTablesLootTables::VillageSavannaHouse { .. } => {
                f.write_str("VILLAGE_SAVANNA_HOUSE")
            }
            LootTablesLootTables::VillageShepherd { .. } => f.write_str("VILLAGE_SHEPHERD"),
            LootTablesLootTables::VillageSnowyHouse { .. } => f.write_str("VILLAGE_SNOWY_HOUSE"),
            LootTablesLootTables::VillageTaigaHouse { .. } => f.write_str("VILLAGE_TAIGA_HOUSE"),
            LootTablesLootTables::VillageTannery { .. } => f.write_str("VILLAGE_TANNERY"),
            LootTablesLootTables::VillageTemple { .. } => f.write_str("VILLAGE_TEMPLE"),
            LootTablesLootTables::VillageToolsmith { .. } => f.write_str("VILLAGE_TOOLSMITH"),
            LootTablesLootTables::VillageWeaponsmith { .. } => f.write_str("VILLAGE_WEAPONSMITH"),
            LootTablesLootTables::WoodlandMansion { .. } => f.write_str("WOODLAND_MANSION"),
            LootTablesLootTables::ArmorStand { .. } => f.write_str("ARMOR_STAND"),
            LootTablesLootTables::Axolotl { .. } => f.write_str("AXOLOTL"),
            LootTablesLootTables::Bat { .. } => f.write_str("BAT"),
            LootTablesLootTables::Bee { .. } => f.write_str("BEE"),
            LootTablesLootTables::Blaze { .. } => f.write_str("BLAZE"),
            LootTablesLootTables::Cat { .. } => f.write_str("CAT"),
            LootTablesLootTables::CaveSpider { .. } => f.write_str("CAVE_SPIDER"),
            LootTablesLootTables::Chicken { .. } => f.write_str("CHICKEN"),
            LootTablesLootTables::Cod { .. } => f.write_str("COD"),
            LootTablesLootTables::Cow { .. } => f.write_str("COW"),
            LootTablesLootTables::Creeper { .. } => f.write_str("CREEPER"),
            LootTablesLootTables::Dolphin { .. } => f.write_str("DOLPHIN"),
            LootTablesLootTables::Donkey { .. } => f.write_str("DONKEY"),
            LootTablesLootTables::Drowned { .. } => f.write_str("DROWNED"),
            LootTablesLootTables::ElderGuardian { .. } => f.write_str("ELDER_GUARDIAN"),
            LootTablesLootTables::EnderDragon { .. } => f.write_str("ENDER_DRAGON"),
            LootTablesLootTables::Enderman { .. } => f.write_str("ENDERMAN"),
            LootTablesLootTables::Endermite { .. } => f.write_str("ENDERMITE"),
            LootTablesLootTables::Evoker { .. } => f.write_str("EVOKER"),
            LootTablesLootTables::Fox { .. } => f.write_str("FOX"),
            LootTablesLootTables::Ghast { .. } => f.write_str("GHAST"),
            LootTablesLootTables::Giant { .. } => f.write_str("GIANT"),
            LootTablesLootTables::GlowSquid { .. } => f.write_str("GLOW_SQUID"),
            LootTablesLootTables::Goat { .. } => f.write_str("GOAT"),
            LootTablesLootTables::Guardian { .. } => f.write_str("GUARDIAN"),
            LootTablesLootTables::Hoglin { .. } => f.write_str("HOGLIN"),
            LootTablesLootTables::Horse { .. } => f.write_str("HORSE"),
            LootTablesLootTables::Husk { .. } => f.write_str("HUSK"),
            LootTablesLootTables::Illusioner { .. } => f.write_str("ILLUSIONER"),
            LootTablesLootTables::IronGolem { .. } => f.write_str("IRON_GOLEM"),
            LootTablesLootTables::Llama { .. } => f.write_str("LLAMA"),
            LootTablesLootTables::MagmaCube { .. } => f.write_str("MAGMA_CUBE"),
            LootTablesLootTables::Mooshroom { .. } => f.write_str("MOOSHROOM"),
            LootTablesLootTables::Mule { .. } => f.write_str("MULE"),
            LootTablesLootTables::Ocelot { .. } => f.write_str("OCELOT"),
            LootTablesLootTables::Panda { .. } => f.write_str("PANDA"),
            LootTablesLootTables::Parrot { .. } => f.write_str("PARROT"),
            LootTablesLootTables::Phantom { .. } => f.write_str("PHANTOM"),
            LootTablesLootTables::Pig { .. } => f.write_str("PIG"),
            LootTablesLootTables::Piglin { .. } => f.write_str("PIGLIN"),
            LootTablesLootTables::PiglinBrute { .. } => f.write_str("PIGLIN_BRUTE"),
            LootTablesLootTables::Pillager { .. } => f.write_str("PILLAGER"),
            LootTablesLootTables::Player { .. } => f.write_str("PLAYER"),
            LootTablesLootTables::PolarBear { .. } => f.write_str("POLAR_BEAR"),
            LootTablesLootTables::Pufferfish { .. } => f.write_str("PUFFERFISH"),
            LootTablesLootTables::Rabbit { .. } => f.write_str("RABBIT"),
            LootTablesLootTables::Ravager { .. } => f.write_str("RAVAGER"),
            LootTablesLootTables::Salmon { .. } => f.write_str("SALMON"),
            LootTablesLootTables::Shulker { .. } => f.write_str("SHULKER"),
            LootTablesLootTables::Silverfish { .. } => f.write_str("SILVERFISH"),
            LootTablesLootTables::Skeleton { .. } => f.write_str("SKELETON"),
            LootTablesLootTables::SkeletonHorse { .. } => f.write_str("SKELETON_HORSE"),
            LootTablesLootTables::Slime { .. } => f.write_str("SLIME"),
            LootTablesLootTables::SnowGolem { .. } => f.write_str("SNOW_GOLEM"),
            LootTablesLootTables::Spider { .. } => f.write_str("SPIDER"),
            LootTablesLootTables::Squid { .. } => f.write_str("SQUID"),
            LootTablesLootTables::Stray { .. } => f.write_str("STRAY"),
            LootTablesLootTables::Strider { .. } => f.write_str("STRIDER"),
            LootTablesLootTables::TraderLlama { .. } => f.write_str("TRADER_LLAMA"),
            LootTablesLootTables::TropicalFish { .. } => f.write_str("TROPICAL_FISH"),
            LootTablesLootTables::Turtle { .. } => f.write_str("TURTLE"),
            LootTablesLootTables::Vex { .. } => f.write_str("VEX"),
            LootTablesLootTables::Villager { .. } => f.write_str("VILLAGER"),
            LootTablesLootTables::Vindicator { .. } => f.write_str("VINDICATOR"),
            LootTablesLootTables::WanderingTrader { .. } => f.write_str("WANDERING_TRADER"),
            LootTablesLootTables::Witch { .. } => f.write_str("WITCH"),
            LootTablesLootTables::Wither { .. } => f.write_str("WITHER"),
            LootTablesLootTables::WitherSkeleton { .. } => f.write_str("WITHER_SKELETON"),
            LootTablesLootTables::Wolf { .. } => f.write_str("WOLF"),
            LootTablesLootTables::Zoglin { .. } => f.write_str("ZOGLIN"),
            LootTablesLootTables::Zombie { .. } => f.write_str("ZOMBIE"),
            LootTablesLootTables::ZombieHorse { .. } => f.write_str("ZOMBIE_HORSE"),
            LootTablesLootTables::ZombieVillager { .. } => f.write_str("ZOMBIE_VILLAGER"),
            LootTablesLootTables::ZombifiedPiglin { .. } => f.write_str("ZOMBIFIED_PIGLIN"),
            LootTablesLootTables::ArmorerGift { .. } => f.write_str("ARMORER_GIFT"),
            LootTablesLootTables::ButcherGift { .. } => f.write_str("BUTCHER_GIFT"),
            LootTablesLootTables::CartographerGift { .. } => f.write_str("CARTOGRAPHER_GIFT"),
            LootTablesLootTables::CatMorningGift { .. } => f.write_str("CAT_MORNING_GIFT"),
            LootTablesLootTables::ClericGift { .. } => f.write_str("CLERIC_GIFT"),
            LootTablesLootTables::FarmerGift { .. } => f.write_str("FARMER_GIFT"),
            LootTablesLootTables::FishermanGift { .. } => f.write_str("FISHERMAN_GIFT"),
            LootTablesLootTables::Fishing { .. } => f.write_str("FISHING"),
            LootTablesLootTables::FishingFish { .. } => f.write_str("FISHING_FISH"),
            LootTablesLootTables::FishingJunk { .. } => f.write_str("FISHING_JUNK"),
            LootTablesLootTables::FishingTreasure { .. } => f.write_str("FISHING_TREASURE"),
            LootTablesLootTables::FletcherGift { .. } => f.write_str("FLETCHER_GIFT"),
            LootTablesLootTables::LeatherworkerGift { .. } => f.write_str("LEATHERWORKER_GIFT"),
            LootTablesLootTables::LibrarianGift { .. } => f.write_str("LIBRARIAN_GIFT"),
            LootTablesLootTables::MasonGift { .. } => f.write_str("MASON_GIFT"),
            LootTablesLootTables::ShepherdGift { .. } => f.write_str("SHEPHERD_GIFT"),
            LootTablesLootTables::ToolsmithGift { .. } => f.write_str("TOOLSMITH_GIFT"),
            LootTablesLootTables::WeaponsmithGift { .. } => f.write_str("WEAPONSMITH_GIFT"),
            LootTablesLootTables::SnifferDigging { .. } => f.write_str("SNIFFER_DIGGING"),
            LootTablesLootTables::PiglinBartering { .. } => f.write_str("PIGLIN_BARTERING"),
            LootTablesLootTables::DesertWellArchaeology { .. } => {
                f.write_str("DESERT_WELL_ARCHAEOLOGY")
            }
            LootTablesLootTables::DesertPyramidArchaeology { .. } => {
                f.write_str("DESERT_PYRAMID_ARCHAEOLOGY")
            }
            LootTablesLootTables::TrailRuinsArchaeologyCommon { .. } => {
                f.write_str("TRAIL_RUINS_ARCHAEOLOGY_COMMON")
            }
            LootTablesLootTables::TrailRuinsArchaeologyRare { .. } => {
                f.write_str("TRAIL_RUINS_ARCHAEOLOGY_RARE")
            }
            LootTablesLootTables::OceanRuinWarmArchaeology { .. } => {
                f.write_str("OCEAN_RUIN_WARM_ARCHAEOLOGY")
            }
            LootTablesLootTables::OceanRuinColdArchaeology { .. } => {
                f.write_str("OCEAN_RUIN_COLD_ARCHAEOLOGY")
            }
            LootTablesLootTables::Sheep { .. } => f.write_str("SHEEP"),
            LootTablesLootTables::SheepBlack { .. } => f.write_str("SHEEP_BLACK"),
            LootTablesLootTables::SheepBlue { .. } => f.write_str("SHEEP_BLUE"),
            LootTablesLootTables::SheepBrown { .. } => f.write_str("SHEEP_BROWN"),
            LootTablesLootTables::SheepCyan { .. } => f.write_str("SHEEP_CYAN"),
            LootTablesLootTables::SheepGray { .. } => f.write_str("SHEEP_GRAY"),
            LootTablesLootTables::SheepGreen { .. } => f.write_str("SHEEP_GREEN"),
            LootTablesLootTables::SheepLightBlue { .. } => f.write_str("SHEEP_LIGHT_BLUE"),
            LootTablesLootTables::SheepLightGray { .. } => f.write_str("SHEEP_LIGHT_GRAY"),
            LootTablesLootTables::SheepLime { .. } => f.write_str("SHEEP_LIME"),
            LootTablesLootTables::SheepMagenta { .. } => f.write_str("SHEEP_MAGENTA"),
            LootTablesLootTables::SheepOrange { .. } => f.write_str("SHEEP_ORANGE"),
            LootTablesLootTables::SheepPink { .. } => f.write_str("SHEEP_PINK"),
            LootTablesLootTables::SheepPurple { .. } => f.write_str("SHEEP_PURPLE"),
            LootTablesLootTables::SheepRed { .. } => f.write_str("SHEEP_RED"),
            LootTablesLootTables::SheepWhite { .. } => f.write_str("SHEEP_WHITE"),
            LootTablesLootTables::SheepYellow { .. } => f.write_str("SHEEP_YELLOW"),
        }
    }
}

impl<'mc> LootTablesLootTables<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<LootTablesLootTables<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/loot/LootTables$LootTables");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/loot/LootTables$LootTables;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "EMPTY" => Ok(LootTablesLootTables::Empty {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ABANDONED_MINESHAFT" => Ok(LootTablesLootTables::AbandonedMineshaft {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "BURIED_TREASURE" => Ok(LootTablesLootTables::BuriedTreasure {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "DESERT_PYRAMID" => Ok(LootTablesLootTables::DesertPyramid {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "END_CITY_TREASURE" => Ok(LootTablesLootTables::EndCityTreasure {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "IGLOO_CHEST" => Ok(LootTablesLootTables::IglooChest {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "JUNGLE_TEMPLE" => Ok(LootTablesLootTables::JungleTemple {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "JUNGLE_TEMPLE_DISPENSER" => Ok(LootTablesLootTables::JungleTempleDispenser {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "NETHER_BRIDGE" => Ok(LootTablesLootTables::NetherBridge {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "PILLAGER_OUTPOST" => Ok(LootTablesLootTables::PillagerOutpost {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_TREASURE" => Ok(LootTablesLootTables::BastionTreasure {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_OTHER" => Ok(LootTablesLootTables::BastionOther {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_BRIDGE" => Ok(LootTablesLootTables::BastionBridge {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_HOGLIN_STABLE" => Ok(LootTablesLootTables::BastionHoglinStable {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ANCIENT_CITY" => Ok(LootTablesLootTables::AncientCity {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ANCIENT_CITY_ICE_BOX" => Ok(LootTablesLootTables::AncientCityIceBox {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "RUINED_PORTAL" => Ok(LootTablesLootTables::RuinedPortal {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHIPWRECK_MAP" => Ok(LootTablesLootTables::ShipwreckMap {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHIPWRECK_SUPPLY" => Ok(LootTablesLootTables::ShipwreckSupply {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHIPWRECK_TREASURE" => Ok(LootTablesLootTables::ShipwreckTreasure {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SIMPLE_DUNGEON" => Ok(LootTablesLootTables::SimpleDungeon {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SPAWN_BONUS_CHEST" => Ok(LootTablesLootTables::SpawnBonusChest {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "STRONGHOLD_CORRIDOR" => Ok(LootTablesLootTables::StrongholdCorridor {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "STRONGHOLD_CROSSING" => Ok(LootTablesLootTables::StrongholdCrossing {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "STRONGHOLD_LIBRARY" => Ok(LootTablesLootTables::StrongholdLibrary {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "UNDERWATER_RUIN_BIG" => Ok(LootTablesLootTables::UnderwaterRuinBig {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "UNDERWATER_RUIN_SMALL" => Ok(LootTablesLootTables::UnderwaterRuinSmall {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_ARMORER" => Ok(LootTablesLootTables::VillageArmorer {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_BUTCHER" => Ok(LootTablesLootTables::VillageButcher {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_CARTOGRAPHER" => Ok(LootTablesLootTables::VillageCartographer {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_DESERT_HOUSE" => Ok(LootTablesLootTables::VillageDesertHouse {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_FISHER" => Ok(LootTablesLootTables::VillageFisher {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_FLETCHER" => Ok(LootTablesLootTables::VillageFletcher {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_MASON" => Ok(LootTablesLootTables::VillageMason {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_PLAINS_HOUSE" => Ok(LootTablesLootTables::VillagePlainsHouse {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_SAVANNA_HOUSE" => Ok(LootTablesLootTables::VillageSavannaHouse {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_SHEPHERD" => Ok(LootTablesLootTables::VillageShepherd {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_SNOWY_HOUSE" => Ok(LootTablesLootTables::VillageSnowyHouse {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TAIGA_HOUSE" => Ok(LootTablesLootTables::VillageTaigaHouse {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TANNERY" => Ok(LootTablesLootTables::VillageTannery {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TEMPLE" => Ok(LootTablesLootTables::VillageTemple {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TOOLSMITH" => Ok(LootTablesLootTables::VillageToolsmith {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_WEAPONSMITH" => Ok(LootTablesLootTables::VillageWeaponsmith {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "WOODLAND_MANSION" => Ok(LootTablesLootTables::WoodlandMansion {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ARMOR_STAND" => Ok(LootTablesLootTables::ArmorStand {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "AXOLOTL" => Ok(LootTablesLootTables::Axolotl {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "BAT" => Ok(LootTablesLootTables::Bat {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "BEE" => Ok(LootTablesLootTables::Bee {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "BLAZE" => Ok(LootTablesLootTables::Blaze {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "CAT" => Ok(LootTablesLootTables::Cat {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "CAVE_SPIDER" => Ok(LootTablesLootTables::CaveSpider {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "CHICKEN" => Ok(LootTablesLootTables::Chicken {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "COD" => Ok(LootTablesLootTables::Cod {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "COW" => Ok(LootTablesLootTables::Cow {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "CREEPER" => Ok(LootTablesLootTables::Creeper {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "DOLPHIN" => Ok(LootTablesLootTables::Dolphin {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "DONKEY" => Ok(LootTablesLootTables::Donkey {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "DROWNED" => Ok(LootTablesLootTables::Drowned {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ELDER_GUARDIAN" => Ok(LootTablesLootTables::ElderGuardian {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ENDER_DRAGON" => Ok(LootTablesLootTables::EnderDragon {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ENDERMAN" => Ok(LootTablesLootTables::Enderman {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ENDERMITE" => Ok(LootTablesLootTables::Endermite {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "EVOKER" => Ok(LootTablesLootTables::Evoker {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "FOX" => Ok(LootTablesLootTables::Fox {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "GHAST" => Ok(LootTablesLootTables::Ghast {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "GIANT" => Ok(LootTablesLootTables::Giant {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "GLOW_SQUID" => Ok(LootTablesLootTables::GlowSquid {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "GOAT" => Ok(LootTablesLootTables::Goat {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "GUARDIAN" => Ok(LootTablesLootTables::Guardian {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "HOGLIN" => Ok(LootTablesLootTables::Hoglin {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "HORSE" => Ok(LootTablesLootTables::Horse {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "HUSK" => Ok(LootTablesLootTables::Husk {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ILLUSIONER" => Ok(LootTablesLootTables::Illusioner {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "IRON_GOLEM" => Ok(LootTablesLootTables::IronGolem {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "LLAMA" => Ok(LootTablesLootTables::Llama {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "MAGMA_CUBE" => Ok(LootTablesLootTables::MagmaCube {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "MOOSHROOM" => Ok(LootTablesLootTables::Mooshroom {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "MULE" => Ok(LootTablesLootTables::Mule {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "OCELOT" => Ok(LootTablesLootTables::Ocelot {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "PANDA" => Ok(LootTablesLootTables::Panda {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "PARROT" => Ok(LootTablesLootTables::Parrot {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "PHANTOM" => Ok(LootTablesLootTables::Phantom {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "PIG" => Ok(LootTablesLootTables::Pig {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "PIGLIN" => Ok(LootTablesLootTables::Piglin {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_BRUTE" => Ok(LootTablesLootTables::PiglinBrute {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "PILLAGER" => Ok(LootTablesLootTables::Pillager {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "PLAYER" => Ok(LootTablesLootTables::Player {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "POLAR_BEAR" => Ok(LootTablesLootTables::PolarBear {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "PUFFERFISH" => Ok(LootTablesLootTables::Pufferfish {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "RABBIT" => Ok(LootTablesLootTables::Rabbit {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "RAVAGER" => Ok(LootTablesLootTables::Ravager {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SALMON" => Ok(LootTablesLootTables::Salmon {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHULKER" => Ok(LootTablesLootTables::Shulker {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SILVERFISH" => Ok(LootTablesLootTables::Silverfish {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SKELETON" => Ok(LootTablesLootTables::Skeleton {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SKELETON_HORSE" => Ok(LootTablesLootTables::SkeletonHorse {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SLIME" => Ok(LootTablesLootTables::Slime {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SNOW_GOLEM" => Ok(LootTablesLootTables::SnowGolem {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SPIDER" => Ok(LootTablesLootTables::Spider {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SQUID" => Ok(LootTablesLootTables::Squid {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "STRAY" => Ok(LootTablesLootTables::Stray {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "STRIDER" => Ok(LootTablesLootTables::Strider {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "TRADER_LLAMA" => Ok(LootTablesLootTables::TraderLlama {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "TROPICAL_FISH" => Ok(LootTablesLootTables::TropicalFish {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "TURTLE" => Ok(LootTablesLootTables::Turtle {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VEX" => Ok(LootTablesLootTables::Vex {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGER" => Ok(LootTablesLootTables::Villager {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "VINDICATOR" => Ok(LootTablesLootTables::Vindicator {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "WANDERING_TRADER" => Ok(LootTablesLootTables::WanderingTrader {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "WITCH" => Ok(LootTablesLootTables::Witch {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "WITHER" => Ok(LootTablesLootTables::Wither {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "WITHER_SKELETON" => Ok(LootTablesLootTables::WitherSkeleton {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "WOLF" => Ok(LootTablesLootTables::Wolf {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOGLIN" => Ok(LootTablesLootTables::Zoglin {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIE" => Ok(LootTablesLootTables::Zombie {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIE_HORSE" => Ok(LootTablesLootTables::ZombieHorse {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIE_VILLAGER" => Ok(LootTablesLootTables::ZombieVillager {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIFIED_PIGLIN" => Ok(LootTablesLootTables::ZombifiedPiglin {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "ARMORER_GIFT" => Ok(LootTablesLootTables::ArmorerGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "BUTCHER_GIFT" => Ok(LootTablesLootTables::ButcherGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "CARTOGRAPHER_GIFT" => Ok(LootTablesLootTables::CartographerGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "CAT_MORNING_GIFT" => Ok(LootTablesLootTables::CatMorningGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "CLERIC_GIFT" => Ok(LootTablesLootTables::ClericGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "FARMER_GIFT" => Ok(LootTablesLootTables::FarmerGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHERMAN_GIFT" => Ok(LootTablesLootTables::FishermanGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING" => Ok(LootTablesLootTables::Fishing {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING_FISH" => Ok(LootTablesLootTables::FishingFish {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING_JUNK" => Ok(LootTablesLootTables::FishingJunk {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING_TREASURE" => Ok(LootTablesLootTables::FishingTreasure {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "FLETCHER_GIFT" => Ok(LootTablesLootTables::FletcherGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "LEATHERWORKER_GIFT" => Ok(LootTablesLootTables::LeatherworkerGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "LIBRARIAN_GIFT" => Ok(LootTablesLootTables::LibrarianGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "MASON_GIFT" => Ok(LootTablesLootTables::MasonGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEPHERD_GIFT" => Ok(LootTablesLootTables::ShepherdGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "TOOLSMITH_GIFT" => Ok(LootTablesLootTables::ToolsmithGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "WEAPONSMITH_GIFT" => Ok(LootTablesLootTables::WeaponsmithGift {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SNIFFER_DIGGING" => Ok(LootTablesLootTables::SnifferDigging {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_BARTERING" => Ok(LootTablesLootTables::PiglinBartering {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "DESERT_WELL_ARCHAEOLOGY" => Ok(LootTablesLootTables::DesertWellArchaeology {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "DESERT_PYRAMID_ARCHAEOLOGY" => Ok(LootTablesLootTables::DesertPyramidArchaeology {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "TRAIL_RUINS_ARCHAEOLOGY_COMMON" => {
                Ok(LootTablesLootTables::TrailRuinsArchaeologyCommon {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                })
            }
            "TRAIL_RUINS_ARCHAEOLOGY_RARE" => Ok(LootTablesLootTables::TrailRuinsArchaeologyRare {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "OCEAN_RUIN_WARM_ARCHAEOLOGY" => Ok(LootTablesLootTables::OceanRuinWarmArchaeology {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "OCEAN_RUIN_COLD_ARCHAEOLOGY" => Ok(LootTablesLootTables::OceanRuinColdArchaeology {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP" => Ok(LootTablesLootTables::Sheep {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_BLACK" => Ok(LootTablesLootTables::SheepBlack {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_BLUE" => Ok(LootTablesLootTables::SheepBlue {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_BROWN" => Ok(LootTablesLootTables::SheepBrown {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_CYAN" => Ok(LootTablesLootTables::SheepCyan {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_GRAY" => Ok(LootTablesLootTables::SheepGray {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_GREEN" => Ok(LootTablesLootTables::SheepGreen {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_LIGHT_BLUE" => Ok(LootTablesLootTables::SheepLightBlue {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_LIGHT_GRAY" => Ok(LootTablesLootTables::SheepLightGray {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_LIME" => Ok(LootTablesLootTables::SheepLime {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_MAGENTA" => Ok(LootTablesLootTables::SheepMagenta {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_ORANGE" => Ok(LootTablesLootTables::SheepOrange {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_PINK" => Ok(LootTablesLootTables::SheepPink {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_PURPLE" => Ok(LootTablesLootTables::SheepPurple {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_RED" => Ok(LootTablesLootTables::SheepRed {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_WHITE" => Ok(LootTablesLootTables::SheepWhite {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_YELLOW" => Ok(LootTablesLootTables::SheepYellow {
                inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct LootTablesLootTablesStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LootTablesLootTables<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Empty { inner } => inner.0.clone(),
            Self::AbandonedMineshaft { inner } => inner.0.clone(),
            Self::BuriedTreasure { inner } => inner.0.clone(),
            Self::DesertPyramid { inner } => inner.0.clone(),
            Self::EndCityTreasure { inner } => inner.0.clone(),
            Self::IglooChest { inner } => inner.0.clone(),
            Self::JungleTemple { inner } => inner.0.clone(),
            Self::JungleTempleDispenser { inner } => inner.0.clone(),
            Self::NetherBridge { inner } => inner.0.clone(),
            Self::PillagerOutpost { inner } => inner.0.clone(),
            Self::BastionTreasure { inner } => inner.0.clone(),
            Self::BastionOther { inner } => inner.0.clone(),
            Self::BastionBridge { inner } => inner.0.clone(),
            Self::BastionHoglinStable { inner } => inner.0.clone(),
            Self::AncientCity { inner } => inner.0.clone(),
            Self::AncientCityIceBox { inner } => inner.0.clone(),
            Self::RuinedPortal { inner } => inner.0.clone(),
            Self::ShipwreckMap { inner } => inner.0.clone(),
            Self::ShipwreckSupply { inner } => inner.0.clone(),
            Self::ShipwreckTreasure { inner } => inner.0.clone(),
            Self::SimpleDungeon { inner } => inner.0.clone(),
            Self::SpawnBonusChest { inner } => inner.0.clone(),
            Self::StrongholdCorridor { inner } => inner.0.clone(),
            Self::StrongholdCrossing { inner } => inner.0.clone(),
            Self::StrongholdLibrary { inner } => inner.0.clone(),
            Self::UnderwaterRuinBig { inner } => inner.0.clone(),
            Self::UnderwaterRuinSmall { inner } => inner.0.clone(),
            Self::VillageArmorer { inner } => inner.0.clone(),
            Self::VillageButcher { inner } => inner.0.clone(),
            Self::VillageCartographer { inner } => inner.0.clone(),
            Self::VillageDesertHouse { inner } => inner.0.clone(),
            Self::VillageFisher { inner } => inner.0.clone(),
            Self::VillageFletcher { inner } => inner.0.clone(),
            Self::VillageMason { inner } => inner.0.clone(),
            Self::VillagePlainsHouse { inner } => inner.0.clone(),
            Self::VillageSavannaHouse { inner } => inner.0.clone(),
            Self::VillageShepherd { inner } => inner.0.clone(),
            Self::VillageSnowyHouse { inner } => inner.0.clone(),
            Self::VillageTaigaHouse { inner } => inner.0.clone(),
            Self::VillageTannery { inner } => inner.0.clone(),
            Self::VillageTemple { inner } => inner.0.clone(),
            Self::VillageToolsmith { inner } => inner.0.clone(),
            Self::VillageWeaponsmith { inner } => inner.0.clone(),
            Self::WoodlandMansion { inner } => inner.0.clone(),
            Self::ArmorStand { inner } => inner.0.clone(),
            Self::Axolotl { inner } => inner.0.clone(),
            Self::Bat { inner } => inner.0.clone(),
            Self::Bee { inner } => inner.0.clone(),
            Self::Blaze { inner } => inner.0.clone(),
            Self::Cat { inner } => inner.0.clone(),
            Self::CaveSpider { inner } => inner.0.clone(),
            Self::Chicken { inner } => inner.0.clone(),
            Self::Cod { inner } => inner.0.clone(),
            Self::Cow { inner } => inner.0.clone(),
            Self::Creeper { inner } => inner.0.clone(),
            Self::Dolphin { inner } => inner.0.clone(),
            Self::Donkey { inner } => inner.0.clone(),
            Self::Drowned { inner } => inner.0.clone(),
            Self::ElderGuardian { inner } => inner.0.clone(),
            Self::EnderDragon { inner } => inner.0.clone(),
            Self::Enderman { inner } => inner.0.clone(),
            Self::Endermite { inner } => inner.0.clone(),
            Self::Evoker { inner } => inner.0.clone(),
            Self::Fox { inner } => inner.0.clone(),
            Self::Ghast { inner } => inner.0.clone(),
            Self::Giant { inner } => inner.0.clone(),
            Self::GlowSquid { inner } => inner.0.clone(),
            Self::Goat { inner } => inner.0.clone(),
            Self::Guardian { inner } => inner.0.clone(),
            Self::Hoglin { inner } => inner.0.clone(),
            Self::Horse { inner } => inner.0.clone(),
            Self::Husk { inner } => inner.0.clone(),
            Self::Illusioner { inner } => inner.0.clone(),
            Self::IronGolem { inner } => inner.0.clone(),
            Self::Llama { inner } => inner.0.clone(),
            Self::MagmaCube { inner } => inner.0.clone(),
            Self::Mooshroom { inner } => inner.0.clone(),
            Self::Mule { inner } => inner.0.clone(),
            Self::Ocelot { inner } => inner.0.clone(),
            Self::Panda { inner } => inner.0.clone(),
            Self::Parrot { inner } => inner.0.clone(),
            Self::Phantom { inner } => inner.0.clone(),
            Self::Pig { inner } => inner.0.clone(),
            Self::Piglin { inner } => inner.0.clone(),
            Self::PiglinBrute { inner } => inner.0.clone(),
            Self::Pillager { inner } => inner.0.clone(),
            Self::Player { inner } => inner.0.clone(),
            Self::PolarBear { inner } => inner.0.clone(),
            Self::Pufferfish { inner } => inner.0.clone(),
            Self::Rabbit { inner } => inner.0.clone(),
            Self::Ravager { inner } => inner.0.clone(),
            Self::Salmon { inner } => inner.0.clone(),
            Self::Shulker { inner } => inner.0.clone(),
            Self::Silverfish { inner } => inner.0.clone(),
            Self::Skeleton { inner } => inner.0.clone(),
            Self::SkeletonHorse { inner } => inner.0.clone(),
            Self::Slime { inner } => inner.0.clone(),
            Self::SnowGolem { inner } => inner.0.clone(),
            Self::Spider { inner } => inner.0.clone(),
            Self::Squid { inner } => inner.0.clone(),
            Self::Stray { inner } => inner.0.clone(),
            Self::Strider { inner } => inner.0.clone(),
            Self::TraderLlama { inner } => inner.0.clone(),
            Self::TropicalFish { inner } => inner.0.clone(),
            Self::Turtle { inner } => inner.0.clone(),
            Self::Vex { inner } => inner.0.clone(),
            Self::Villager { inner } => inner.0.clone(),
            Self::Vindicator { inner } => inner.0.clone(),
            Self::WanderingTrader { inner } => inner.0.clone(),
            Self::Witch { inner } => inner.0.clone(),
            Self::Wither { inner } => inner.0.clone(),
            Self::WitherSkeleton { inner } => inner.0.clone(),
            Self::Wolf { inner } => inner.0.clone(),
            Self::Zoglin { inner } => inner.0.clone(),
            Self::Zombie { inner } => inner.0.clone(),
            Self::ZombieHorse { inner } => inner.0.clone(),
            Self::ZombieVillager { inner } => inner.0.clone(),
            Self::ZombifiedPiglin { inner } => inner.0.clone(),
            Self::ArmorerGift { inner } => inner.0.clone(),
            Self::ButcherGift { inner } => inner.0.clone(),
            Self::CartographerGift { inner } => inner.0.clone(),
            Self::CatMorningGift { inner } => inner.0.clone(),
            Self::ClericGift { inner } => inner.0.clone(),
            Self::FarmerGift { inner } => inner.0.clone(),
            Self::FishermanGift { inner } => inner.0.clone(),
            Self::Fishing { inner } => inner.0.clone(),
            Self::FishingFish { inner } => inner.0.clone(),
            Self::FishingJunk { inner } => inner.0.clone(),
            Self::FishingTreasure { inner } => inner.0.clone(),
            Self::FletcherGift { inner } => inner.0.clone(),
            Self::LeatherworkerGift { inner } => inner.0.clone(),
            Self::LibrarianGift { inner } => inner.0.clone(),
            Self::MasonGift { inner } => inner.0.clone(),
            Self::ShepherdGift { inner } => inner.0.clone(),
            Self::ToolsmithGift { inner } => inner.0.clone(),
            Self::WeaponsmithGift { inner } => inner.0.clone(),
            Self::SnifferDigging { inner } => inner.0.clone(),
            Self::PiglinBartering { inner } => inner.0.clone(),
            Self::DesertWellArchaeology { inner } => inner.0.clone(),
            Self::DesertPyramidArchaeology { inner } => inner.0.clone(),
            Self::TrailRuinsArchaeologyCommon { inner } => inner.0.clone(),
            Self::TrailRuinsArchaeologyRare { inner } => inner.0.clone(),
            Self::OceanRuinWarmArchaeology { inner } => inner.0.clone(),
            Self::OceanRuinColdArchaeology { inner } => inner.0.clone(),
            Self::Sheep { inner } => inner.0.clone(),
            Self::SheepBlack { inner } => inner.0.clone(),
            Self::SheepBlue { inner } => inner.0.clone(),
            Self::SheepBrown { inner } => inner.0.clone(),
            Self::SheepCyan { inner } => inner.0.clone(),
            Self::SheepGray { inner } => inner.0.clone(),
            Self::SheepGreen { inner } => inner.0.clone(),
            Self::SheepLightBlue { inner } => inner.0.clone(),
            Self::SheepLightGray { inner } => inner.0.clone(),
            Self::SheepLime { inner } => inner.0.clone(),
            Self::SheepMagenta { inner } => inner.0.clone(),
            Self::SheepOrange { inner } => inner.0.clone(),
            Self::SheepPink { inner } => inner.0.clone(),
            Self::SheepPurple { inner } => inner.0.clone(),
            Self::SheepRed { inner } => inner.0.clone(),
            Self::SheepWhite { inner } => inner.0.clone(),
            Self::SheepYellow { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Empty { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::AbandonedMineshaft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuriedTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DesertPyramid { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EndCityTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::IglooChest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::JungleTemple { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::JungleTempleDispenser { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NetherBridge { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PillagerOutpost { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionOther { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionBridge { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionHoglinStable { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AncientCity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AncientCityIceBox { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RuinedPortal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShipwreckMap { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShipwreckSupply { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShipwreckTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SimpleDungeon { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SpawnBonusChest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongholdCorridor { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongholdCrossing { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongholdLibrary { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::UnderwaterRuinBig { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::UnderwaterRuinSmall { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageArmorer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageButcher { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageCartographer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageDesertHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageFisher { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageFletcher { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageMason { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillagePlainsHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageSavannaHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageShepherd { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageSnowyHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageTaigaHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageTannery { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageTemple { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageToolsmith { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageWeaponsmith { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WoodlandMansion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ArmorStand { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Axolotl { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Bat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Bee { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Blaze { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::CaveSpider { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Chicken { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cod { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Creeper { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Dolphin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Donkey { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Drowned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ElderGuardian { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnderDragon { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Enderman { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Endermite { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Evoker { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Fox { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ghast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Giant { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::GlowSquid { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Goat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Guardian { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Hoglin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Horse { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Husk { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Illusioner { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::IronGolem { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Llama { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::MagmaCube { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Mooshroom { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Mule { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ocelot { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Panda { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Parrot { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Phantom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Pig { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Piglin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PiglinBrute { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Pillager { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Player { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PolarBear { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Pufferfish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Rabbit { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ravager { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Salmon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Shulker { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Silverfish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Skeleton { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SkeletonHorse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Slime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SnowGolem { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Spider { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Squid { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Stray { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Strider { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TraderLlama { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TropicalFish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Turtle { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Vex { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Villager { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Vindicator { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WanderingTrader { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Witch { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Wither { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WitherSkeleton { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Wolf { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Zoglin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Zombie { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ZombieHorse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ZombieVillager { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ZombifiedPiglin { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ArmorerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ButcherGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CartographerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CatMorningGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ClericGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FarmerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FishermanGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Fishing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FishingFish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FishingJunk { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FishingTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FletcherGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LeatherworkerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LibrarianGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::MasonGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShepherdGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ToolsmithGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WeaponsmithGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SnifferDigging { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PiglinBartering { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DesertWellArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DesertPyramidArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrailRuinsArchaeologyCommon { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrailRuinsArchaeologyRare { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OceanRuinWarmArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OceanRuinColdArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Sheep { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SheepBlack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepBlue { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepBrown { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepCyan { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepGray { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepGreen { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepLightBlue { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepLightGray { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepLime { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepMagenta { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepOrange { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepPink { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepPurple { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepRed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SheepWhite { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepYellow { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootTablesLootTables<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "EMPTY" => Ok(LootTablesLootTables::Empty {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ABANDONED_MINESHAFT" => Ok(LootTablesLootTables::AbandonedMineshaft {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "BURIED_TREASURE" => Ok(LootTablesLootTables::BuriedTreasure {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "DESERT_PYRAMID" => Ok(LootTablesLootTables::DesertPyramid {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "END_CITY_TREASURE" => Ok(LootTablesLootTables::EndCityTreasure {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "IGLOO_CHEST" => Ok(LootTablesLootTables::IglooChest {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "JUNGLE_TEMPLE" => Ok(LootTablesLootTables::JungleTemple {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "JUNGLE_TEMPLE_DISPENSER" => Ok(LootTablesLootTables::JungleTempleDispenser {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "NETHER_BRIDGE" => Ok(LootTablesLootTables::NetherBridge {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "PILLAGER_OUTPOST" => Ok(LootTablesLootTables::PillagerOutpost {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_TREASURE" => Ok(LootTablesLootTables::BastionTreasure {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_OTHER" => Ok(LootTablesLootTables::BastionOther {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_BRIDGE" => Ok(LootTablesLootTables::BastionBridge {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_HOGLIN_STABLE" => Ok(LootTablesLootTables::BastionHoglinStable {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ANCIENT_CITY" => Ok(LootTablesLootTables::AncientCity {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ANCIENT_CITY_ICE_BOX" => Ok(LootTablesLootTables::AncientCityIceBox {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "RUINED_PORTAL" => Ok(LootTablesLootTables::RuinedPortal {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHIPWRECK_MAP" => Ok(LootTablesLootTables::ShipwreckMap {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHIPWRECK_SUPPLY" => Ok(LootTablesLootTables::ShipwreckSupply {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHIPWRECK_TREASURE" => Ok(LootTablesLootTables::ShipwreckTreasure {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SIMPLE_DUNGEON" => Ok(LootTablesLootTables::SimpleDungeon {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SPAWN_BONUS_CHEST" => Ok(LootTablesLootTables::SpawnBonusChest {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "STRONGHOLD_CORRIDOR" => Ok(LootTablesLootTables::StrongholdCorridor {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "STRONGHOLD_CROSSING" => Ok(LootTablesLootTables::StrongholdCrossing {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "STRONGHOLD_LIBRARY" => Ok(LootTablesLootTables::StrongholdLibrary {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "UNDERWATER_RUIN_BIG" => Ok(LootTablesLootTables::UnderwaterRuinBig {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "UNDERWATER_RUIN_SMALL" => Ok(LootTablesLootTables::UnderwaterRuinSmall {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_ARMORER" => Ok(LootTablesLootTables::VillageArmorer {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_BUTCHER" => Ok(LootTablesLootTables::VillageButcher {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_CARTOGRAPHER" => Ok(LootTablesLootTables::VillageCartographer {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_DESERT_HOUSE" => Ok(LootTablesLootTables::VillageDesertHouse {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_FISHER" => Ok(LootTablesLootTables::VillageFisher {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_FLETCHER" => Ok(LootTablesLootTables::VillageFletcher {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_MASON" => Ok(LootTablesLootTables::VillageMason {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_PLAINS_HOUSE" => Ok(LootTablesLootTables::VillagePlainsHouse {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_SAVANNA_HOUSE" => Ok(LootTablesLootTables::VillageSavannaHouse {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_SHEPHERD" => Ok(LootTablesLootTables::VillageShepherd {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_SNOWY_HOUSE" => Ok(LootTablesLootTables::VillageSnowyHouse {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TAIGA_HOUSE" => Ok(LootTablesLootTables::VillageTaigaHouse {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TANNERY" => Ok(LootTablesLootTables::VillageTannery {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TEMPLE" => Ok(LootTablesLootTables::VillageTemple {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TOOLSMITH" => Ok(LootTablesLootTables::VillageToolsmith {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_WEAPONSMITH" => Ok(LootTablesLootTables::VillageWeaponsmith {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "WOODLAND_MANSION" => Ok(LootTablesLootTables::WoodlandMansion {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ARMOR_STAND" => Ok(LootTablesLootTables::ArmorStand {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "AXOLOTL" => Ok(LootTablesLootTables::Axolotl {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "BAT" => Ok(LootTablesLootTables::Bat {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "BEE" => Ok(LootTablesLootTables::Bee {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "BLAZE" => Ok(LootTablesLootTables::Blaze {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "CAT" => Ok(LootTablesLootTables::Cat {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "CAVE_SPIDER" => Ok(LootTablesLootTables::CaveSpider {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "CHICKEN" => Ok(LootTablesLootTables::Chicken {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "COD" => Ok(LootTablesLootTables::Cod {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "COW" => Ok(LootTablesLootTables::Cow {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "CREEPER" => Ok(LootTablesLootTables::Creeper {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "DOLPHIN" => Ok(LootTablesLootTables::Dolphin {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "DONKEY" => Ok(LootTablesLootTables::Donkey {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "DROWNED" => Ok(LootTablesLootTables::Drowned {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ELDER_GUARDIAN" => Ok(LootTablesLootTables::ElderGuardian {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ENDER_DRAGON" => Ok(LootTablesLootTables::EnderDragon {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ENDERMAN" => Ok(LootTablesLootTables::Enderman {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ENDERMITE" => Ok(LootTablesLootTables::Endermite {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "EVOKER" => Ok(LootTablesLootTables::Evoker {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "FOX" => Ok(LootTablesLootTables::Fox {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "GHAST" => Ok(LootTablesLootTables::Ghast {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "GIANT" => Ok(LootTablesLootTables::Giant {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "GLOW_SQUID" => Ok(LootTablesLootTables::GlowSquid {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "GOAT" => Ok(LootTablesLootTables::Goat {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "GUARDIAN" => Ok(LootTablesLootTables::Guardian {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "HOGLIN" => Ok(LootTablesLootTables::Hoglin {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "HORSE" => Ok(LootTablesLootTables::Horse {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "HUSK" => Ok(LootTablesLootTables::Husk {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ILLUSIONER" => Ok(LootTablesLootTables::Illusioner {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "IRON_GOLEM" => Ok(LootTablesLootTables::IronGolem {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "LLAMA" => Ok(LootTablesLootTables::Llama {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "MAGMA_CUBE" => Ok(LootTablesLootTables::MagmaCube {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "MOOSHROOM" => Ok(LootTablesLootTables::Mooshroom {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "MULE" => Ok(LootTablesLootTables::Mule {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "OCELOT" => Ok(LootTablesLootTables::Ocelot {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "PANDA" => Ok(LootTablesLootTables::Panda {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "PARROT" => Ok(LootTablesLootTables::Parrot {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "PHANTOM" => Ok(LootTablesLootTables::Phantom {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "PIG" => Ok(LootTablesLootTables::Pig {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "PIGLIN" => Ok(LootTablesLootTables::Piglin {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_BRUTE" => Ok(LootTablesLootTables::PiglinBrute {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "PILLAGER" => Ok(LootTablesLootTables::Pillager {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "PLAYER" => Ok(LootTablesLootTables::Player {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "POLAR_BEAR" => Ok(LootTablesLootTables::PolarBear {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "PUFFERFISH" => Ok(LootTablesLootTables::Pufferfish {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "RABBIT" => Ok(LootTablesLootTables::Rabbit {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "RAVAGER" => Ok(LootTablesLootTables::Ravager {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SALMON" => Ok(LootTablesLootTables::Salmon {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHULKER" => Ok(LootTablesLootTables::Shulker {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SILVERFISH" => Ok(LootTablesLootTables::Silverfish {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SKELETON" => Ok(LootTablesLootTables::Skeleton {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SKELETON_HORSE" => Ok(LootTablesLootTables::SkeletonHorse {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SLIME" => Ok(LootTablesLootTables::Slime {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SNOW_GOLEM" => Ok(LootTablesLootTables::SnowGolem {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SPIDER" => Ok(LootTablesLootTables::Spider {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SQUID" => Ok(LootTablesLootTables::Squid {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "STRAY" => Ok(LootTablesLootTables::Stray {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "STRIDER" => Ok(LootTablesLootTables::Strider {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "TRADER_LLAMA" => Ok(LootTablesLootTables::TraderLlama {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "TROPICAL_FISH" => Ok(LootTablesLootTables::TropicalFish {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "TURTLE" => Ok(LootTablesLootTables::Turtle {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VEX" => Ok(LootTablesLootTables::Vex {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGER" => Ok(LootTablesLootTables::Villager {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "VINDICATOR" => Ok(LootTablesLootTables::Vindicator {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "WANDERING_TRADER" => Ok(LootTablesLootTables::WanderingTrader {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "WITCH" => Ok(LootTablesLootTables::Witch {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "WITHER" => Ok(LootTablesLootTables::Wither {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "WITHER_SKELETON" => Ok(LootTablesLootTables::WitherSkeleton {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "WOLF" => Ok(LootTablesLootTables::Wolf {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOGLIN" => Ok(LootTablesLootTables::Zoglin {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIE" => Ok(LootTablesLootTables::Zombie {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIE_HORSE" => Ok(LootTablesLootTables::ZombieHorse {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIE_VILLAGER" => Ok(LootTablesLootTables::ZombieVillager {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIFIED_PIGLIN" => Ok(LootTablesLootTables::ZombifiedPiglin {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "ARMORER_GIFT" => Ok(LootTablesLootTables::ArmorerGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "BUTCHER_GIFT" => Ok(LootTablesLootTables::ButcherGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "CARTOGRAPHER_GIFT" => Ok(LootTablesLootTables::CartographerGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "CAT_MORNING_GIFT" => Ok(LootTablesLootTables::CatMorningGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "CLERIC_GIFT" => Ok(LootTablesLootTables::ClericGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "FARMER_GIFT" => Ok(LootTablesLootTables::FarmerGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHERMAN_GIFT" => Ok(LootTablesLootTables::FishermanGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING" => Ok(LootTablesLootTables::Fishing {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING_FISH" => Ok(LootTablesLootTables::FishingFish {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING_JUNK" => Ok(LootTablesLootTables::FishingJunk {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING_TREASURE" => Ok(LootTablesLootTables::FishingTreasure {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "FLETCHER_GIFT" => Ok(LootTablesLootTables::FletcherGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "LEATHERWORKER_GIFT" => Ok(LootTablesLootTables::LeatherworkerGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "LIBRARIAN_GIFT" => Ok(LootTablesLootTables::LibrarianGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "MASON_GIFT" => Ok(LootTablesLootTables::MasonGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEPHERD_GIFT" => Ok(LootTablesLootTables::ShepherdGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "TOOLSMITH_GIFT" => Ok(LootTablesLootTables::ToolsmithGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "WEAPONSMITH_GIFT" => Ok(LootTablesLootTables::WeaponsmithGift {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SNIFFER_DIGGING" => Ok(LootTablesLootTables::SnifferDigging {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_BARTERING" => Ok(LootTablesLootTables::PiglinBartering {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "DESERT_WELL_ARCHAEOLOGY" => Ok(LootTablesLootTables::DesertWellArchaeology {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "DESERT_PYRAMID_ARCHAEOLOGY" => {
                    Ok(LootTablesLootTables::DesertPyramidArchaeology {
                        inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "TRAIL_RUINS_ARCHAEOLOGY_COMMON" => {
                    Ok(LootTablesLootTables::TrailRuinsArchaeologyCommon {
                        inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "TRAIL_RUINS_ARCHAEOLOGY_RARE" => {
                    Ok(LootTablesLootTables::TrailRuinsArchaeologyRare {
                        inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "OCEAN_RUIN_WARM_ARCHAEOLOGY" => {
                    Ok(LootTablesLootTables::OceanRuinWarmArchaeology {
                        inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "OCEAN_RUIN_COLD_ARCHAEOLOGY" => {
                    Ok(LootTablesLootTables::OceanRuinColdArchaeology {
                        inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "SHEEP" => Ok(LootTablesLootTables::Sheep {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_BLACK" => Ok(LootTablesLootTables::SheepBlack {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_BLUE" => Ok(LootTablesLootTables::SheepBlue {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_BROWN" => Ok(LootTablesLootTables::SheepBrown {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_CYAN" => Ok(LootTablesLootTables::SheepCyan {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_GRAY" => Ok(LootTablesLootTables::SheepGray {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_GREEN" => Ok(LootTablesLootTables::SheepGreen {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_LIGHT_BLUE" => Ok(LootTablesLootTables::SheepLightBlue {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_LIGHT_GRAY" => Ok(LootTablesLootTables::SheepLightGray {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_LIME" => Ok(LootTablesLootTables::SheepLime {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_MAGENTA" => Ok(LootTablesLootTables::SheepMagenta {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_ORANGE" => Ok(LootTablesLootTables::SheepOrange {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_PINK" => Ok(LootTablesLootTables::SheepPink {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_PURPLE" => Ok(LootTablesLootTables::SheepPurple {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_RED" => Ok(LootTablesLootTables::SheepRed {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_WHITE" => Ok(LootTablesLootTables::SheepWhite {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_YELLOW" => Ok(LootTablesLootTables::SheepYellow {
                    inner: LootTablesLootTablesStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for LootTablesLootTablesStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootTablesLootTablesStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate LootTablesLootTablesStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootTables$LootTables")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootTablesLootTablesStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LootTablesLootTablesStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents a <a href="../block/Container.html" title="interface in org.bukkit.block"><code>Container</code></a> or a <a title="interface in org.bukkit.entity" href="../entity/Mob.html"><code>Mob</code></a> that can have a loot table.
///
/// Container loot will only generate upon opening, and only when the container is <i>first</i> opened.
///
/// Entities will only generate loot upon death.
///
/// This is a representation of an abstract class.
#[repr(C)]
pub struct Lootable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Lootable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Lootable<'mc> {
    fn from_raw(
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
}

impl<'mc> Lootable<'mc> {
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }
    /// Set the seed used when this Loot Table generates loot.
    pub fn set_seed(&self, arg0: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(arg0);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_loot_table(
        &self,
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

    pub fn loot_table(
        &self,
    ) -> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootTable;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootTable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::loot::LootTable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
pub enum LootTables<'mc> {
    Empty { inner: LootTablesStruct<'mc> },
    AbandonedMineshaft { inner: LootTablesStruct<'mc> },
    BuriedTreasure { inner: LootTablesStruct<'mc> },
    DesertPyramid { inner: LootTablesStruct<'mc> },
    EndCityTreasure { inner: LootTablesStruct<'mc> },
    IglooChest { inner: LootTablesStruct<'mc> },
    JungleTemple { inner: LootTablesStruct<'mc> },
    JungleTempleDispenser { inner: LootTablesStruct<'mc> },
    NetherBridge { inner: LootTablesStruct<'mc> },
    PillagerOutpost { inner: LootTablesStruct<'mc> },
    BastionTreasure { inner: LootTablesStruct<'mc> },
    BastionOther { inner: LootTablesStruct<'mc> },
    BastionBridge { inner: LootTablesStruct<'mc> },
    BastionHoglinStable { inner: LootTablesStruct<'mc> },
    AncientCity { inner: LootTablesStruct<'mc> },
    AncientCityIceBox { inner: LootTablesStruct<'mc> },
    RuinedPortal { inner: LootTablesStruct<'mc> },
    ShipwreckMap { inner: LootTablesStruct<'mc> },
    ShipwreckSupply { inner: LootTablesStruct<'mc> },
    ShipwreckTreasure { inner: LootTablesStruct<'mc> },
    SimpleDungeon { inner: LootTablesStruct<'mc> },
    SpawnBonusChest { inner: LootTablesStruct<'mc> },
    StrongholdCorridor { inner: LootTablesStruct<'mc> },
    StrongholdCrossing { inner: LootTablesStruct<'mc> },
    StrongholdLibrary { inner: LootTablesStruct<'mc> },
    UnderwaterRuinBig { inner: LootTablesStruct<'mc> },
    UnderwaterRuinSmall { inner: LootTablesStruct<'mc> },
    VillageArmorer { inner: LootTablesStruct<'mc> },
    VillageButcher { inner: LootTablesStruct<'mc> },
    VillageCartographer { inner: LootTablesStruct<'mc> },
    VillageDesertHouse { inner: LootTablesStruct<'mc> },
    VillageFisher { inner: LootTablesStruct<'mc> },
    VillageFletcher { inner: LootTablesStruct<'mc> },
    VillageMason { inner: LootTablesStruct<'mc> },
    VillagePlainsHouse { inner: LootTablesStruct<'mc> },
    VillageSavannaHouse { inner: LootTablesStruct<'mc> },
    VillageShepherd { inner: LootTablesStruct<'mc> },
    VillageSnowyHouse { inner: LootTablesStruct<'mc> },
    VillageTaigaHouse { inner: LootTablesStruct<'mc> },
    VillageTannery { inner: LootTablesStruct<'mc> },
    VillageTemple { inner: LootTablesStruct<'mc> },
    VillageToolsmith { inner: LootTablesStruct<'mc> },
    VillageWeaponsmith { inner: LootTablesStruct<'mc> },
    WoodlandMansion { inner: LootTablesStruct<'mc> },
    ArmorStand { inner: LootTablesStruct<'mc> },
    Axolotl { inner: LootTablesStruct<'mc> },
    Bat { inner: LootTablesStruct<'mc> },
    Bee { inner: LootTablesStruct<'mc> },
    Blaze { inner: LootTablesStruct<'mc> },
    Cat { inner: LootTablesStruct<'mc> },
    CaveSpider { inner: LootTablesStruct<'mc> },
    Chicken { inner: LootTablesStruct<'mc> },
    Cod { inner: LootTablesStruct<'mc> },
    Cow { inner: LootTablesStruct<'mc> },
    Creeper { inner: LootTablesStruct<'mc> },
    Dolphin { inner: LootTablesStruct<'mc> },
    Donkey { inner: LootTablesStruct<'mc> },
    Drowned { inner: LootTablesStruct<'mc> },
    ElderGuardian { inner: LootTablesStruct<'mc> },
    EnderDragon { inner: LootTablesStruct<'mc> },
    Enderman { inner: LootTablesStruct<'mc> },
    Endermite { inner: LootTablesStruct<'mc> },
    Evoker { inner: LootTablesStruct<'mc> },
    Fox { inner: LootTablesStruct<'mc> },
    Ghast { inner: LootTablesStruct<'mc> },
    Giant { inner: LootTablesStruct<'mc> },
    GlowSquid { inner: LootTablesStruct<'mc> },
    Goat { inner: LootTablesStruct<'mc> },
    Guardian { inner: LootTablesStruct<'mc> },
    Hoglin { inner: LootTablesStruct<'mc> },
    Horse { inner: LootTablesStruct<'mc> },
    Husk { inner: LootTablesStruct<'mc> },
    Illusioner { inner: LootTablesStruct<'mc> },
    IronGolem { inner: LootTablesStruct<'mc> },
    Llama { inner: LootTablesStruct<'mc> },
    MagmaCube { inner: LootTablesStruct<'mc> },
    Mooshroom { inner: LootTablesStruct<'mc> },
    Mule { inner: LootTablesStruct<'mc> },
    Ocelot { inner: LootTablesStruct<'mc> },
    Panda { inner: LootTablesStruct<'mc> },
    Parrot { inner: LootTablesStruct<'mc> },
    Phantom { inner: LootTablesStruct<'mc> },
    Pig { inner: LootTablesStruct<'mc> },
    Piglin { inner: LootTablesStruct<'mc> },
    PiglinBrute { inner: LootTablesStruct<'mc> },
    Pillager { inner: LootTablesStruct<'mc> },
    Player { inner: LootTablesStruct<'mc> },
    PolarBear { inner: LootTablesStruct<'mc> },
    Pufferfish { inner: LootTablesStruct<'mc> },
    Rabbit { inner: LootTablesStruct<'mc> },
    Ravager { inner: LootTablesStruct<'mc> },
    Salmon { inner: LootTablesStruct<'mc> },
    Shulker { inner: LootTablesStruct<'mc> },
    Silverfish { inner: LootTablesStruct<'mc> },
    Skeleton { inner: LootTablesStruct<'mc> },
    SkeletonHorse { inner: LootTablesStruct<'mc> },
    Slime { inner: LootTablesStruct<'mc> },
    SnowGolem { inner: LootTablesStruct<'mc> },
    Spider { inner: LootTablesStruct<'mc> },
    Squid { inner: LootTablesStruct<'mc> },
    Stray { inner: LootTablesStruct<'mc> },
    Strider { inner: LootTablesStruct<'mc> },
    TraderLlama { inner: LootTablesStruct<'mc> },
    TropicalFish { inner: LootTablesStruct<'mc> },
    Turtle { inner: LootTablesStruct<'mc> },
    Vex { inner: LootTablesStruct<'mc> },
    Villager { inner: LootTablesStruct<'mc> },
    Vindicator { inner: LootTablesStruct<'mc> },
    WanderingTrader { inner: LootTablesStruct<'mc> },
    Witch { inner: LootTablesStruct<'mc> },
    Wither { inner: LootTablesStruct<'mc> },
    WitherSkeleton { inner: LootTablesStruct<'mc> },
    Wolf { inner: LootTablesStruct<'mc> },
    Zoglin { inner: LootTablesStruct<'mc> },
    Zombie { inner: LootTablesStruct<'mc> },
    ZombieHorse { inner: LootTablesStruct<'mc> },
    ZombieVillager { inner: LootTablesStruct<'mc> },
    ZombifiedPiglin { inner: LootTablesStruct<'mc> },
    ArmorerGift { inner: LootTablesStruct<'mc> },
    ButcherGift { inner: LootTablesStruct<'mc> },
    CartographerGift { inner: LootTablesStruct<'mc> },
    CatMorningGift { inner: LootTablesStruct<'mc> },
    ClericGift { inner: LootTablesStruct<'mc> },
    FarmerGift { inner: LootTablesStruct<'mc> },
    FishermanGift { inner: LootTablesStruct<'mc> },
    Fishing { inner: LootTablesStruct<'mc> },
    FishingFish { inner: LootTablesStruct<'mc> },
    FishingJunk { inner: LootTablesStruct<'mc> },
    FishingTreasure { inner: LootTablesStruct<'mc> },
    FletcherGift { inner: LootTablesStruct<'mc> },
    LeatherworkerGift { inner: LootTablesStruct<'mc> },
    LibrarianGift { inner: LootTablesStruct<'mc> },
    MasonGift { inner: LootTablesStruct<'mc> },
    ShepherdGift { inner: LootTablesStruct<'mc> },
    ToolsmithGift { inner: LootTablesStruct<'mc> },
    WeaponsmithGift { inner: LootTablesStruct<'mc> },
    SnifferDigging { inner: LootTablesStruct<'mc> },
    PiglinBartering { inner: LootTablesStruct<'mc> },
    DesertWellArchaeology { inner: LootTablesStruct<'mc> },
    DesertPyramidArchaeology { inner: LootTablesStruct<'mc> },
    TrailRuinsArchaeologyCommon { inner: LootTablesStruct<'mc> },
    TrailRuinsArchaeologyRare { inner: LootTablesStruct<'mc> },
    OceanRuinWarmArchaeology { inner: LootTablesStruct<'mc> },
    OceanRuinColdArchaeology { inner: LootTablesStruct<'mc> },
    Sheep { inner: LootTablesStruct<'mc> },
    SheepBlack { inner: LootTablesStruct<'mc> },
    SheepBlue { inner: LootTablesStruct<'mc> },
    SheepBrown { inner: LootTablesStruct<'mc> },
    SheepCyan { inner: LootTablesStruct<'mc> },
    SheepGray { inner: LootTablesStruct<'mc> },
    SheepGreen { inner: LootTablesStruct<'mc> },
    SheepLightBlue { inner: LootTablesStruct<'mc> },
    SheepLightGray { inner: LootTablesStruct<'mc> },
    SheepLime { inner: LootTablesStruct<'mc> },
    SheepMagenta { inner: LootTablesStruct<'mc> },
    SheepOrange { inner: LootTablesStruct<'mc> },
    SheepPink { inner: LootTablesStruct<'mc> },
    SheepPurple { inner: LootTablesStruct<'mc> },
    SheepRed { inner: LootTablesStruct<'mc> },
    SheepWhite { inner: LootTablesStruct<'mc> },
    SheepYellow { inner: LootTablesStruct<'mc> },
}
impl<'mc> std::fmt::Display for LootTables<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LootTables::Empty { .. } => f.write_str("EMPTY"),
            LootTables::AbandonedMineshaft { .. } => f.write_str("ABANDONED_MINESHAFT"),
            LootTables::BuriedTreasure { .. } => f.write_str("BURIED_TREASURE"),
            LootTables::DesertPyramid { .. } => f.write_str("DESERT_PYRAMID"),
            LootTables::EndCityTreasure { .. } => f.write_str("END_CITY_TREASURE"),
            LootTables::IglooChest { .. } => f.write_str("IGLOO_CHEST"),
            LootTables::JungleTemple { .. } => f.write_str("JUNGLE_TEMPLE"),
            LootTables::JungleTempleDispenser { .. } => f.write_str("JUNGLE_TEMPLE_DISPENSER"),
            LootTables::NetherBridge { .. } => f.write_str("NETHER_BRIDGE"),
            LootTables::PillagerOutpost { .. } => f.write_str("PILLAGER_OUTPOST"),
            LootTables::BastionTreasure { .. } => f.write_str("BASTION_TREASURE"),
            LootTables::BastionOther { .. } => f.write_str("BASTION_OTHER"),
            LootTables::BastionBridge { .. } => f.write_str("BASTION_BRIDGE"),
            LootTables::BastionHoglinStable { .. } => f.write_str("BASTION_HOGLIN_STABLE"),
            LootTables::AncientCity { .. } => f.write_str("ANCIENT_CITY"),
            LootTables::AncientCityIceBox { .. } => f.write_str("ANCIENT_CITY_ICE_BOX"),
            LootTables::RuinedPortal { .. } => f.write_str("RUINED_PORTAL"),
            LootTables::ShipwreckMap { .. } => f.write_str("SHIPWRECK_MAP"),
            LootTables::ShipwreckSupply { .. } => f.write_str("SHIPWRECK_SUPPLY"),
            LootTables::ShipwreckTreasure { .. } => f.write_str("SHIPWRECK_TREASURE"),
            LootTables::SimpleDungeon { .. } => f.write_str("SIMPLE_DUNGEON"),
            LootTables::SpawnBonusChest { .. } => f.write_str("SPAWN_BONUS_CHEST"),
            LootTables::StrongholdCorridor { .. } => f.write_str("STRONGHOLD_CORRIDOR"),
            LootTables::StrongholdCrossing { .. } => f.write_str("STRONGHOLD_CROSSING"),
            LootTables::StrongholdLibrary { .. } => f.write_str("STRONGHOLD_LIBRARY"),
            LootTables::UnderwaterRuinBig { .. } => f.write_str("UNDERWATER_RUIN_BIG"),
            LootTables::UnderwaterRuinSmall { .. } => f.write_str("UNDERWATER_RUIN_SMALL"),
            LootTables::VillageArmorer { .. } => f.write_str("VILLAGE_ARMORER"),
            LootTables::VillageButcher { .. } => f.write_str("VILLAGE_BUTCHER"),
            LootTables::VillageCartographer { .. } => f.write_str("VILLAGE_CARTOGRAPHER"),
            LootTables::VillageDesertHouse { .. } => f.write_str("VILLAGE_DESERT_HOUSE"),
            LootTables::VillageFisher { .. } => f.write_str("VILLAGE_FISHER"),
            LootTables::VillageFletcher { .. } => f.write_str("VILLAGE_FLETCHER"),
            LootTables::VillageMason { .. } => f.write_str("VILLAGE_MASON"),
            LootTables::VillagePlainsHouse { .. } => f.write_str("VILLAGE_PLAINS_HOUSE"),
            LootTables::VillageSavannaHouse { .. } => f.write_str("VILLAGE_SAVANNA_HOUSE"),
            LootTables::VillageShepherd { .. } => f.write_str("VILLAGE_SHEPHERD"),
            LootTables::VillageSnowyHouse { .. } => f.write_str("VILLAGE_SNOWY_HOUSE"),
            LootTables::VillageTaigaHouse { .. } => f.write_str("VILLAGE_TAIGA_HOUSE"),
            LootTables::VillageTannery { .. } => f.write_str("VILLAGE_TANNERY"),
            LootTables::VillageTemple { .. } => f.write_str("VILLAGE_TEMPLE"),
            LootTables::VillageToolsmith { .. } => f.write_str("VILLAGE_TOOLSMITH"),
            LootTables::VillageWeaponsmith { .. } => f.write_str("VILLAGE_WEAPONSMITH"),
            LootTables::WoodlandMansion { .. } => f.write_str("WOODLAND_MANSION"),
            LootTables::ArmorStand { .. } => f.write_str("ARMOR_STAND"),
            LootTables::Axolotl { .. } => f.write_str("AXOLOTL"),
            LootTables::Bat { .. } => f.write_str("BAT"),
            LootTables::Bee { .. } => f.write_str("BEE"),
            LootTables::Blaze { .. } => f.write_str("BLAZE"),
            LootTables::Cat { .. } => f.write_str("CAT"),
            LootTables::CaveSpider { .. } => f.write_str("CAVE_SPIDER"),
            LootTables::Chicken { .. } => f.write_str("CHICKEN"),
            LootTables::Cod { .. } => f.write_str("COD"),
            LootTables::Cow { .. } => f.write_str("COW"),
            LootTables::Creeper { .. } => f.write_str("CREEPER"),
            LootTables::Dolphin { .. } => f.write_str("DOLPHIN"),
            LootTables::Donkey { .. } => f.write_str("DONKEY"),
            LootTables::Drowned { .. } => f.write_str("DROWNED"),
            LootTables::ElderGuardian { .. } => f.write_str("ELDER_GUARDIAN"),
            LootTables::EnderDragon { .. } => f.write_str("ENDER_DRAGON"),
            LootTables::Enderman { .. } => f.write_str("ENDERMAN"),
            LootTables::Endermite { .. } => f.write_str("ENDERMITE"),
            LootTables::Evoker { .. } => f.write_str("EVOKER"),
            LootTables::Fox { .. } => f.write_str("FOX"),
            LootTables::Ghast { .. } => f.write_str("GHAST"),
            LootTables::Giant { .. } => f.write_str("GIANT"),
            LootTables::GlowSquid { .. } => f.write_str("GLOW_SQUID"),
            LootTables::Goat { .. } => f.write_str("GOAT"),
            LootTables::Guardian { .. } => f.write_str("GUARDIAN"),
            LootTables::Hoglin { .. } => f.write_str("HOGLIN"),
            LootTables::Horse { .. } => f.write_str("HORSE"),
            LootTables::Husk { .. } => f.write_str("HUSK"),
            LootTables::Illusioner { .. } => f.write_str("ILLUSIONER"),
            LootTables::IronGolem { .. } => f.write_str("IRON_GOLEM"),
            LootTables::Llama { .. } => f.write_str("LLAMA"),
            LootTables::MagmaCube { .. } => f.write_str("MAGMA_CUBE"),
            LootTables::Mooshroom { .. } => f.write_str("MOOSHROOM"),
            LootTables::Mule { .. } => f.write_str("MULE"),
            LootTables::Ocelot { .. } => f.write_str("OCELOT"),
            LootTables::Panda { .. } => f.write_str("PANDA"),
            LootTables::Parrot { .. } => f.write_str("PARROT"),
            LootTables::Phantom { .. } => f.write_str("PHANTOM"),
            LootTables::Pig { .. } => f.write_str("PIG"),
            LootTables::Piglin { .. } => f.write_str("PIGLIN"),
            LootTables::PiglinBrute { .. } => f.write_str("PIGLIN_BRUTE"),
            LootTables::Pillager { .. } => f.write_str("PILLAGER"),
            LootTables::Player { .. } => f.write_str("PLAYER"),
            LootTables::PolarBear { .. } => f.write_str("POLAR_BEAR"),
            LootTables::Pufferfish { .. } => f.write_str("PUFFERFISH"),
            LootTables::Rabbit { .. } => f.write_str("RABBIT"),
            LootTables::Ravager { .. } => f.write_str("RAVAGER"),
            LootTables::Salmon { .. } => f.write_str("SALMON"),
            LootTables::Shulker { .. } => f.write_str("SHULKER"),
            LootTables::Silverfish { .. } => f.write_str("SILVERFISH"),
            LootTables::Skeleton { .. } => f.write_str("SKELETON"),
            LootTables::SkeletonHorse { .. } => f.write_str("SKELETON_HORSE"),
            LootTables::Slime { .. } => f.write_str("SLIME"),
            LootTables::SnowGolem { .. } => f.write_str("SNOW_GOLEM"),
            LootTables::Spider { .. } => f.write_str("SPIDER"),
            LootTables::Squid { .. } => f.write_str("SQUID"),
            LootTables::Stray { .. } => f.write_str("STRAY"),
            LootTables::Strider { .. } => f.write_str("STRIDER"),
            LootTables::TraderLlama { .. } => f.write_str("TRADER_LLAMA"),
            LootTables::TropicalFish { .. } => f.write_str("TROPICAL_FISH"),
            LootTables::Turtle { .. } => f.write_str("TURTLE"),
            LootTables::Vex { .. } => f.write_str("VEX"),
            LootTables::Villager { .. } => f.write_str("VILLAGER"),
            LootTables::Vindicator { .. } => f.write_str("VINDICATOR"),
            LootTables::WanderingTrader { .. } => f.write_str("WANDERING_TRADER"),
            LootTables::Witch { .. } => f.write_str("WITCH"),
            LootTables::Wither { .. } => f.write_str("WITHER"),
            LootTables::WitherSkeleton { .. } => f.write_str("WITHER_SKELETON"),
            LootTables::Wolf { .. } => f.write_str("WOLF"),
            LootTables::Zoglin { .. } => f.write_str("ZOGLIN"),
            LootTables::Zombie { .. } => f.write_str("ZOMBIE"),
            LootTables::ZombieHorse { .. } => f.write_str("ZOMBIE_HORSE"),
            LootTables::ZombieVillager { .. } => f.write_str("ZOMBIE_VILLAGER"),
            LootTables::ZombifiedPiglin { .. } => f.write_str("ZOMBIFIED_PIGLIN"),
            LootTables::ArmorerGift { .. } => f.write_str("ARMORER_GIFT"),
            LootTables::ButcherGift { .. } => f.write_str("BUTCHER_GIFT"),
            LootTables::CartographerGift { .. } => f.write_str("CARTOGRAPHER_GIFT"),
            LootTables::CatMorningGift { .. } => f.write_str("CAT_MORNING_GIFT"),
            LootTables::ClericGift { .. } => f.write_str("CLERIC_GIFT"),
            LootTables::FarmerGift { .. } => f.write_str("FARMER_GIFT"),
            LootTables::FishermanGift { .. } => f.write_str("FISHERMAN_GIFT"),
            LootTables::Fishing { .. } => f.write_str("FISHING"),
            LootTables::FishingFish { .. } => f.write_str("FISHING_FISH"),
            LootTables::FishingJunk { .. } => f.write_str("FISHING_JUNK"),
            LootTables::FishingTreasure { .. } => f.write_str("FISHING_TREASURE"),
            LootTables::FletcherGift { .. } => f.write_str("FLETCHER_GIFT"),
            LootTables::LeatherworkerGift { .. } => f.write_str("LEATHERWORKER_GIFT"),
            LootTables::LibrarianGift { .. } => f.write_str("LIBRARIAN_GIFT"),
            LootTables::MasonGift { .. } => f.write_str("MASON_GIFT"),
            LootTables::ShepherdGift { .. } => f.write_str("SHEPHERD_GIFT"),
            LootTables::ToolsmithGift { .. } => f.write_str("TOOLSMITH_GIFT"),
            LootTables::WeaponsmithGift { .. } => f.write_str("WEAPONSMITH_GIFT"),
            LootTables::SnifferDigging { .. } => f.write_str("SNIFFER_DIGGING"),
            LootTables::PiglinBartering { .. } => f.write_str("PIGLIN_BARTERING"),
            LootTables::DesertWellArchaeology { .. } => f.write_str("DESERT_WELL_ARCHAEOLOGY"),
            LootTables::DesertPyramidArchaeology { .. } => {
                f.write_str("DESERT_PYRAMID_ARCHAEOLOGY")
            }
            LootTables::TrailRuinsArchaeologyCommon { .. } => {
                f.write_str("TRAIL_RUINS_ARCHAEOLOGY_COMMON")
            }
            LootTables::TrailRuinsArchaeologyRare { .. } => {
                f.write_str("TRAIL_RUINS_ARCHAEOLOGY_RARE")
            }
            LootTables::OceanRuinWarmArchaeology { .. } => {
                f.write_str("OCEAN_RUIN_WARM_ARCHAEOLOGY")
            }
            LootTables::OceanRuinColdArchaeology { .. } => {
                f.write_str("OCEAN_RUIN_COLD_ARCHAEOLOGY")
            }
            LootTables::Sheep { .. } => f.write_str("SHEEP"),
            LootTables::SheepBlack { .. } => f.write_str("SHEEP_BLACK"),
            LootTables::SheepBlue { .. } => f.write_str("SHEEP_BLUE"),
            LootTables::SheepBrown { .. } => f.write_str("SHEEP_BROWN"),
            LootTables::SheepCyan { .. } => f.write_str("SHEEP_CYAN"),
            LootTables::SheepGray { .. } => f.write_str("SHEEP_GRAY"),
            LootTables::SheepGreen { .. } => f.write_str("SHEEP_GREEN"),
            LootTables::SheepLightBlue { .. } => f.write_str("SHEEP_LIGHT_BLUE"),
            LootTables::SheepLightGray { .. } => f.write_str("SHEEP_LIGHT_GRAY"),
            LootTables::SheepLime { .. } => f.write_str("SHEEP_LIME"),
            LootTables::SheepMagenta { .. } => f.write_str("SHEEP_MAGENTA"),
            LootTables::SheepOrange { .. } => f.write_str("SHEEP_ORANGE"),
            LootTables::SheepPink { .. } => f.write_str("SHEEP_PINK"),
            LootTables::SheepPurple { .. } => f.write_str("SHEEP_PURPLE"),
            LootTables::SheepRed { .. } => f.write_str("SHEEP_RED"),
            LootTables::SheepWhite { .. } => f.write_str("SHEEP_WHITE"),
            LootTables::SheepYellow { .. } => f.write_str("SHEEP_YELLOW"),
        }
    }
}

impl<'mc> LootTables<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<LootTables<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/loot/LootTables");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/loot/LootTables;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "EMPTY" => Ok(LootTables::Empty {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ABANDONED_MINESHAFT" => Ok(LootTables::AbandonedMineshaft {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BURIED_TREASURE" => Ok(LootTables::BuriedTreasure {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DESERT_PYRAMID" => Ok(LootTables::DesertPyramid {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "END_CITY_TREASURE" => Ok(LootTables::EndCityTreasure {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "IGLOO_CHEST" => Ok(LootTables::IglooChest {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "JUNGLE_TEMPLE" => Ok(LootTables::JungleTemple {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "JUNGLE_TEMPLE_DISPENSER" => Ok(LootTables::JungleTempleDispenser {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "NETHER_BRIDGE" => Ok(LootTables::NetherBridge {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PILLAGER_OUTPOST" => Ok(LootTables::PillagerOutpost {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_TREASURE" => Ok(LootTables::BastionTreasure {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_OTHER" => Ok(LootTables::BastionOther {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_BRIDGE" => Ok(LootTables::BastionBridge {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_HOGLIN_STABLE" => Ok(LootTables::BastionHoglinStable {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ANCIENT_CITY" => Ok(LootTables::AncientCity {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ANCIENT_CITY_ICE_BOX" => Ok(LootTables::AncientCityIceBox {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "RUINED_PORTAL" => Ok(LootTables::RuinedPortal {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHIPWRECK_MAP" => Ok(LootTables::ShipwreckMap {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHIPWRECK_SUPPLY" => Ok(LootTables::ShipwreckSupply {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHIPWRECK_TREASURE" => Ok(LootTables::ShipwreckTreasure {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SIMPLE_DUNGEON" => Ok(LootTables::SimpleDungeon {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SPAWN_BONUS_CHEST" => Ok(LootTables::SpawnBonusChest {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "STRONGHOLD_CORRIDOR" => Ok(LootTables::StrongholdCorridor {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "STRONGHOLD_CROSSING" => Ok(LootTables::StrongholdCrossing {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "STRONGHOLD_LIBRARY" => Ok(LootTables::StrongholdLibrary {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "UNDERWATER_RUIN_BIG" => Ok(LootTables::UnderwaterRuinBig {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "UNDERWATER_RUIN_SMALL" => Ok(LootTables::UnderwaterRuinSmall {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_ARMORER" => Ok(LootTables::VillageArmorer {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_BUTCHER" => Ok(LootTables::VillageButcher {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_CARTOGRAPHER" => Ok(LootTables::VillageCartographer {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_DESERT_HOUSE" => Ok(LootTables::VillageDesertHouse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_FISHER" => Ok(LootTables::VillageFisher {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_FLETCHER" => Ok(LootTables::VillageFletcher {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_MASON" => Ok(LootTables::VillageMason {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_PLAINS_HOUSE" => Ok(LootTables::VillagePlainsHouse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_SAVANNA_HOUSE" => Ok(LootTables::VillageSavannaHouse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_SHEPHERD" => Ok(LootTables::VillageShepherd {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_SNOWY_HOUSE" => Ok(LootTables::VillageSnowyHouse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TAIGA_HOUSE" => Ok(LootTables::VillageTaigaHouse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TANNERY" => Ok(LootTables::VillageTannery {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TEMPLE" => Ok(LootTables::VillageTemple {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TOOLSMITH" => Ok(LootTables::VillageToolsmith {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_WEAPONSMITH" => Ok(LootTables::VillageWeaponsmith {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WOODLAND_MANSION" => Ok(LootTables::WoodlandMansion {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ARMOR_STAND" => Ok(LootTables::ArmorStand {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "AXOLOTL" => Ok(LootTables::Axolotl {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BAT" => Ok(LootTables::Bat {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BEE" => Ok(LootTables::Bee {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BLAZE" => Ok(LootTables::Blaze {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CAT" => Ok(LootTables::Cat {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CAVE_SPIDER" => Ok(LootTables::CaveSpider {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CHICKEN" => Ok(LootTables::Chicken {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "COD" => Ok(LootTables::Cod {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "COW" => Ok(LootTables::Cow {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CREEPER" => Ok(LootTables::Creeper {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DOLPHIN" => Ok(LootTables::Dolphin {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DONKEY" => Ok(LootTables::Donkey {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DROWNED" => Ok(LootTables::Drowned {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ELDER_GUARDIAN" => Ok(LootTables::ElderGuardian {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ENDER_DRAGON" => Ok(LootTables::EnderDragon {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ENDERMAN" => Ok(LootTables::Enderman {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ENDERMITE" => Ok(LootTables::Endermite {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "EVOKER" => Ok(LootTables::Evoker {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FOX" => Ok(LootTables::Fox {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "GHAST" => Ok(LootTables::Ghast {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "GIANT" => Ok(LootTables::Giant {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "GLOW_SQUID" => Ok(LootTables::GlowSquid {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "GOAT" => Ok(LootTables::Goat {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "GUARDIAN" => Ok(LootTables::Guardian {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "HOGLIN" => Ok(LootTables::Hoglin {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "HORSE" => Ok(LootTables::Horse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "HUSK" => Ok(LootTables::Husk {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ILLUSIONER" => Ok(LootTables::Illusioner {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "IRON_GOLEM" => Ok(LootTables::IronGolem {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "LLAMA" => Ok(LootTables::Llama {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "MAGMA_CUBE" => Ok(LootTables::MagmaCube {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "MOOSHROOM" => Ok(LootTables::Mooshroom {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "MULE" => Ok(LootTables::Mule {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "OCELOT" => Ok(LootTables::Ocelot {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PANDA" => Ok(LootTables::Panda {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PARROT" => Ok(LootTables::Parrot {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PHANTOM" => Ok(LootTables::Phantom {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PIG" => Ok(LootTables::Pig {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PIGLIN" => Ok(LootTables::Piglin {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_BRUTE" => Ok(LootTables::PiglinBrute {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PILLAGER" => Ok(LootTables::Pillager {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PLAYER" => Ok(LootTables::Player {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "POLAR_BEAR" => Ok(LootTables::PolarBear {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PUFFERFISH" => Ok(LootTables::Pufferfish {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "RABBIT" => Ok(LootTables::Rabbit {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "RAVAGER" => Ok(LootTables::Ravager {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SALMON" => Ok(LootTables::Salmon {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHULKER" => Ok(LootTables::Shulker {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SILVERFISH" => Ok(LootTables::Silverfish {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SKELETON" => Ok(LootTables::Skeleton {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SKELETON_HORSE" => Ok(LootTables::SkeletonHorse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SLIME" => Ok(LootTables::Slime {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SNOW_GOLEM" => Ok(LootTables::SnowGolem {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SPIDER" => Ok(LootTables::Spider {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SQUID" => Ok(LootTables::Squid {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "STRAY" => Ok(LootTables::Stray {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "STRIDER" => Ok(LootTables::Strider {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRADER_LLAMA" => Ok(LootTables::TraderLlama {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TROPICAL_FISH" => Ok(LootTables::TropicalFish {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TURTLE" => Ok(LootTables::Turtle {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VEX" => Ok(LootTables::Vex {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGER" => Ok(LootTables::Villager {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VINDICATOR" => Ok(LootTables::Vindicator {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WANDERING_TRADER" => Ok(LootTables::WanderingTrader {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WITCH" => Ok(LootTables::Witch {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WITHER" => Ok(LootTables::Wither {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WITHER_SKELETON" => Ok(LootTables::WitherSkeleton {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WOLF" => Ok(LootTables::Wolf {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOGLIN" => Ok(LootTables::Zoglin {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIE" => Ok(LootTables::Zombie {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIE_HORSE" => Ok(LootTables::ZombieHorse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIE_VILLAGER" => Ok(LootTables::ZombieVillager {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIFIED_PIGLIN" => Ok(LootTables::ZombifiedPiglin {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ARMORER_GIFT" => Ok(LootTables::ArmorerGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BUTCHER_GIFT" => Ok(LootTables::ButcherGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CARTOGRAPHER_GIFT" => Ok(LootTables::CartographerGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CAT_MORNING_GIFT" => Ok(LootTables::CatMorningGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CLERIC_GIFT" => Ok(LootTables::ClericGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FARMER_GIFT" => Ok(LootTables::FarmerGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHERMAN_GIFT" => Ok(LootTables::FishermanGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING" => Ok(LootTables::Fishing {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING_FISH" => Ok(LootTables::FishingFish {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING_JUNK" => Ok(LootTables::FishingJunk {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING_TREASURE" => Ok(LootTables::FishingTreasure {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FLETCHER_GIFT" => Ok(LootTables::FletcherGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "LEATHERWORKER_GIFT" => Ok(LootTables::LeatherworkerGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "LIBRARIAN_GIFT" => Ok(LootTables::LibrarianGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "MASON_GIFT" => Ok(LootTables::MasonGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEPHERD_GIFT" => Ok(LootTables::ShepherdGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TOOLSMITH_GIFT" => Ok(LootTables::ToolsmithGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WEAPONSMITH_GIFT" => Ok(LootTables::WeaponsmithGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SNIFFER_DIGGING" => Ok(LootTables::SnifferDigging {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_BARTERING" => Ok(LootTables::PiglinBartering {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DESERT_WELL_ARCHAEOLOGY" => Ok(LootTables::DesertWellArchaeology {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DESERT_PYRAMID_ARCHAEOLOGY" => Ok(LootTables::DesertPyramidArchaeology {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRAIL_RUINS_ARCHAEOLOGY_COMMON" => Ok(LootTables::TrailRuinsArchaeologyCommon {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRAIL_RUINS_ARCHAEOLOGY_RARE" => Ok(LootTables::TrailRuinsArchaeologyRare {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "OCEAN_RUIN_WARM_ARCHAEOLOGY" => Ok(LootTables::OceanRuinWarmArchaeology {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "OCEAN_RUIN_COLD_ARCHAEOLOGY" => Ok(LootTables::OceanRuinColdArchaeology {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP" => Ok(LootTables::Sheep {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_BLACK" => Ok(LootTables::SheepBlack {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_BLUE" => Ok(LootTables::SheepBlue {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_BROWN" => Ok(LootTables::SheepBrown {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_CYAN" => Ok(LootTables::SheepCyan {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_GRAY" => Ok(LootTables::SheepGray {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_GREEN" => Ok(LootTables::SheepGreen {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_LIGHT_BLUE" => Ok(LootTables::SheepLightBlue {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_LIGHT_GRAY" => Ok(LootTables::SheepLightGray {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_LIME" => Ok(LootTables::SheepLime {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_MAGENTA" => Ok(LootTables::SheepMagenta {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_ORANGE" => Ok(LootTables::SheepOrange {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_PINK" => Ok(LootTables::SheepPink {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_PURPLE" => Ok(LootTables::SheepPurple {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_RED" => Ok(LootTables::SheepRed {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_WHITE" => Ok(LootTables::SheepWhite {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_YELLOW" => Ok(LootTables::SheepYellow {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct LootTablesStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LootTables<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Empty { inner } => inner.0.clone(),
            Self::AbandonedMineshaft { inner } => inner.0.clone(),
            Self::BuriedTreasure { inner } => inner.0.clone(),
            Self::DesertPyramid { inner } => inner.0.clone(),
            Self::EndCityTreasure { inner } => inner.0.clone(),
            Self::IglooChest { inner } => inner.0.clone(),
            Self::JungleTemple { inner } => inner.0.clone(),
            Self::JungleTempleDispenser { inner } => inner.0.clone(),
            Self::NetherBridge { inner } => inner.0.clone(),
            Self::PillagerOutpost { inner } => inner.0.clone(),
            Self::BastionTreasure { inner } => inner.0.clone(),
            Self::BastionOther { inner } => inner.0.clone(),
            Self::BastionBridge { inner } => inner.0.clone(),
            Self::BastionHoglinStable { inner } => inner.0.clone(),
            Self::AncientCity { inner } => inner.0.clone(),
            Self::AncientCityIceBox { inner } => inner.0.clone(),
            Self::RuinedPortal { inner } => inner.0.clone(),
            Self::ShipwreckMap { inner } => inner.0.clone(),
            Self::ShipwreckSupply { inner } => inner.0.clone(),
            Self::ShipwreckTreasure { inner } => inner.0.clone(),
            Self::SimpleDungeon { inner } => inner.0.clone(),
            Self::SpawnBonusChest { inner } => inner.0.clone(),
            Self::StrongholdCorridor { inner } => inner.0.clone(),
            Self::StrongholdCrossing { inner } => inner.0.clone(),
            Self::StrongholdLibrary { inner } => inner.0.clone(),
            Self::UnderwaterRuinBig { inner } => inner.0.clone(),
            Self::UnderwaterRuinSmall { inner } => inner.0.clone(),
            Self::VillageArmorer { inner } => inner.0.clone(),
            Self::VillageButcher { inner } => inner.0.clone(),
            Self::VillageCartographer { inner } => inner.0.clone(),
            Self::VillageDesertHouse { inner } => inner.0.clone(),
            Self::VillageFisher { inner } => inner.0.clone(),
            Self::VillageFletcher { inner } => inner.0.clone(),
            Self::VillageMason { inner } => inner.0.clone(),
            Self::VillagePlainsHouse { inner } => inner.0.clone(),
            Self::VillageSavannaHouse { inner } => inner.0.clone(),
            Self::VillageShepherd { inner } => inner.0.clone(),
            Self::VillageSnowyHouse { inner } => inner.0.clone(),
            Self::VillageTaigaHouse { inner } => inner.0.clone(),
            Self::VillageTannery { inner } => inner.0.clone(),
            Self::VillageTemple { inner } => inner.0.clone(),
            Self::VillageToolsmith { inner } => inner.0.clone(),
            Self::VillageWeaponsmith { inner } => inner.0.clone(),
            Self::WoodlandMansion { inner } => inner.0.clone(),
            Self::ArmorStand { inner } => inner.0.clone(),
            Self::Axolotl { inner } => inner.0.clone(),
            Self::Bat { inner } => inner.0.clone(),
            Self::Bee { inner } => inner.0.clone(),
            Self::Blaze { inner } => inner.0.clone(),
            Self::Cat { inner } => inner.0.clone(),
            Self::CaveSpider { inner } => inner.0.clone(),
            Self::Chicken { inner } => inner.0.clone(),
            Self::Cod { inner } => inner.0.clone(),
            Self::Cow { inner } => inner.0.clone(),
            Self::Creeper { inner } => inner.0.clone(),
            Self::Dolphin { inner } => inner.0.clone(),
            Self::Donkey { inner } => inner.0.clone(),
            Self::Drowned { inner } => inner.0.clone(),
            Self::ElderGuardian { inner } => inner.0.clone(),
            Self::EnderDragon { inner } => inner.0.clone(),
            Self::Enderman { inner } => inner.0.clone(),
            Self::Endermite { inner } => inner.0.clone(),
            Self::Evoker { inner } => inner.0.clone(),
            Self::Fox { inner } => inner.0.clone(),
            Self::Ghast { inner } => inner.0.clone(),
            Self::Giant { inner } => inner.0.clone(),
            Self::GlowSquid { inner } => inner.0.clone(),
            Self::Goat { inner } => inner.0.clone(),
            Self::Guardian { inner } => inner.0.clone(),
            Self::Hoglin { inner } => inner.0.clone(),
            Self::Horse { inner } => inner.0.clone(),
            Self::Husk { inner } => inner.0.clone(),
            Self::Illusioner { inner } => inner.0.clone(),
            Self::IronGolem { inner } => inner.0.clone(),
            Self::Llama { inner } => inner.0.clone(),
            Self::MagmaCube { inner } => inner.0.clone(),
            Self::Mooshroom { inner } => inner.0.clone(),
            Self::Mule { inner } => inner.0.clone(),
            Self::Ocelot { inner } => inner.0.clone(),
            Self::Panda { inner } => inner.0.clone(),
            Self::Parrot { inner } => inner.0.clone(),
            Self::Phantom { inner } => inner.0.clone(),
            Self::Pig { inner } => inner.0.clone(),
            Self::Piglin { inner } => inner.0.clone(),
            Self::PiglinBrute { inner } => inner.0.clone(),
            Self::Pillager { inner } => inner.0.clone(),
            Self::Player { inner } => inner.0.clone(),
            Self::PolarBear { inner } => inner.0.clone(),
            Self::Pufferfish { inner } => inner.0.clone(),
            Self::Rabbit { inner } => inner.0.clone(),
            Self::Ravager { inner } => inner.0.clone(),
            Self::Salmon { inner } => inner.0.clone(),
            Self::Shulker { inner } => inner.0.clone(),
            Self::Silverfish { inner } => inner.0.clone(),
            Self::Skeleton { inner } => inner.0.clone(),
            Self::SkeletonHorse { inner } => inner.0.clone(),
            Self::Slime { inner } => inner.0.clone(),
            Self::SnowGolem { inner } => inner.0.clone(),
            Self::Spider { inner } => inner.0.clone(),
            Self::Squid { inner } => inner.0.clone(),
            Self::Stray { inner } => inner.0.clone(),
            Self::Strider { inner } => inner.0.clone(),
            Self::TraderLlama { inner } => inner.0.clone(),
            Self::TropicalFish { inner } => inner.0.clone(),
            Self::Turtle { inner } => inner.0.clone(),
            Self::Vex { inner } => inner.0.clone(),
            Self::Villager { inner } => inner.0.clone(),
            Self::Vindicator { inner } => inner.0.clone(),
            Self::WanderingTrader { inner } => inner.0.clone(),
            Self::Witch { inner } => inner.0.clone(),
            Self::Wither { inner } => inner.0.clone(),
            Self::WitherSkeleton { inner } => inner.0.clone(),
            Self::Wolf { inner } => inner.0.clone(),
            Self::Zoglin { inner } => inner.0.clone(),
            Self::Zombie { inner } => inner.0.clone(),
            Self::ZombieHorse { inner } => inner.0.clone(),
            Self::ZombieVillager { inner } => inner.0.clone(),
            Self::ZombifiedPiglin { inner } => inner.0.clone(),
            Self::ArmorerGift { inner } => inner.0.clone(),
            Self::ButcherGift { inner } => inner.0.clone(),
            Self::CartographerGift { inner } => inner.0.clone(),
            Self::CatMorningGift { inner } => inner.0.clone(),
            Self::ClericGift { inner } => inner.0.clone(),
            Self::FarmerGift { inner } => inner.0.clone(),
            Self::FishermanGift { inner } => inner.0.clone(),
            Self::Fishing { inner } => inner.0.clone(),
            Self::FishingFish { inner } => inner.0.clone(),
            Self::FishingJunk { inner } => inner.0.clone(),
            Self::FishingTreasure { inner } => inner.0.clone(),
            Self::FletcherGift { inner } => inner.0.clone(),
            Self::LeatherworkerGift { inner } => inner.0.clone(),
            Self::LibrarianGift { inner } => inner.0.clone(),
            Self::MasonGift { inner } => inner.0.clone(),
            Self::ShepherdGift { inner } => inner.0.clone(),
            Self::ToolsmithGift { inner } => inner.0.clone(),
            Self::WeaponsmithGift { inner } => inner.0.clone(),
            Self::SnifferDigging { inner } => inner.0.clone(),
            Self::PiglinBartering { inner } => inner.0.clone(),
            Self::DesertWellArchaeology { inner } => inner.0.clone(),
            Self::DesertPyramidArchaeology { inner } => inner.0.clone(),
            Self::TrailRuinsArchaeologyCommon { inner } => inner.0.clone(),
            Self::TrailRuinsArchaeologyRare { inner } => inner.0.clone(),
            Self::OceanRuinWarmArchaeology { inner } => inner.0.clone(),
            Self::OceanRuinColdArchaeology { inner } => inner.0.clone(),
            Self::Sheep { inner } => inner.0.clone(),
            Self::SheepBlack { inner } => inner.0.clone(),
            Self::SheepBlue { inner } => inner.0.clone(),
            Self::SheepBrown { inner } => inner.0.clone(),
            Self::SheepCyan { inner } => inner.0.clone(),
            Self::SheepGray { inner } => inner.0.clone(),
            Self::SheepGreen { inner } => inner.0.clone(),
            Self::SheepLightBlue { inner } => inner.0.clone(),
            Self::SheepLightGray { inner } => inner.0.clone(),
            Self::SheepLime { inner } => inner.0.clone(),
            Self::SheepMagenta { inner } => inner.0.clone(),
            Self::SheepOrange { inner } => inner.0.clone(),
            Self::SheepPink { inner } => inner.0.clone(),
            Self::SheepPurple { inner } => inner.0.clone(),
            Self::SheepRed { inner } => inner.0.clone(),
            Self::SheepWhite { inner } => inner.0.clone(),
            Self::SheepYellow { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Empty { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::AbandonedMineshaft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuriedTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DesertPyramid { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EndCityTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::IglooChest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::JungleTemple { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::JungleTempleDispenser { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NetherBridge { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PillagerOutpost { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionOther { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionBridge { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionHoglinStable { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AncientCity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AncientCityIceBox { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RuinedPortal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShipwreckMap { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShipwreckSupply { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShipwreckTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SimpleDungeon { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SpawnBonusChest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongholdCorridor { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongholdCrossing { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongholdLibrary { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::UnderwaterRuinBig { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::UnderwaterRuinSmall { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageArmorer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageButcher { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageCartographer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageDesertHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageFisher { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageFletcher { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageMason { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillagePlainsHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageSavannaHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageShepherd { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageSnowyHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageTaigaHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageTannery { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageTemple { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageToolsmith { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageWeaponsmith { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WoodlandMansion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ArmorStand { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Axolotl { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Bat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Bee { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Blaze { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::CaveSpider { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Chicken { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cod { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Creeper { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Dolphin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Donkey { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Drowned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ElderGuardian { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnderDragon { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Enderman { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Endermite { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Evoker { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Fox { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ghast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Giant { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::GlowSquid { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Goat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Guardian { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Hoglin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Horse { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Husk { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Illusioner { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::IronGolem { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Llama { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::MagmaCube { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Mooshroom { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Mule { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ocelot { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Panda { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Parrot { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Phantom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Pig { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Piglin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PiglinBrute { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Pillager { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Player { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PolarBear { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Pufferfish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Rabbit { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ravager { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Salmon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Shulker { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Silverfish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Skeleton { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SkeletonHorse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Slime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SnowGolem { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Spider { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Squid { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Stray { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Strider { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TraderLlama { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TropicalFish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Turtle { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Vex { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Villager { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Vindicator { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WanderingTrader { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Witch { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Wither { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WitherSkeleton { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Wolf { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Zoglin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Zombie { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ZombieHorse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ZombieVillager { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ZombifiedPiglin { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ArmorerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ButcherGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CartographerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CatMorningGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ClericGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FarmerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FishermanGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Fishing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FishingFish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FishingJunk { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FishingTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FletcherGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LeatherworkerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LibrarianGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::MasonGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShepherdGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ToolsmithGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WeaponsmithGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SnifferDigging { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PiglinBartering { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DesertWellArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DesertPyramidArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrailRuinsArchaeologyCommon { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrailRuinsArchaeologyRare { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OceanRuinWarmArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OceanRuinColdArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Sheep { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SheepBlack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepBlue { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepBrown { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepCyan { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepGray { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepGreen { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepLightBlue { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepLightGray { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepLime { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepMagenta { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepOrange { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepPink { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepPurple { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepRed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SheepWhite { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepYellow { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootTables<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
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
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "EMPTY" => Ok(LootTables::Empty {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ABANDONED_MINESHAFT" => Ok(LootTables::AbandonedMineshaft {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BURIED_TREASURE" => Ok(LootTables::BuriedTreasure {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DESERT_PYRAMID" => Ok(LootTables::DesertPyramid {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "END_CITY_TREASURE" => Ok(LootTables::EndCityTreasure {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "IGLOO_CHEST" => Ok(LootTables::IglooChest {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "JUNGLE_TEMPLE" => Ok(LootTables::JungleTemple {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "JUNGLE_TEMPLE_DISPENSER" => Ok(LootTables::JungleTempleDispenser {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "NETHER_BRIDGE" => Ok(LootTables::NetherBridge {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PILLAGER_OUTPOST" => Ok(LootTables::PillagerOutpost {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_TREASURE" => Ok(LootTables::BastionTreasure {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_OTHER" => Ok(LootTables::BastionOther {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_BRIDGE" => Ok(LootTables::BastionBridge {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_HOGLIN_STABLE" => Ok(LootTables::BastionHoglinStable {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ANCIENT_CITY" => Ok(LootTables::AncientCity {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ANCIENT_CITY_ICE_BOX" => Ok(LootTables::AncientCityIceBox {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "RUINED_PORTAL" => Ok(LootTables::RuinedPortal {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHIPWRECK_MAP" => Ok(LootTables::ShipwreckMap {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHIPWRECK_SUPPLY" => Ok(LootTables::ShipwreckSupply {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHIPWRECK_TREASURE" => Ok(LootTables::ShipwreckTreasure {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SIMPLE_DUNGEON" => Ok(LootTables::SimpleDungeon {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SPAWN_BONUS_CHEST" => Ok(LootTables::SpawnBonusChest {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "STRONGHOLD_CORRIDOR" => Ok(LootTables::StrongholdCorridor {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "STRONGHOLD_CROSSING" => Ok(LootTables::StrongholdCrossing {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "STRONGHOLD_LIBRARY" => Ok(LootTables::StrongholdLibrary {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "UNDERWATER_RUIN_BIG" => Ok(LootTables::UnderwaterRuinBig {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "UNDERWATER_RUIN_SMALL" => Ok(LootTables::UnderwaterRuinSmall {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_ARMORER" => Ok(LootTables::VillageArmorer {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_BUTCHER" => Ok(LootTables::VillageButcher {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_CARTOGRAPHER" => Ok(LootTables::VillageCartographer {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_DESERT_HOUSE" => Ok(LootTables::VillageDesertHouse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_FISHER" => Ok(LootTables::VillageFisher {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_FLETCHER" => Ok(LootTables::VillageFletcher {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_MASON" => Ok(LootTables::VillageMason {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_PLAINS_HOUSE" => Ok(LootTables::VillagePlainsHouse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_SAVANNA_HOUSE" => Ok(LootTables::VillageSavannaHouse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_SHEPHERD" => Ok(LootTables::VillageShepherd {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_SNOWY_HOUSE" => Ok(LootTables::VillageSnowyHouse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TAIGA_HOUSE" => Ok(LootTables::VillageTaigaHouse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TANNERY" => Ok(LootTables::VillageTannery {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TEMPLE" => Ok(LootTables::VillageTemple {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TOOLSMITH" => Ok(LootTables::VillageToolsmith {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_WEAPONSMITH" => Ok(LootTables::VillageWeaponsmith {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WOODLAND_MANSION" => Ok(LootTables::WoodlandMansion {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ARMOR_STAND" => Ok(LootTables::ArmorStand {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "AXOLOTL" => Ok(LootTables::Axolotl {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BAT" => Ok(LootTables::Bat {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BEE" => Ok(LootTables::Bee {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BLAZE" => Ok(LootTables::Blaze {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CAT" => Ok(LootTables::Cat {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CAVE_SPIDER" => Ok(LootTables::CaveSpider {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CHICKEN" => Ok(LootTables::Chicken {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "COD" => Ok(LootTables::Cod {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "COW" => Ok(LootTables::Cow {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CREEPER" => Ok(LootTables::Creeper {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DOLPHIN" => Ok(LootTables::Dolphin {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DONKEY" => Ok(LootTables::Donkey {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DROWNED" => Ok(LootTables::Drowned {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ELDER_GUARDIAN" => Ok(LootTables::ElderGuardian {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ENDER_DRAGON" => Ok(LootTables::EnderDragon {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ENDERMAN" => Ok(LootTables::Enderman {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ENDERMITE" => Ok(LootTables::Endermite {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "EVOKER" => Ok(LootTables::Evoker {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FOX" => Ok(LootTables::Fox {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "GHAST" => Ok(LootTables::Ghast {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "GIANT" => Ok(LootTables::Giant {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "GLOW_SQUID" => Ok(LootTables::GlowSquid {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "GOAT" => Ok(LootTables::Goat {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "GUARDIAN" => Ok(LootTables::Guardian {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "HOGLIN" => Ok(LootTables::Hoglin {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "HORSE" => Ok(LootTables::Horse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "HUSK" => Ok(LootTables::Husk {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ILLUSIONER" => Ok(LootTables::Illusioner {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "IRON_GOLEM" => Ok(LootTables::IronGolem {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "LLAMA" => Ok(LootTables::Llama {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "MAGMA_CUBE" => Ok(LootTables::MagmaCube {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "MOOSHROOM" => Ok(LootTables::Mooshroom {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "MULE" => Ok(LootTables::Mule {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "OCELOT" => Ok(LootTables::Ocelot {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PANDA" => Ok(LootTables::Panda {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PARROT" => Ok(LootTables::Parrot {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PHANTOM" => Ok(LootTables::Phantom {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PIG" => Ok(LootTables::Pig {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PIGLIN" => Ok(LootTables::Piglin {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_BRUTE" => Ok(LootTables::PiglinBrute {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PILLAGER" => Ok(LootTables::Pillager {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PLAYER" => Ok(LootTables::Player {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "POLAR_BEAR" => Ok(LootTables::PolarBear {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PUFFERFISH" => Ok(LootTables::Pufferfish {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "RABBIT" => Ok(LootTables::Rabbit {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "RAVAGER" => Ok(LootTables::Ravager {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SALMON" => Ok(LootTables::Salmon {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHULKER" => Ok(LootTables::Shulker {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SILVERFISH" => Ok(LootTables::Silverfish {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SKELETON" => Ok(LootTables::Skeleton {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SKELETON_HORSE" => Ok(LootTables::SkeletonHorse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SLIME" => Ok(LootTables::Slime {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SNOW_GOLEM" => Ok(LootTables::SnowGolem {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SPIDER" => Ok(LootTables::Spider {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SQUID" => Ok(LootTables::Squid {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "STRAY" => Ok(LootTables::Stray {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "STRIDER" => Ok(LootTables::Strider {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRADER_LLAMA" => Ok(LootTables::TraderLlama {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TROPICAL_FISH" => Ok(LootTables::TropicalFish {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TURTLE" => Ok(LootTables::Turtle {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VEX" => Ok(LootTables::Vex {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGER" => Ok(LootTables::Villager {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VINDICATOR" => Ok(LootTables::Vindicator {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WANDERING_TRADER" => Ok(LootTables::WanderingTrader {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WITCH" => Ok(LootTables::Witch {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WITHER" => Ok(LootTables::Wither {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WITHER_SKELETON" => Ok(LootTables::WitherSkeleton {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WOLF" => Ok(LootTables::Wolf {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOGLIN" => Ok(LootTables::Zoglin {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIE" => Ok(LootTables::Zombie {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIE_HORSE" => Ok(LootTables::ZombieHorse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIE_VILLAGER" => Ok(LootTables::ZombieVillager {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIFIED_PIGLIN" => Ok(LootTables::ZombifiedPiglin {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ARMORER_GIFT" => Ok(LootTables::ArmorerGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BUTCHER_GIFT" => Ok(LootTables::ButcherGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CARTOGRAPHER_GIFT" => Ok(LootTables::CartographerGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CAT_MORNING_GIFT" => Ok(LootTables::CatMorningGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CLERIC_GIFT" => Ok(LootTables::ClericGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FARMER_GIFT" => Ok(LootTables::FarmerGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHERMAN_GIFT" => Ok(LootTables::FishermanGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING" => Ok(LootTables::Fishing {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING_FISH" => Ok(LootTables::FishingFish {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING_JUNK" => Ok(LootTables::FishingJunk {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING_TREASURE" => Ok(LootTables::FishingTreasure {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FLETCHER_GIFT" => Ok(LootTables::FletcherGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "LEATHERWORKER_GIFT" => Ok(LootTables::LeatherworkerGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "LIBRARIAN_GIFT" => Ok(LootTables::LibrarianGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "MASON_GIFT" => Ok(LootTables::MasonGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEPHERD_GIFT" => Ok(LootTables::ShepherdGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TOOLSMITH_GIFT" => Ok(LootTables::ToolsmithGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WEAPONSMITH_GIFT" => Ok(LootTables::WeaponsmithGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SNIFFER_DIGGING" => Ok(LootTables::SnifferDigging {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_BARTERING" => Ok(LootTables::PiglinBartering {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DESERT_WELL_ARCHAEOLOGY" => Ok(LootTables::DesertWellArchaeology {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DESERT_PYRAMID_ARCHAEOLOGY" => Ok(LootTables::DesertPyramidArchaeology {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRAIL_RUINS_ARCHAEOLOGY_COMMON" => Ok(LootTables::TrailRuinsArchaeologyCommon {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRAIL_RUINS_ARCHAEOLOGY_RARE" => Ok(LootTables::TrailRuinsArchaeologyRare {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "OCEAN_RUIN_WARM_ARCHAEOLOGY" => Ok(LootTables::OceanRuinWarmArchaeology {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "OCEAN_RUIN_COLD_ARCHAEOLOGY" => Ok(LootTables::OceanRuinColdArchaeology {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP" => Ok(LootTables::Sheep {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_BLACK" => Ok(LootTables::SheepBlack {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_BLUE" => Ok(LootTables::SheepBlue {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_BROWN" => Ok(LootTables::SheepBrown {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_CYAN" => Ok(LootTables::SheepCyan {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_GRAY" => Ok(LootTables::SheepGray {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_GREEN" => Ok(LootTables::SheepGreen {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_LIGHT_BLUE" => Ok(LootTables::SheepLightBlue {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_LIGHT_GRAY" => Ok(LootTables::SheepLightGray {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_LIME" => Ok(LootTables::SheepLime {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_MAGENTA" => Ok(LootTables::SheepMagenta {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_ORANGE" => Ok(LootTables::SheepOrange {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_PINK" => Ok(LootTables::SheepPink {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_PURPLE" => Ok(LootTables::SheepPurple {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_RED" => Ok(LootTables::SheepRed {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_WHITE" => Ok(LootTables::SheepWhite {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_YELLOW" => Ok(LootTables::SheepYellow {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for LootTablesStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootTablesStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LootTablesStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootTables")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootTablesStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LootTablesStruct<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
/// Represents additional information a <a href="LootTable.html" title="interface in org.bukkit.loot"><code>LootTable</code></a> can use to modify it's generated loot.
#[repr(C)]
pub struct LootContext<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
#[repr(C)]
pub struct LootContextBuilder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LootContextBuilder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootContextBuilder<'mc> {
    fn from_raw(
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
}

impl<'mc> LootContextBuilder<'mc> {
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
    pub fn build(&self) -> Result<crate::loot::LootContext<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootContext;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "build", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContext::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn luck(
        &self,
        arg0: f32,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(F)Lorg/bukkit/loot/LootContext$Builder;");
        let val_1 = jni::objects::JValueGen::Float(arg0);
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
    pub fn killer(
        &self,
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
    pub fn looting_modifier(
        &self,
        arg0: i32,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/loot/LootContext$Builder;");
        let val_1 = jni::objects::JValueGen::Int(arg0);
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
    pub fn looted_entity(
        &self,
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}

impl<'mc> JNIRaw<'mc> for LootContext<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootContext<'mc> {
    fn from_raw(
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
}

impl<'mc> LootContext<'mc> {
    pub fn killer(
        &self,
    ) -> Result<Option<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKiller", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::HumanEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn luck(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLuck", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }

    pub fn looting_modifier(&self) -> Result<i32, Box<dyn std::error::Error>> {
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

    pub fn looted_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootedEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
