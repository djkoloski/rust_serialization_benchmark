@0xa093b6e172459c50;

enum GameType {
    survival @0;
    creative @1;
    adventure @2;
    spectator @3;
}

struct Item {
    count @0 :Int8;
    slot @1 :UInt8;
    id @2 :Text;
}

struct Abilities {
    walkSpeed @0 :Float32;
    flySpeed @1 :Float32;
    mayFly @2 :Bool;
    flying @3 :Bool;
    invulnerable @4 :Bool;
    mayBuild @5 :Bool;
    instabuild @6 :Bool;
}

struct Entity {
    id @0 :Text;
    pos :group {
        x @1 :Float64;
        y @2 :Float64;
        z @3 :Float64;
    }
    motion :group {
        x @4 :Float64;
        y @5 :Float64;
        z @6 :Float64;
    }
    rotation :group {
        x @7 :Float32;
        y @8 :Float32;
    }
    fallDistance @9 :Float32;
    fire @10 :UInt16;
    air @11 :UInt16;
    onGround @12 :Bool;
    noGravity @13 :Bool;
    invulnerable @14 :Bool;
    portalCooldown @15 :Int32;
    uuid :group {
        x0 @16 :UInt32;
        x1 @17 :UInt32;
        x2 @18 :UInt32;
        x3 @19 :UInt32;
    }
    customName @20 :Text;
    customNameVisible @21 :Bool;
    silent @22 :Bool;
    glowing @23 :Bool;
}

struct RecipeBook {
    recipes @0 :List(Text);
    toBeDisplayed @1 :List(Text);
    isFilteringCraftable @2 :Bool;
    isGuiOpen @3 :Bool;
    isFurnaceFilteringCraftable @4 :Bool;
    isFurnaceGuiOpen @5 :Bool;
    isBlastingFurnaceFilteringCraftable @6 :Bool;
    isBlastingFurnaceGuiOpen @7 :Bool;
    isSmokerFilteringCraftable @8 :Bool;
    isSmokerGuiOpen @9 :Bool;
}

struct Player {
    gameType @0 :GameType;
    previousGameType @1 :GameType;
    score @2 :Int64;
    dimension @3 :Text;
    selectedItemSlot @4 :UInt32;
    selectedItem @5 :Item;
    spawnDimension :union {
        none @6 :Void;
        some @7 :Text;
    }
    spawn :group {
        x @8 :Int64;
        y @9 :Int64;
        z @10 :Int64;
    }
    spawnForced :union {
        none @11 :Void;
        some @12 :Bool;
    }
    sleepTimer @13 :UInt16;
    foodExhaustionLevel @14 :Float32;
    foodSaturationLevel @15 :Float32;
    foodTickTimer @16 :UInt32;
    xpLevel @17 :UInt32;
    xpP @18 :Float32;
    xpTotal @19 :Int32;
    xpSeed @20 :Int32;
    inventory @21 :List(Item);
    enderItems @22 :List(Item);
    abilities @23 :Abilities;
    enteredNetherPosition :union {
        none @24 :Void;
        some :group {
            x @25 :Float64;
            y @26 :Float64;
            z @27 :Float64;
        }
    }
    rootVehicle :union {
        none @28 :Void;
        some :group {
            uuid :group {
                x0 @29 :UInt32;
                x1 @30 :UInt32;
                x2 @31 :UInt32;
                x3 @32 :UInt32;
            }
            entity @33 :Entity;
        }
    }
    shoulderEntityLeft :union {
        none @34 :Void;
        some @35 :Entity;
    }
    shoulderEntityRight :union {
        none @36 :Void;
        some @37 :Entity;
    }
    seenCredits @38 :Bool;
    recipeBook @39 :RecipeBook;
}

struct Players {
    players @0 :List(Player);
}
