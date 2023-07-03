@0xc3182888fa8baeb0;

enum EntityType {
    arleighBurke @0;
    bismarck @1;
    clemenceau @2;
    fletcher @3;
    g5 @4;
    iowa @5;
    kolkata @6;
    osa @7;
    yasen @8;
    zubr @9;
}

struct Transform {
    altitude @0 :Int8;
    angle @1 :UInt16;
    position :group {
        x @2 :Float32;
        y @3 :Float32;
    }
    velocity @4 :Int16;
}

struct Guidance {
    angle @0 :UInt16;
    submerge @1 :Bool;
    velocity @2 :Int16;
}

struct Contact {
    damage @0 :UInt8;
    entityId @1 :UInt32;
    entityType :union {
        none @2 :Void;
        some @3 :EntityType;
    }
    guidance @4 :Guidance;
    playerId :union {
        none @5 :Void;
        some @6 :UInt16;
    }
    reloads @7 :List(Bool);
    transform @8 :Transform;
    turretAngles @9 :List(UInt16);
}

struct TerrainUpdate {
    chunkId :group {
        x @0 :Int8;
        y @1 :Int8;
    }
    data @2 :List(UInt8);
}

struct Update {
    contacts @0 :List(Contact);
    score @1 :UInt32;
    worldRadius @2 :Float32;
    terrainUpdates @3 :List(TerrainUpdate);
}

struct Updates {
    updates @0 :List(Update);
}
