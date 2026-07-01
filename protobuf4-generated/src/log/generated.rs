#[path="log.u.pb.rs"]
#[allow(nonstandard_style, unused, unreachable_pub)]
#[doc(hidden)]
mod internal_do_not_use_log;

#[allow(nonstandard_style, unused)]
#[doc(inline)]
pub use internal_do_not_use_log::*;
#[allow(nonstandard_style, unused)]
pub mod __unstable {
pub static LOG_DESCRIPTOR_INFO: ::protobuf::__internal::runtime::__unstable::DescriptorInfo = ::protobuf::__internal::runtime::__unstable::DescriptorInfo {
  descriptor: b"\n\tlog.proto\x12\tprost.log\"9\n\x07\x41\x64\x64ress\x12\n\n\x02x0\x18\x01 \x01(\r\x12\n\n\x02x1\x18\x02 \x01(\r\x12\n\n\x02x2\x18\x03 \x01(\r\x12\n\n\x02x3\x18\x04 \x01(\r\"\x87\x01\n\x03Log\x12#\n\x07\x61\x64\x64ress\x18\x01 \x01(\x0b\x32\x12.prost.log.Address\x12\x10\n\x08identity\x18\x02 \x01(\t\x12\x0e\n\x06userid\x18\x03 \x01(\t\x12\x0c\n\x04\x64\x61te\x18\x04 \x01(\t\x12\x0f\n\x07request\x18\x05 \x01(\t\x12\x0c\n\x04\x63ode\x18\x06 \x01(\r\x12\x0c\n\x04size\x18\x07 \x01(\x04\"$\n\x04Logs\x12\x1c\n\x04logs\x18\x01 \x03(\x0b\x32\x0e.prost.log.Logb\x06proto3",
  deps: &[
  ],
};
}
