@0xb59df916a799be73;

struct Address {
    x0 @0 :UInt8;
    x1 @1 :UInt8;
    x2 @2 :UInt8;
    x3 @3 :UInt8;
}

struct Log {
    address @0 :Address;
    identity @1 :Text;
    userid @2 :Text;
    date @3 :Text;
    request @4 :Text;
    code @5 :UInt16;
    size @6 :UInt64;
}

struct Logs {
    logs @0 :List(Log);
}
