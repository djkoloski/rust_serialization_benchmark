pub mod log_capnp;
pub mod log_generated;

pub mod log_prost {
    include!(concat!(env!("OUT_DIR"), "/prost.log.rs"));
}

use core::pin::Pin;
use crate::{Generate, bench_capnp, bench_flatbuffers, bench_prost};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
pub use log_capnp as cp;
pub use log_generated::log as fb;
use rand::Rng;
use rkyv::Archived;

#[derive(
    Clone, Copy,
    abomonation_derive::Abomonation,
    borsh::BorshSerialize, borsh::BorshDeserialize,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, bytecheck::CheckBytes,
    serde::Serialize, serde::Deserialize,
    speedy::Readable, speedy::Writable,
)]
#[archive(copy)]
pub struct Address {
    pub x0: u8,
    pub x1: u8,
    pub x2: u8,
    pub x3: u8,
}

impl Generate for Address {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        Self {
            x0: rand.gen_range(0..=255),
            x1: rand.gen_range(0..=255),
            x2: rand.gen_range(0..=255),
            x3: rand.gen_range(0..=255),
        }
    }
}

impl Into<fb::Address> for Address {
    fn into(self) -> fb::Address {
        fb::Address::new(self.x0, self.x1, self.x2, self.x3)
    }
}

impl<'a> bench_capnp::Serialize<'a> for Address {
    type Reader = cp::address::Reader<'a>;
    type Builder = cp::address::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_x0(self.x0);
        builder.set_x1(self.x1);
        builder.set_x2(self.x2);
        builder.set_x3(self.x3);
    }
}

impl bench_prost::Serialize for Address {
    type Message = log_prost::Address;

    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        result.x0 = self.x0 as u32;
        result.x1 = self.x1 as u32;
        result.x2 = self.x2 as u32;
        result.x3 = self.x3 as u32;
        result
    }
}

#[derive(
    abomonation_derive::Abomonation,
    borsh::BorshSerialize, borsh::BorshDeserialize,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Serialize, serde::Deserialize,
    speedy::Readable, speedy::Writable,
)]
#[archive(derive(bytecheck::CheckBytes))]
pub struct Log {
    pub address: Address,
    pub identity: String,
    pub userid: String,
    pub date: String,
    pub request: String,
    pub code: u16,
    pub size: u64,
}

impl ArchivedLog {
    pub fn address_pin(self: Pin<&mut Self>) -> Pin<&mut Address> {
        unsafe { self.map_unchecked_mut(|s| &mut s.address) }
    }

    pub fn code_pin(self: Pin<&mut Self>) -> Pin<&mut u16> {
        unsafe { self.map_unchecked_mut(|s| &mut s.code) }
    }

    pub fn size_pin(self: Pin<&mut Self>) -> Pin<&mut u64> {
        unsafe { self.map_unchecked_mut(|s| &mut s.size) }
    }
}

impl Generate for Log {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        const USERID: [&'static str; 9] = [
            "-",
            "alice",
            "bob",
            "carmen",
            "david",
            "eric",
            "frank",
            "george",
            "harry",
        ];
        const MONTHS: [&'static str; 12] = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun",
            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        const TIMEZONE: [&'static str; 25] = [
            "-1200", "-1100", "-1000", "-0900", "-0800", "-0700", "-0600", "-0500", "-0400",
            "-0300", "-0200", "-0100", "+0000", "+0100", "+0200", "+0300", "+0400", "+0500",
            "+0600", "+0700", "+0800", "+0900", "+1000", "+1100", "+1200",
        ];
        let date = format!(
            "{}/{}/{}:{}:{}:{} {}",
            rand.gen_range(1..=28),
            MONTHS[rand.gen_range(0..12)],
            rand.gen_range(1970..=2021),
            rand.gen_range(0..24),
            rand.gen_range(0..60),
            rand.gen_range(0..60),
            TIMEZONE[rand.gen_range(0..25)],
        );
        const CODES: [u16; 63] = [
            100, 101, 102, 103,
            200, 201, 202, 203, 204, 205, 206, 207, 208, 226,
            300, 301, 302, 303, 304, 305, 306, 307, 308,
            400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416,
            417, 418, 421, 422, 423, 424, 425, 426, 428, 429, 431, 451,
            500, 501, 502, 503, 504, 505, 506, 507, 508, 510, 511,
        ];
        const METHODS: [&'static str; 5] = ["GET", "POST", "PUT", "UPDATE", "DELETE"];
        const ROUTES: [&'static str; 7] = [
            "/favicon.ico",
            "/css/index.css",
            "/css/font-awsome.min.css",
            "/img/logo-full.svg",
            "/img/splash.jpg",
            "/api/login",
            "/api/logout",
        ];
        const PROTOCOLS: [&'static str; 4] = [
            "HTTP/1.0",
            "HTTP/1.1",
            "HTTP/2",
            "HTTP/3",
        ];
        let request = format!(
            "{} {} {}",
            METHODS[rand.gen_range(0..5)],
            ROUTES[rand.gen_range(0..7)],
            PROTOCOLS[rand.gen_range(0..4)],
        );
        Self {
            address: Address::generate(rand),
            identity: "-".into(),
            userid: USERID[rand.gen_range(0..USERID.len())].into(),
            date,
            request,
            code: CODES[rand.gen_range(0..CODES.len())],
            size: rand.gen_range(0..100_000_000),
        }
    }
}

impl<'a> bench_flatbuffers::Serialize<'a> for Log {
    type Target = fb::Log<'a>;

    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let address = self.address.into();

