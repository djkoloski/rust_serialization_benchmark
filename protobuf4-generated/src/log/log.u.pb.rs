const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__log__Address_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Address {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Address>
}

impl ::protobuf::Message for Address {
  type MessageView<'msg> = AddressView<'msg>;
  type MessageMut<'msg> = AddressMut<'msg>;
}

impl ::std::default::Default for Address {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Address {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Address` is `Sync` because it does not implement interior mutability.
//    Neither does `AddressMut`.
unsafe impl ::std::marker::Sync for Address {}

// SAFETY:
// - `Address` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Address {}

impl ::protobuf::Proxied for Address {
  type View<'msg> = AddressView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Address {}

impl ::protobuf::MutProxied for Address {
  type Mut<'msg> = AddressMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AddressView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Address>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AddressView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AddressView<'msg> {
  type Message = Address;
}

impl ::std::fmt::Debug for AddressView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AddressView<'_> {
  fn default() -> AddressView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Address>> for AddressView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Address>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AddressView<'msg> {

  pub fn to_owned(&self) -> Address {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // x0: optional uint32
  pub fn x0(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // x1: optional uint32
  pub fn x1(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // x2: optional uint32
  pub fn x2(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // x3: optional uint32
  pub fn x3(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `AddressView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AddressView<'_> {}

// SAFETY:
// - `AddressView` is `Send` because while its alive a `AddressMut` cannot.
// - `AddressView` does not use thread-local data.
unsafe impl ::std::marker::Send for AddressView<'_> {}

impl<'msg> ::protobuf::AsView for AddressView<'msg> {
  type Proxied = Address;
  fn as_view(&self) -> ::protobuf::View<'msg, Address> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AddressView<'msg> {
  fn into_view<'shorter>(self) -> AddressView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Address> for AddressView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Address {
    let mut dst = Address::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Address> for AddressMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Address {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Address {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AddressView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AddressMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AddressMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Address>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AddressMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AddressMut<'msg> {
  type Message = Address;
}

impl ::std::fmt::Debug for AddressMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Address>> for AddressMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Address>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AddressMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Address> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Address {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // x0: optional uint32
  pub fn x0(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x0(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // x1: optional uint32
  pub fn x1(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x1(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

  // x2: optional uint32
  pub fn x2(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x2(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // x3: optional uint32
  pub fn x3(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x3(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `AddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AddressMut<'_> {}

// SAFETY:
// - `AddressMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AddressMut<'_> {}

impl<'msg> ::protobuf::AsView for AddressMut<'msg> {
  type Proxied = Address;
  fn as_view(&self) -> ::protobuf::View<'_, Address> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AddressMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Address>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AddressMut<'msg> {
  type MutProxied = Address;
  fn as_mut(&mut self) -> AddressMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AddressMut<'msg> {
  fn into_mut<'shorter>(self) -> AddressMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Address {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Address> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AddressView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AddressMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // x0: optional uint32
  pub fn x0(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x0(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // x1: optional uint32
  pub fn x1(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        1, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x1(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        1, val.into()
      )
    }
  }

  // x2: optional uint32
  pub fn x2(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x2(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // x3: optional uint32
  pub fn x3(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        3, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x3(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        3, val.into()
      )
    }
  }

}  // impl Address

impl ::std::ops::Drop for Address {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Address {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Address {
  type Proxied = Self;
  fn as_view(&self) -> AddressView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Address {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AddressMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Address {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__log__Address_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P)P)P)P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__log__Address_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__log__Address_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Address {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Address {
  type Msg = Address;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Address> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Address {
  type Msg = Address;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Address> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AddressMut<'_> {
  type Msg = Address;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Address> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AddressMut<'_> {
  type Msg = Address;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Address> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AddressView<'_> {
  type Msg = Address;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Address> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AddressMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__log__Log_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Log {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Log>
}

impl ::protobuf::Message for Log {
  type MessageView<'msg> = LogView<'msg>;
  type MessageMut<'msg> = LogMut<'msg>;
}

impl ::std::default::Default for Log {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Log {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Log` is `Sync` because it does not implement interior mutability.
//    Neither does `LogMut`.
unsafe impl ::std::marker::Sync for Log {}

// SAFETY:
// - `Log` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Log {}

impl ::protobuf::Proxied for Log {
  type View<'msg> = LogView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Log {}

impl ::protobuf::MutProxied for Log {
  type Mut<'msg> = LogMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LogView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Log>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LogView<'msg> {
  type Message = Log;
}

impl ::std::fmt::Debug for LogView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LogView<'_> {
  fn default() -> LogView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Log>> for LogView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Log>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogView<'msg> {

  pub fn to_owned(&self) -> Log {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // address: optional message prost.log.Address
  pub fn has_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn address_opt(self) -> ::std::option::Option<super::AddressView<'msg>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(self) -> super::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AddressView::default())
  }

  // identity: optional string
  pub fn identity(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // userid: optional string
  pub fn userid(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // date: optional string
  pub fn date(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // request: optional string
  pub fn request(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // code: optional uint32
  pub fn code(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // size: optional uint64
  pub fn size(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        6, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `LogView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LogView<'_> {}

// SAFETY:
// - `LogView` is `Send` because while its alive a `LogMut` cannot.
// - `LogView` does not use thread-local data.
unsafe impl ::std::marker::Send for LogView<'_> {}

impl<'msg> ::protobuf::AsView for LogView<'msg> {
  type Proxied = Log;
  fn as_view(&self) -> ::protobuf::View<'msg, Log> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogView<'msg> {
  fn into_view<'shorter>(self) -> LogView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Log> for LogView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Log {
    let mut dst = Log::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Log> for LogMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Log {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Log {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LogView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LogMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LogMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Log>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LogMut<'msg> {
  type Message = Log;
}

impl ::std::fmt::Debug for LogMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Log>> for LogMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Log>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Log> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Log {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // address: optional message prost.log.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<super::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> super::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AddressView::default())
  }
  pub fn address_mut(&mut self) -> super::AddressMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<super::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // identity: optional string
  pub fn identity(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_identity(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // userid: optional string
  pub fn userid(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_userid(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // date: optional string
  pub fn date(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_date(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // request: optional string
  pub fn request(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_request(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // code: optional uint32
  pub fn code(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_code(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        5, val.into()
      )
    }
  }

  // size: optional uint64
  pub fn size(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        6, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_size(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        6, val.into()
      )
    }
  }

}

// SAFETY:
// - `LogMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LogMut<'_> {}

// SAFETY:
// - `LogMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LogMut<'_> {}

impl<'msg> ::protobuf::AsView for LogMut<'msg> {
  type Proxied = Log;
  fn as_view(&self) -> ::protobuf::View<'_, Log> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Log>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LogMut<'msg> {
  type MutProxied = Log;
  fn as_mut(&mut self) -> LogMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LogMut<'msg> {
  fn into_mut<'shorter>(self) -> LogMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Log {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Log> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LogView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LogMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // address: optional message prost.log.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<super::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> super::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AddressView::default())
  }
  pub fn address_mut(&mut self) -> super::AddressMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<super::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // identity: optional string
  pub fn identity(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_identity(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // userid: optional string
  pub fn userid(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_userid(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // date: optional string
  pub fn date(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_date(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // request: optional string
  pub fn request(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_request(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // code: optional uint32
  pub fn code(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_code(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        5, val.into()
      )
    }
  }

  // size: optional uint64
  pub fn size(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        6, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_size(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        6, val.into()
      )
    }
  }

}  // impl Log

impl ::std::ops::Drop for Log {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Log {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Log {
  type Proxied = Self;
  fn as_view(&self) -> LogView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Log {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LogMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Log {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__log__Log_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31X1X1X1X)P,P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__log__Log_msg_init.0, &[<super::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__log__Log_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Log {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Log {
  type Msg = Log;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Log> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Log {
  type Msg = Log;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Log> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogMut<'_> {
  type Msg = Log;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Log> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogMut<'_> {
  type Msg = Log;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Log> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogView<'_> {
  type Msg = Log;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Log> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__log__Logs_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Logs {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Logs>
}

impl ::protobuf::Message for Logs {
  type MessageView<'msg> = LogsView<'msg>;
  type MessageMut<'msg> = LogsMut<'msg>;
}

impl ::std::default::Default for Logs {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Logs {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Logs` is `Sync` because it does not implement interior mutability.
//    Neither does `LogsMut`.
unsafe impl ::std::marker::Sync for Logs {}

// SAFETY:
// - `Logs` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Logs {}

impl ::protobuf::Proxied for Logs {
  type View<'msg> = LogsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Logs {}

impl ::protobuf::MutProxied for Logs {
  type Mut<'msg> = LogsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LogsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Logs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LogsView<'msg> {
  type Message = Logs;
}

impl ::std::fmt::Debug for LogsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LogsView<'_> {
  fn default() -> LogsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Logs>> for LogsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Logs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogsView<'msg> {

  pub fn to_owned(&self) -> Logs {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // logs: repeated message prost.log.Log
  pub fn logs(self) -> ::protobuf::RepeatedView<'msg, super::Log> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Log>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `LogsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LogsView<'_> {}

// SAFETY:
// - `LogsView` is `Send` because while its alive a `LogsMut` cannot.
// - `LogsView` does not use thread-local data.
unsafe impl ::std::marker::Send for LogsView<'_> {}

impl<'msg> ::protobuf::AsView for LogsView<'msg> {
  type Proxied = Logs;
  fn as_view(&self) -> ::protobuf::View<'msg, Logs> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogsView<'msg> {
  fn into_view<'shorter>(self) -> LogsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Logs> for LogsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Logs {
    let mut dst = Logs::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Logs> for LogsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Logs {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Logs {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LogsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LogsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LogsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Logs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LogsMut<'msg> {
  type Message = Logs;
}

impl ::std::fmt::Debug for LogsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Logs>> for LogsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Logs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Logs> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Logs {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // logs: repeated message prost.log.Log
  pub fn logs(&self) -> ::protobuf::RepeatedView<'_, super::Log> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Log>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn logs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Log> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_logs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Log>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `LogsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LogsMut<'_> {}

// SAFETY:
// - `LogsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LogsMut<'_> {}

impl<'msg> ::protobuf::AsView for LogsMut<'msg> {
  type Proxied = Logs;
  fn as_view(&self) -> ::protobuf::View<'_, Logs> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Logs>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LogsMut<'msg> {
  type MutProxied = Logs;
  fn as_mut(&mut self) -> LogsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LogsMut<'msg> {
  fn into_mut<'shorter>(self) -> LogsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Logs {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Logs> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LogsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LogsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // logs: repeated message prost.log.Log
  pub fn logs(&self) -> ::protobuf::RepeatedView<'_, super::Log> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Log>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn logs_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Log> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_logs(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Log>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Logs

impl ::std::ops::Drop for Logs {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Logs {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Logs {
  type Proxied = Self;
  fn as_view(&self) -> LogsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Logs {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LogsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Logs {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__log__Logs_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__log__Logs_msg_init.0, &[<super::Log as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__log__Logs_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Logs {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Logs {
  type Msg = Logs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Logs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Logs {
  type Msg = Logs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Logs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogsMut<'_> {
  type Msg = Logs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Logs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogsMut<'_> {
  type Msg = Logs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Logs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogsView<'_> {
  type Msg = Logs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Logs> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



