@0x9311d79f7b43bb9a;

struct Vector3 {
    x @0 :Float32;
    y @1 :Float32;
    z @2 :Float32;
}

struct Triangle {
    v0 @0 :Vector3;
    v1 @1 :Vector3;
    v2 @2 :Vector3;
    normal @3 :Vector3;
}

struct Mesh {
    triangles @0 :List(Triangle);
}