        let identity = fbb.create_string(&self.identity);
        let userid = fbb.create_string(&self.userid);
        let date = fbb.create_string(&self.date);
        let request = fbb.create_string(&self.request);

        let mut builder = fb::LogBuilder::new(fbb);
        builder.add_address(&address);
        builder.add_identity(identity);
        builder.add_userid(userid);
        builder.add_date(date);
        builder.add_request(request);
        builder.add_code(self.code);
        builder.add_size_(self.size);
        builder.finish()
    }
}

impl<'a> bench_capnp::Serialize<'a> for Log {
    type Reader = cp::log::Reader<'a>;
    type Builder = cp::log::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        self.address.serialize_capnp(&mut builder.reborrow().init_address());
        builder.set_identity(&self.identity);
        builder.set_userid(&self.userid);
        builder.set_date(&self.date);
        builder.set_request(&self.request);
        builder.set_code(self.code);
        builder.set_size(self.size);
    }
}

impl bench_prost::Serialize for Log {
    type Message = log_prost::Log;

    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        result.identity = self.identity.clone();
        result.userid = self.userid.clone();
        result.date = self.date.clone();
        result.request = self.request.clone();
        result.code = self.code as u32;
        result.size = self.size;
        result
    }
}

#[derive(
    abomonation_derive::Abomonation,
    borsh::BorshSerialize, borsh::BorshDeserialize,
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
    serde::Serialize, serde::Deserialize,
    speedy::Readable, speedy::Writable,
)]
#[archive(derive(bytecheck::CheckBytes))]
pub struct Logs {
    pub logs: Vec<Log>,
}

impl ArchivedLogs {
    pub fn logs_pin(self: Pin<&mut Self>) -> Pin<&mut Archived<Vec<Log>>> {
        unsafe { self.map_unchecked_mut(|s| &mut s.logs) }
    }
}

impl<'a> bench_flatbuffers::Serialize<'a> for Logs {
    type Target = fb::Logs<'a>;

    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        let mut logs = Vec::new();
        for log in self.logs.iter() {
            logs.push(log.serialize_fb(fbb));
        }
        let logs = fbb.create_vector(&logs);

        let mut builder = fb::LogsBuilder::new(fbb);
        builder.add_logs(logs);
        builder.finish()
    }
}

impl<'a> bench_capnp::Serialize<'a> for Logs {
    type Reader = cp::logs::Reader<'a>;
    type Builder = cp::logs::Builder<'a>;

    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        let mut logs = builder.reborrow().init_logs(self.logs.len() as u32);
        for (i, value) in self.logs.iter().enumerate() {
            value.serialize_capnp(&mut logs.reborrow().get(i as u32));
        }
    }
}

impl bench_prost::Serialize for Logs {
    type Message = log_prost::Logs;

    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        for log in self.logs.iter() {
            result.logs.push(log.serialize_pb());
        }
        result
    }
}
