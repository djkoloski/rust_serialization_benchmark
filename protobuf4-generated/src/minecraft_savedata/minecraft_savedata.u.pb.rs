const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__minecraft_0savedata__Item_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Item {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Item>
}

impl ::protobuf::Message for Item {
  type MessageView<'msg> = ItemView<'msg>;
  type MessageMut<'msg> = ItemMut<'msg>;
}

impl ::std::default::Default for Item {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Item {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Item` is `Sync` because it does not implement interior mutability.
//    Neither does `ItemMut`.
unsafe impl ::std::marker::Sync for Item {}

// SAFETY:
// - `Item` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Item {}

impl ::protobuf::Proxied for Item {
  type View<'msg> = ItemView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Item {}

impl ::protobuf::MutProxied for Item {
  type Mut<'msg> = ItemMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ItemView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Item>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ItemView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ItemView<'msg> {
  type Message = Item;
}

impl ::std::fmt::Debug for ItemView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ItemView<'_> {
  fn default() -> ItemView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Item>> for ItemView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Item>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ItemView<'msg> {

  pub fn to_owned(&self) -> Item {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // count: optional int32
  pub fn count(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // slot: optional uint32
  pub fn slot(self) -> u32 {
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

  // id: optional string
  pub fn id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `ItemView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ItemView<'_> {}

// SAFETY:
// - `ItemView` is `Send` because while its alive a `ItemMut` cannot.
// - `ItemView` does not use thread-local data.
unsafe impl ::std::marker::Send for ItemView<'_> {}

impl<'msg> ::protobuf::AsView for ItemView<'msg> {
  type Proxied = Item;
  fn as_view(&self) -> ::protobuf::View<'msg, Item> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ItemView<'msg> {
  fn into_view<'shorter>(self) -> ItemView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Item> for ItemView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Item {
    let mut dst = Item::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Item> for ItemMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Item {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Item {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ItemView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ItemMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ItemMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Item>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ItemMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ItemMut<'msg> {
  type Message = Item;
}

impl ::std::fmt::Debug for ItemMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Item>> for ItemMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Item>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ItemMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Item> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Item {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // count: optional int32
  pub fn count(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_count(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // slot: optional uint32
  pub fn slot(&self) -> u32 {
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
  pub fn set_slot(&mut self, val: u32) {
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

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `ItemMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ItemMut<'_> {}

// SAFETY:
// - `ItemMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ItemMut<'_> {}

impl<'msg> ::protobuf::AsView for ItemMut<'msg> {
  type Proxied = Item;
  fn as_view(&self) -> ::protobuf::View<'_, Item> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ItemMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Item>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ItemMut<'msg> {
  type MutProxied = Item;
  fn as_mut(&mut self) -> ItemMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ItemMut<'msg> {
  fn into_mut<'shorter>(self) -> ItemMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Item {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Item> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ItemView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ItemMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // count: optional int32
  pub fn count(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_count(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // slot: optional uint32
  pub fn slot(&self) -> u32 {
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
  pub fn set_slot(&mut self, val: u32) {
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

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl Item

impl ::std::ops::Drop for Item {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Item {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Item {
  type Proxied = Self;
  fn as_view(&self) -> ItemView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Item {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ItemMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Item {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__minecraft_0savedata__Item_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P)P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__minecraft_0savedata__Item_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__minecraft_0savedata__Item_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Item {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Item {
  type Msg = Item;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Item {
  type Msg = Item;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ItemMut<'_> {
  type Msg = Item;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ItemMut<'_> {
  type Msg = Item;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ItemView<'_> {
  type Msg = Item;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Item> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ItemMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__minecraft_0savedata__Abilities_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Abilities {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Abilities>
}

impl ::protobuf::Message for Abilities {
  type MessageView<'msg> = AbilitiesView<'msg>;
  type MessageMut<'msg> = AbilitiesMut<'msg>;
}

impl ::std::default::Default for Abilities {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Abilities {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Abilities` is `Sync` because it does not implement interior mutability.
//    Neither does `AbilitiesMut`.
unsafe impl ::std::marker::Sync for Abilities {}

// SAFETY:
// - `Abilities` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Abilities {}

impl ::protobuf::Proxied for Abilities {
  type View<'msg> = AbilitiesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Abilities {}

impl ::protobuf::MutProxied for Abilities {
  type Mut<'msg> = AbilitiesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AbilitiesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Abilities>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AbilitiesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AbilitiesView<'msg> {
  type Message = Abilities;
}

impl ::std::fmt::Debug for AbilitiesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AbilitiesView<'_> {
  fn default() -> AbilitiesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Abilities>> for AbilitiesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Abilities>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AbilitiesView<'msg> {

  pub fn to_owned(&self) -> Abilities {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // walk_speed: optional float
  pub fn walk_speed(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // fly_speed: optional float
  pub fn fly_speed(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // may_fly: optional bool
  pub fn may_fly(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }

  // flying: optional bool
  pub fn flying(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

  // invulnerable: optional bool
  pub fn invulnerable(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }

  // may_build: optional bool
  pub fn may_build(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }

  // instabuild: optional bool
  pub fn instabuild(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `AbilitiesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AbilitiesView<'_> {}

// SAFETY:
// - `AbilitiesView` is `Send` because while its alive a `AbilitiesMut` cannot.
// - `AbilitiesView` does not use thread-local data.
unsafe impl ::std::marker::Send for AbilitiesView<'_> {}

impl<'msg> ::protobuf::AsView for AbilitiesView<'msg> {
  type Proxied = Abilities;
  fn as_view(&self) -> ::protobuf::View<'msg, Abilities> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AbilitiesView<'msg> {
  fn into_view<'shorter>(self) -> AbilitiesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Abilities> for AbilitiesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Abilities {
    let mut dst = Abilities::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Abilities> for AbilitiesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Abilities {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Abilities {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AbilitiesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AbilitiesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AbilitiesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Abilities>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AbilitiesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AbilitiesMut<'msg> {
  type Message = Abilities;
}

impl ::std::fmt::Debug for AbilitiesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Abilities>> for AbilitiesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Abilities>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AbilitiesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Abilities> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Abilities {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // walk_speed: optional float
  pub fn walk_speed(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_walk_speed(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        0, val.into()
      )
    }
  }

  // fly_speed: optional float
  pub fn fly_speed(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fly_speed(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        1, val.into()
      )
    }
  }

  // may_fly: optional bool
  pub fn may_fly(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_may_fly(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // flying: optional bool
  pub fn flying(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_flying(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // invulnerable: optional bool
  pub fn invulnerable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_invulnerable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

  // may_build: optional bool
  pub fn may_build(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_may_build(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // instabuild: optional bool
  pub fn instabuild(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_instabuild(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

}

// SAFETY:
// - `AbilitiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AbilitiesMut<'_> {}

// SAFETY:
// - `AbilitiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AbilitiesMut<'_> {}

impl<'msg> ::protobuf::AsView for AbilitiesMut<'msg> {
  type Proxied = Abilities;
  fn as_view(&self) -> ::protobuf::View<'_, Abilities> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AbilitiesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Abilities>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AbilitiesMut<'msg> {
  type MutProxied = Abilities;
  fn as_mut(&mut self) -> AbilitiesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AbilitiesMut<'msg> {
  fn into_mut<'shorter>(self) -> AbilitiesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Abilities {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Abilities> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AbilitiesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AbilitiesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // walk_speed: optional float
  pub fn walk_speed(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_walk_speed(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        0, val.into()
      )
    }
  }

  // fly_speed: optional float
  pub fn fly_speed(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fly_speed(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        1, val.into()
      )
    }
  }

  // may_fly: optional bool
  pub fn may_fly(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_may_fly(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // flying: optional bool
  pub fn flying(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_flying(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // invulnerable: optional bool
  pub fn invulnerable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_invulnerable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

  // may_build: optional bool
  pub fn may_build(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_may_build(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // instabuild: optional bool
  pub fn instabuild(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_instabuild(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

}  // impl Abilities

impl ::std::ops::Drop for Abilities {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Abilities {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Abilities {
  type Proxied = Self;
  fn as_view(&self) -> AbilitiesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Abilities {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AbilitiesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Abilities {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__minecraft_0savedata__Abilities_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$!P!P/P/P/P/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__minecraft_0savedata__Abilities_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__minecraft_0savedata__Abilities_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Abilities {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Abilities {
  type Msg = Abilities;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Abilities> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Abilities {
  type Msg = Abilities;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Abilities> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AbilitiesMut<'_> {
  type Msg = Abilities;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Abilities> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AbilitiesMut<'_> {
  type Msg = Abilities;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Abilities> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AbilitiesView<'_> {
  type Msg = Abilities;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Abilities> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AbilitiesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__minecraft_0savedata__Vector3d_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Vector3d {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Vector3d>
}

impl ::protobuf::Message for Vector3d {
  type MessageView<'msg> = Vector3dView<'msg>;
  type MessageMut<'msg> = Vector3dMut<'msg>;
}

impl ::std::default::Default for Vector3d {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Vector3d {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Vector3d` is `Sync` because it does not implement interior mutability.
//    Neither does `Vector3dMut`.
unsafe impl ::std::marker::Sync for Vector3d {}

// SAFETY:
// - `Vector3d` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Vector3d {}

impl ::protobuf::Proxied for Vector3d {
  type View<'msg> = Vector3dView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Vector3d {}

impl ::protobuf::MutProxied for Vector3d {
  type Mut<'msg> = Vector3dMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Vector3dView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vector3d>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Vector3dView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Vector3dView<'msg> {
  type Message = Vector3d;
}

impl ::std::fmt::Debug for Vector3dView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Vector3dView<'_> {
  fn default() -> Vector3dView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Vector3d>> for Vector3dView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vector3d>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Vector3dView<'msg> {

  pub fn to_owned(&self) -> Vector3d {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // x: optional double
  pub fn x(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // y: optional double
  pub fn y(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // z: optional double
  pub fn z(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `Vector3dView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Vector3dView<'_> {}

// SAFETY:
// - `Vector3dView` is `Send` because while its alive a `Vector3dMut` cannot.
// - `Vector3dView` does not use thread-local data.
unsafe impl ::std::marker::Send for Vector3dView<'_> {}

impl<'msg> ::protobuf::AsView for Vector3dView<'msg> {
  type Proxied = Vector3d;
  fn as_view(&self) -> ::protobuf::View<'msg, Vector3d> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Vector3dView<'msg> {
  fn into_view<'shorter>(self) -> Vector3dView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Vector3d> for Vector3dView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vector3d {
    let mut dst = Vector3d::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Vector3d> for Vector3dMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vector3d {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Vector3d {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Vector3dView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Vector3dMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Vector3dMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector3d>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Vector3dMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Vector3dMut<'msg> {
  type Message = Vector3d;
}

impl ::std::fmt::Debug for Vector3dMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Vector3d>> for Vector3dMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector3d>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Vector3dMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector3d> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Vector3d {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // x: optional double
  pub fn x(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        0, val.into()
      )
    }
  }

  // y: optional double
  pub fn y(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_y(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

  // z: optional double
  pub fn z(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_z(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `Vector3dMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Vector3dMut<'_> {}

// SAFETY:
// - `Vector3dMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Vector3dMut<'_> {}

impl<'msg> ::protobuf::AsView for Vector3dMut<'msg> {
  type Proxied = Vector3d;
  fn as_view(&self) -> ::protobuf::View<'_, Vector3d> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Vector3dMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Vector3d>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Vector3dMut<'msg> {
  type MutProxied = Vector3d;
  fn as_mut(&mut self) -> Vector3dMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Vector3dMut<'msg> {
  fn into_mut<'shorter>(self) -> Vector3dMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Vector3d {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Vector3d> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Vector3dView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Vector3dMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // x: optional double
  pub fn x(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        0, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        0, val.into()
      )
    }
  }

  // y: optional double
  pub fn y(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        1, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_y(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        1, val.into()
      )
    }
  }

  // z: optional double
  pub fn z(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        2, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_z(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        2, val.into()
      )
    }
  }

}  // impl Vector3d

impl ::std::ops::Drop for Vector3d {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Vector3d {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Vector3d {
  type Proxied = Self;
  fn as_view(&self) -> Vector3dView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Vector3d {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Vector3dMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Vector3d {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__minecraft_0savedata__Vector3d_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ P P P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__minecraft_0savedata__Vector3d_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__minecraft_0savedata__Vector3d_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Vector3d {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Vector3d {
  type Msg = Vector3d;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3d> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector3d {
  type Msg = Vector3d;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3d> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Vector3dMut<'_> {
  type Msg = Vector3d;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3d> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector3dMut<'_> {
  type Msg = Vector3d;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3d> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector3dView<'_> {
  type Msg = Vector3d;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3d> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Vector3dMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__minecraft_0savedata__Vector2f_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Vector2f {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Vector2f>
}

impl ::protobuf::Message for Vector2f {
  type MessageView<'msg> = Vector2fView<'msg>;
  type MessageMut<'msg> = Vector2fMut<'msg>;
}

impl ::std::default::Default for Vector2f {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Vector2f {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Vector2f` is `Sync` because it does not implement interior mutability.
//    Neither does `Vector2fMut`.
unsafe impl ::std::marker::Sync for Vector2f {}

// SAFETY:
// - `Vector2f` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Vector2f {}

impl ::protobuf::Proxied for Vector2f {
  type View<'msg> = Vector2fView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Vector2f {}

impl ::protobuf::MutProxied for Vector2f {
  type Mut<'msg> = Vector2fMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Vector2fView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vector2f>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Vector2fView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Vector2fView<'msg> {
  type Message = Vector2f;
}

impl ::std::fmt::Debug for Vector2fView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Vector2fView<'_> {
  fn default() -> Vector2fView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Vector2f>> for Vector2fView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vector2f>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Vector2fView<'msg> {

  pub fn to_owned(&self) -> Vector2f {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // x: optional float
  pub fn x(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // y: optional float
  pub fn y(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `Vector2fView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Vector2fView<'_> {}

// SAFETY:
// - `Vector2fView` is `Send` because while its alive a `Vector2fMut` cannot.
// - `Vector2fView` does not use thread-local data.
unsafe impl ::std::marker::Send for Vector2fView<'_> {}

impl<'msg> ::protobuf::AsView for Vector2fView<'msg> {
  type Proxied = Vector2f;
  fn as_view(&self) -> ::protobuf::View<'msg, Vector2f> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Vector2fView<'msg> {
  fn into_view<'shorter>(self) -> Vector2fView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Vector2f> for Vector2fView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vector2f {
    let mut dst = Vector2f::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Vector2f> for Vector2fMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vector2f {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Vector2f {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Vector2fView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Vector2fMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Vector2fMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector2f>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Vector2fMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Vector2fMut<'msg> {
  type Message = Vector2f;
}

impl ::std::fmt::Debug for Vector2fMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Vector2f>> for Vector2fMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector2f>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Vector2fMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector2f> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Vector2f {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // x: optional float
  pub fn x(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        0, val.into()
      )
    }
  }

  // y: optional float
  pub fn y(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_y(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `Vector2fMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Vector2fMut<'_> {}

// SAFETY:
// - `Vector2fMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Vector2fMut<'_> {}

impl<'msg> ::protobuf::AsView for Vector2fMut<'msg> {
  type Proxied = Vector2f;
  fn as_view(&self) -> ::protobuf::View<'_, Vector2f> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Vector2fMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Vector2f>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Vector2fMut<'msg> {
  type MutProxied = Vector2f;
  fn as_mut(&mut self) -> Vector2fMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Vector2fMut<'msg> {
  fn into_mut<'shorter>(self) -> Vector2fMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Vector2f {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Vector2f> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Vector2fView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Vector2fMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // x: optional float
  pub fn x(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_x(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        0, val.into()
      )
    }
  }

  // y: optional float
  pub fn y(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_y(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        1, val.into()
      )
    }
  }

}  // impl Vector2f

impl ::std::ops::Drop for Vector2f {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Vector2f {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Vector2f {
  type Proxied = Self;
  fn as_view(&self) -> Vector2fView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Vector2f {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Vector2fMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Vector2f {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__minecraft_0savedata__Vector2f_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$!P!P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__minecraft_0savedata__Vector2f_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__minecraft_0savedata__Vector2f_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Vector2f {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Vector2f {
  type Msg = Vector2f;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector2f> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector2f {
  type Msg = Vector2f;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector2f> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Vector2fMut<'_> {
  type Msg = Vector2f;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector2f> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector2fMut<'_> {
  type Msg = Vector2f;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector2f> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector2fView<'_> {
  type Msg = Vector2f;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector2f> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Vector2fMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__minecraft_0savedata__Uuid_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Uuid {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Uuid>
}

impl ::protobuf::Message for Uuid {
  type MessageView<'msg> = UuidView<'msg>;
  type MessageMut<'msg> = UuidMut<'msg>;
}

impl ::std::default::Default for Uuid {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Uuid {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Uuid` is `Sync` because it does not implement interior mutability.
//    Neither does `UuidMut`.
unsafe impl ::std::marker::Sync for Uuid {}

// SAFETY:
// - `Uuid` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Uuid {}

impl ::protobuf::Proxied for Uuid {
  type View<'msg> = UuidView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Uuid {}

impl ::protobuf::MutProxied for Uuid {
  type Mut<'msg> = UuidMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UuidView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Uuid>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UuidView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UuidView<'msg> {
  type Message = Uuid;
}

impl ::std::fmt::Debug for UuidView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UuidView<'_> {
  fn default() -> UuidView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Uuid>> for UuidView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Uuid>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UuidView<'msg> {

  pub fn to_owned(&self) -> Uuid {
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
// - `UuidView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UuidView<'_> {}

// SAFETY:
// - `UuidView` is `Send` because while its alive a `UuidMut` cannot.
// - `UuidView` does not use thread-local data.
unsafe impl ::std::marker::Send for UuidView<'_> {}

impl<'msg> ::protobuf::AsView for UuidView<'msg> {
  type Proxied = Uuid;
  fn as_view(&self) -> ::protobuf::View<'msg, Uuid> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UuidView<'msg> {
  fn into_view<'shorter>(self) -> UuidView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Uuid> for UuidView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Uuid {
    let mut dst = Uuid::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Uuid> for UuidMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Uuid {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Uuid {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UuidView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UuidMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UuidMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Uuid>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UuidMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UuidMut<'msg> {
  type Message = Uuid;
}

impl ::std::fmt::Debug for UuidMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Uuid>> for UuidMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Uuid>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UuidMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Uuid> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Uuid {
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
// - `UuidMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UuidMut<'_> {}

// SAFETY:
// - `UuidMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UuidMut<'_> {}

impl<'msg> ::protobuf::AsView for UuidMut<'msg> {
  type Proxied = Uuid;
  fn as_view(&self) -> ::protobuf::View<'_, Uuid> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UuidMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Uuid>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UuidMut<'msg> {
  type MutProxied = Uuid;
  fn as_mut(&mut self) -> UuidMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UuidMut<'msg> {
  fn into_mut<'shorter>(self) -> UuidMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Uuid {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Uuid> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UuidView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UuidMut<'_> {
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

}  // impl Uuid

impl ::std::ops::Drop for Uuid {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Uuid {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Uuid {
  type Proxied = Self;
  fn as_view(&self) -> UuidView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Uuid {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UuidMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Uuid {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__minecraft_0savedata__Uuid_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P)P)P)P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__minecraft_0savedata__Uuid_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__minecraft_0savedata__Uuid_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Uuid {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Uuid {
  type Msg = Uuid;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Uuid> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Uuid {
  type Msg = Uuid;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Uuid> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UuidMut<'_> {
  type Msg = Uuid;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Uuid> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UuidMut<'_> {
  type Msg = Uuid;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Uuid> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UuidView<'_> {
  type Msg = Uuid;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Uuid> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UuidMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__minecraft_0savedata__Entity_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Entity {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Entity>
}

impl ::protobuf::Message for Entity {
  type MessageView<'msg> = EntityView<'msg>;
  type MessageMut<'msg> = EntityMut<'msg>;
}

impl ::std::default::Default for Entity {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Entity {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Entity` is `Sync` because it does not implement interior mutability.
//    Neither does `EntityMut`.
unsafe impl ::std::marker::Sync for Entity {}

// SAFETY:
// - `Entity` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Entity {}

impl ::protobuf::Proxied for Entity {
  type View<'msg> = EntityView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Entity {}

impl ::protobuf::MutProxied for Entity {
  type Mut<'msg> = EntityMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EntityView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Entity>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EntityView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EntityView<'msg> {
  type Message = Entity;
}

impl ::std::fmt::Debug for EntityView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EntityView<'_> {
  fn default() -> EntityView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Entity>> for EntityView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Entity>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EntityView<'msg> {

  pub fn to_owned(&self) -> Entity {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional string
  pub fn id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // pos: optional message prost.minecraft_savedata.Vector3d
  pub fn has_pos(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn pos_opt(self) -> ::std::option::Option<super::Vector3dView<'msg>> {
    self.has_pos().then(|| self.pos())
  }
  pub fn pos(self) -> super::Vector3dView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3dView::default())
  }

  // motion: optional message prost.minecraft_savedata.Vector3d
  pub fn has_motion(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn motion_opt(self) -> ::std::option::Option<super::Vector3dView<'msg>> {
    self.has_motion().then(|| self.motion())
  }
  pub fn motion(self) -> super::Vector3dView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3dView::default())
  }

  // rotation: optional message prost.minecraft_savedata.Vector2f
  pub fn has_rotation(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn rotation_opt(self) -> ::std::option::Option<super::Vector2fView<'msg>> {
    self.has_rotation().then(|| self.rotation())
  }
  pub fn rotation(self) -> super::Vector2fView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector2fView::default())
  }

  // fall_distance: optional float
  pub fn fall_distance(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // fire: optional uint32
  pub fn fire(self) -> u32 {
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

  // air: optional uint32
  pub fn air(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        6, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // on_ground: optional bool
  pub fn on_ground(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

  // no_gravity: optional bool
  pub fn no_gravity(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }

  // invulnerable: optional bool
  pub fn invulnerable(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }

  // portal_cooldown: optional int32
  pub fn portal_cooldown(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        10, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // uuid: optional message prost.minecraft_savedata.Uuid
  pub fn has_uuid(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn uuid_opt(self) -> ::std::option::Option<super::UuidView<'msg>> {
    self.has_uuid().then(|| self.uuid())
  }
  pub fn uuid(self) -> super::UuidView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UuidView::default())
  }

  // custom_name: optional string
  pub fn has_custom_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn custom_name_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_custom_name().then(|| self.custom_name())
  }
  pub fn custom_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // custom_name_visible: optional bool
  pub fn custom_name_visible(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }

  // silent: optional bool
  pub fn silent(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }

  // glowing: optional bool
  pub fn glowing(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `EntityView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EntityView<'_> {}

// SAFETY:
// - `EntityView` is `Send` because while its alive a `EntityMut` cannot.
// - `EntityView` does not use thread-local data.
unsafe impl ::std::marker::Send for EntityView<'_> {}

impl<'msg> ::protobuf::AsView for EntityView<'msg> {
  type Proxied = Entity;
  fn as_view(&self) -> ::protobuf::View<'msg, Entity> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EntityView<'msg> {
  fn into_view<'shorter>(self) -> EntityView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Entity> for EntityView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Entity {
    let mut dst = Entity::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Entity> for EntityMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Entity {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Entity {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EntityView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EntityMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EntityMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Entity>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EntityMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EntityMut<'msg> {
  type Message = Entity;
}

impl ::std::fmt::Debug for EntityMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Entity>> for EntityMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Entity>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EntityMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Entity> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Entity {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // pos: optional message prost.minecraft_savedata.Vector3d
  pub fn has_pos(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_pos(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn pos_opt(&self) -> ::std::option::Option<super::Vector3dView<'_>> {
    self.has_pos().then(|| self.pos())
  }
  pub fn pos(&self) -> super::Vector3dView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3dView::default())
  }
  pub fn pos_mut(&mut self) -> super::Vector3dMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_pos(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3d>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // motion: optional message prost.minecraft_savedata.Vector3d
  pub fn has_motion(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_motion(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn motion_opt(&self) -> ::std::option::Option<super::Vector3dView<'_>> {
    self.has_motion().then(|| self.motion())
  }
  pub fn motion(&self) -> super::Vector3dView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3dView::default())
  }
  pub fn motion_mut(&mut self) -> super::Vector3dMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_motion(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3d>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // rotation: optional message prost.minecraft_savedata.Vector2f
  pub fn has_rotation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_rotation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn rotation_opt(&self) -> ::std::option::Option<super::Vector2fView<'_>> {
    self.has_rotation().then(|| self.rotation())
  }
  pub fn rotation(&self) -> super::Vector2fView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector2fView::default())
  }
  pub fn rotation_mut(&mut self) -> super::Vector2fMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_rotation(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector2f>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // fall_distance: optional float
  pub fn fall_distance(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fall_distance(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        4, val.into()
      )
    }
  }

  // fire: optional uint32
  pub fn fire(&self) -> u32 {
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
  pub fn set_fire(&mut self, val: u32) {
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

  // air: optional uint32
  pub fn air(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        6, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_air(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        6, val.into()
      )
    }
  }

  // on_ground: optional bool
  pub fn on_ground(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_on_ground(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

  // no_gravity: optional bool
  pub fn no_gravity(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_no_gravity(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        8, val.into()
      )
    }
  }

  // invulnerable: optional bool
  pub fn invulnerable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_invulnerable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        9, val.into()
      )
    }
  }

  // portal_cooldown: optional int32
  pub fn portal_cooldown(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        10, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_portal_cooldown(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        10, val.into()
      )
    }
  }

  // uuid: optional message prost.minecraft_savedata.Uuid
  pub fn has_uuid(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_uuid(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn uuid_opt(&self) -> ::std::option::Option<super::UuidView<'_>> {
    self.has_uuid().then(|| self.uuid())
  }
  pub fn uuid(&self) -> super::UuidView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UuidView::default())
  }
  pub fn uuid_mut(&mut self) -> super::UuidMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_uuid(&mut self,
    val: impl ::protobuf::IntoProxied<super::Uuid>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // custom_name: optional string
  pub fn has_custom_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_custom_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn custom_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_custom_name().then(|| self.custom_name())
  }
  pub fn custom_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_custom_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val);
    }
  }

  // custom_name_visible: optional bool
  pub fn custom_name_visible(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_custom_name_visible(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  // silent: optional bool
  pub fn silent(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_silent(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        14, val.into()
      )
    }
  }

  // glowing: optional bool
  pub fn glowing(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_glowing(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        15, val.into()
      )
    }
  }

}

// SAFETY:
// - `EntityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EntityMut<'_> {}

// SAFETY:
// - `EntityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EntityMut<'_> {}

impl<'msg> ::protobuf::AsView for EntityMut<'msg> {
  type Proxied = Entity;
  fn as_view(&self) -> ::protobuf::View<'_, Entity> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EntityMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Entity>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EntityMut<'msg> {
  type MutProxied = Entity;
  fn as_mut(&mut self) -> EntityMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EntityMut<'msg> {
  fn into_mut<'shorter>(self) -> EntityMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Entity {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Entity> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EntityView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EntityMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional string
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // pos: optional message prost.minecraft_savedata.Vector3d
  pub fn has_pos(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_pos(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn pos_opt(&self) -> ::std::option::Option<super::Vector3dView<'_>> {
    self.has_pos().then(|| self.pos())
  }
  pub fn pos(&self) -> super::Vector3dView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3dView::default())
  }
  pub fn pos_mut(&mut self) -> super::Vector3dMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_pos(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3d>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // motion: optional message prost.minecraft_savedata.Vector3d
  pub fn has_motion(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_motion(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn motion_opt(&self) -> ::std::option::Option<super::Vector3dView<'_>> {
    self.has_motion().then(|| self.motion())
  }
  pub fn motion(&self) -> super::Vector3dView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3dView::default())
  }
  pub fn motion_mut(&mut self) -> super::Vector3dMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_motion(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3d>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // rotation: optional message prost.minecraft_savedata.Vector2f
  pub fn has_rotation(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_rotation(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn rotation_opt(&self) -> ::std::option::Option<super::Vector2fView<'_>> {
    self.has_rotation().then(|| self.rotation())
  }
  pub fn rotation(&self) -> super::Vector2fView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector2fView::default())
  }
  pub fn rotation_mut(&mut self) -> super::Vector2fMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_rotation(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector2f>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // fall_distance: optional float
  pub fn fall_distance(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_fall_distance(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        4, val.into()
      )
    }
  }

  // fire: optional uint32
  pub fn fire(&self) -> u32 {
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
  pub fn set_fire(&mut self, val: u32) {
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

  // air: optional uint32
  pub fn air(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        6, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_air(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        6, val.into()
      )
    }
  }

  // on_ground: optional bool
  pub fn on_ground(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_on_ground(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

  // no_gravity: optional bool
  pub fn no_gravity(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_no_gravity(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        8, val.into()
      )
    }
  }

  // invulnerable: optional bool
  pub fn invulnerable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_invulnerable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        9, val.into()
      )
    }
  }

  // portal_cooldown: optional int32
  pub fn portal_cooldown(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        10, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_portal_cooldown(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        10, val.into()
      )
    }
  }

  // uuid: optional message prost.minecraft_savedata.Uuid
  pub fn has_uuid(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_uuid(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn uuid_opt(&self) -> ::std::option::Option<super::UuidView<'_>> {
    self.has_uuid().then(|| self.uuid())
  }
  pub fn uuid(&self) -> super::UuidView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UuidView::default())
  }
  pub fn uuid_mut(&mut self) -> super::UuidMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_uuid(&mut self,
    val: impl ::protobuf::IntoProxied<super::Uuid>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // custom_name: optional string
  pub fn has_custom_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_custom_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn custom_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_custom_name().then(|| self.custom_name())
  }
  pub fn custom_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_custom_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val);
    }
  }

  // custom_name_visible: optional bool
  pub fn custom_name_visible(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_custom_name_visible(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  // silent: optional bool
  pub fn silent(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        14, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_silent(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        14, val.into()
      )
    }
  }

  // glowing: optional bool
  pub fn glowing(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        15, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_glowing(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        15, val.into()
      )
    }
  }

}  // impl Entity

impl ::std::ops::Drop for Entity {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Entity {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Entity {
  type Proxied = Self;
  fn as_view(&self) -> EntityView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Entity {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EntityMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Entity {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__minecraft_0savedata__Entity_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X333!P)P)P/P/P/P(P31T/P/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__minecraft_0savedata__Entity_msg_init.0, &[<super::Vector3d as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Vector3d as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Vector2f as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Uuid as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__minecraft_0savedata__Entity_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Entity {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Entity {
  type Msg = Entity;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Entity> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Entity {
  type Msg = Entity;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Entity> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EntityMut<'_> {
  type Msg = Entity;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Entity> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EntityMut<'_> {
  type Msg = Entity;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Entity> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EntityView<'_> {
  type Msg = Entity;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Entity> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EntityMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__minecraft_0savedata__RecipeBook_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RecipeBook {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RecipeBook>
}

impl ::protobuf::Message for RecipeBook {
  type MessageView<'msg> = RecipeBookView<'msg>;
  type MessageMut<'msg> = RecipeBookMut<'msg>;
}

impl ::std::default::Default for RecipeBook {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RecipeBook {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RecipeBook` is `Sync` because it does not implement interior mutability.
//    Neither does `RecipeBookMut`.
unsafe impl ::std::marker::Sync for RecipeBook {}

// SAFETY:
// - `RecipeBook` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RecipeBook {}

impl ::protobuf::Proxied for RecipeBook {
  type View<'msg> = RecipeBookView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RecipeBook {}

impl ::protobuf::MutProxied for RecipeBook {
  type Mut<'msg> = RecipeBookMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RecipeBookView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RecipeBook>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RecipeBookView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RecipeBookView<'msg> {
  type Message = RecipeBook;
}

impl ::std::fmt::Debug for RecipeBookView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RecipeBookView<'_> {
  fn default() -> RecipeBookView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RecipeBook>> for RecipeBookView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RecipeBook>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RecipeBookView<'msg> {

  pub fn to_owned(&self) -> RecipeBook {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // recipes: repeated string
  pub fn recipes(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // to_be_displayed: repeated string
  pub fn to_be_displayed(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // is_filtering_craftable: optional bool
  pub fn is_filtering_craftable(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }

  // is_gui_open: optional bool
  pub fn is_gui_open(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }

  // is_furnace_filtering_craftable: optional bool
  pub fn is_furnace_filtering_craftable(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }

  // is_furnace_gui_open: optional bool
  pub fn is_furnace_gui_open(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }

  // is_blasting_furnace_filtering_craftable: optional bool
  pub fn is_blasting_furnace_filtering_craftable(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }

  // is_blasting_furnace_gui_open: optional bool
  pub fn is_blasting_furnace_gui_open(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }

  // is_smoker_filtering_craftable: optional bool
  pub fn is_smoker_filtering_craftable(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }

  // is_smoker_gui_open: optional bool
  pub fn is_smoker_gui_open(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `RecipeBookView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RecipeBookView<'_> {}

// SAFETY:
// - `RecipeBookView` is `Send` because while its alive a `RecipeBookMut` cannot.
// - `RecipeBookView` does not use thread-local data.
unsafe impl ::std::marker::Send for RecipeBookView<'_> {}

impl<'msg> ::protobuf::AsView for RecipeBookView<'msg> {
  type Proxied = RecipeBook;
  fn as_view(&self) -> ::protobuf::View<'msg, RecipeBook> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RecipeBookView<'msg> {
  fn into_view<'shorter>(self) -> RecipeBookView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RecipeBook> for RecipeBookView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RecipeBook {
    let mut dst = RecipeBook::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RecipeBook> for RecipeBookMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RecipeBook {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RecipeBook {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RecipeBookView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RecipeBookMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RecipeBookMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RecipeBook>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RecipeBookMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RecipeBookMut<'msg> {
  type Message = RecipeBook;
}

impl ::std::fmt::Debug for RecipeBookMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RecipeBook>> for RecipeBookMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RecipeBook>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RecipeBookMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RecipeBook> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RecipeBook {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // recipes: repeated string
  pub fn recipes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn recipes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_recipes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // to_be_displayed: repeated string
  pub fn to_be_displayed(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn to_be_displayed_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_to_be_displayed(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // is_filtering_craftable: optional bool
  pub fn is_filtering_craftable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_filtering_craftable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // is_gui_open: optional bool
  pub fn is_gui_open(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_gui_open(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // is_furnace_filtering_craftable: optional bool
  pub fn is_furnace_filtering_craftable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_furnace_filtering_craftable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

  // is_furnace_gui_open: optional bool
  pub fn is_furnace_gui_open(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_furnace_gui_open(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // is_blasting_furnace_filtering_craftable: optional bool
  pub fn is_blasting_furnace_filtering_craftable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_blasting_furnace_filtering_craftable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // is_blasting_furnace_gui_open: optional bool
  pub fn is_blasting_furnace_gui_open(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_blasting_furnace_gui_open(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

  // is_smoker_filtering_craftable: optional bool
  pub fn is_smoker_filtering_craftable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_smoker_filtering_craftable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        8, val.into()
      )
    }
  }

  // is_smoker_gui_open: optional bool
  pub fn is_smoker_gui_open(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_smoker_gui_open(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        9, val.into()
      )
    }
  }

}

// SAFETY:
// - `RecipeBookMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RecipeBookMut<'_> {}

// SAFETY:
// - `RecipeBookMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RecipeBookMut<'_> {}

impl<'msg> ::protobuf::AsView for RecipeBookMut<'msg> {
  type Proxied = RecipeBook;
  fn as_view(&self) -> ::protobuf::View<'_, RecipeBook> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RecipeBookMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RecipeBook>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RecipeBookMut<'msg> {
  type MutProxied = RecipeBook;
  fn as_mut(&mut self) -> RecipeBookMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RecipeBookMut<'msg> {
  fn into_mut<'shorter>(self) -> RecipeBookMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RecipeBook {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RecipeBook> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RecipeBookView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RecipeBookMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // recipes: repeated string
  pub fn recipes(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn recipes_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
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
  pub fn set_recipes(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // to_be_displayed: repeated string
  pub fn to_be_displayed(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn to_be_displayed_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
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
  pub fn set_to_be_displayed(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // is_filtering_craftable: optional bool
  pub fn is_filtering_craftable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        2, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_filtering_craftable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        2, val.into()
      )
    }
  }

  // is_gui_open: optional bool
  pub fn is_gui_open(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        3, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_gui_open(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        3, val.into()
      )
    }
  }

  // is_furnace_filtering_craftable: optional bool
  pub fn is_furnace_filtering_craftable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_furnace_filtering_craftable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

  // is_furnace_gui_open: optional bool
  pub fn is_furnace_gui_open(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_furnace_gui_open(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // is_blasting_furnace_filtering_craftable: optional bool
  pub fn is_blasting_furnace_filtering_craftable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        6, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_blasting_furnace_filtering_craftable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        6, val.into()
      )
    }
  }

  // is_blasting_furnace_gui_open: optional bool
  pub fn is_blasting_furnace_gui_open(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        7, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_blasting_furnace_gui_open(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        7, val.into()
      )
    }
  }

  // is_smoker_filtering_craftable: optional bool
  pub fn is_smoker_filtering_craftable(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        8, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_smoker_filtering_craftable(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        8, val.into()
      )
    }
  }

  // is_smoker_gui_open: optional bool
  pub fn is_smoker_gui_open(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        9, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_smoker_gui_open(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        9, val.into()
      )
    }
  }

}  // impl RecipeBook

impl ::std::ops::Drop for RecipeBook {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RecipeBook {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RecipeBook {
  type Proxied = Self;
  fn as_view(&self) -> RecipeBookView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RecipeBook {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RecipeBookMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RecipeBook {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__minecraft_0savedata__RecipeBook_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$ETET/P/P/P/P/P/P/P/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__minecraft_0savedata__RecipeBook_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__minecraft_0savedata__RecipeBook_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RecipeBook {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RecipeBook {
  type Msg = RecipeBook;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RecipeBook> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RecipeBook {
  type Msg = RecipeBook;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RecipeBook> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RecipeBookMut<'_> {
  type Msg = RecipeBook;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RecipeBook> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RecipeBookMut<'_> {
  type Msg = RecipeBook;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RecipeBook> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RecipeBookView<'_> {
  type Msg = RecipeBook;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RecipeBook> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RecipeBookMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__minecraft_0savedata__Vehicle_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Vehicle {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Vehicle>
}

impl ::protobuf::Message for Vehicle {
  type MessageView<'msg> = VehicleView<'msg>;
  type MessageMut<'msg> = VehicleMut<'msg>;
}

impl ::std::default::Default for Vehicle {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Vehicle {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Vehicle` is `Sync` because it does not implement interior mutability.
//    Neither does `VehicleMut`.
unsafe impl ::std::marker::Sync for Vehicle {}

// SAFETY:
// - `Vehicle` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Vehicle {}

impl ::protobuf::Proxied for Vehicle {
  type View<'msg> = VehicleView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Vehicle {}

impl ::protobuf::MutProxied for Vehicle {
  type Mut<'msg> = VehicleMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct VehicleView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vehicle>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VehicleView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for VehicleView<'msg> {
  type Message = Vehicle;
}

impl ::std::fmt::Debug for VehicleView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for VehicleView<'_> {
  fn default() -> VehicleView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Vehicle>> for VehicleView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vehicle>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VehicleView<'msg> {

  pub fn to_owned(&self) -> Vehicle {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // uuid: optional message prost.minecraft_savedata.Uuid
  pub fn has_uuid(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn uuid_opt(self) -> ::std::option::Option<super::UuidView<'msg>> {
    self.has_uuid().then(|| self.uuid())
  }
  pub fn uuid(self) -> super::UuidView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UuidView::default())
  }

  // entity: optional message prost.minecraft_savedata.Entity
  pub fn has_entity(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn entity_opt(self) -> ::std::option::Option<super::EntityView<'msg>> {
    self.has_entity().then(|| self.entity())
  }
  pub fn entity(self) -> super::EntityView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EntityView::default())
  }

}

// SAFETY:
// - `VehicleView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for VehicleView<'_> {}

// SAFETY:
// - `VehicleView` is `Send` because while its alive a `VehicleMut` cannot.
// - `VehicleView` does not use thread-local data.
unsafe impl ::std::marker::Send for VehicleView<'_> {}

impl<'msg> ::protobuf::AsView for VehicleView<'msg> {
  type Proxied = Vehicle;
  fn as_view(&self) -> ::protobuf::View<'msg, Vehicle> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VehicleView<'msg> {
  fn into_view<'shorter>(self) -> VehicleView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Vehicle> for VehicleView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vehicle {
    let mut dst = Vehicle::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Vehicle> for VehicleMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vehicle {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Vehicle {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VehicleView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VehicleMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct VehicleMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vehicle>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VehicleMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for VehicleMut<'msg> {
  type Message = Vehicle;
}

impl ::std::fmt::Debug for VehicleMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Vehicle>> for VehicleMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vehicle>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VehicleMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Vehicle> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Vehicle {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // uuid: optional message prost.minecraft_savedata.Uuid
  pub fn has_uuid(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_uuid(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn uuid_opt(&self) -> ::std::option::Option<super::UuidView<'_>> {
    self.has_uuid().then(|| self.uuid())
  }
  pub fn uuid(&self) -> super::UuidView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UuidView::default())
  }
  pub fn uuid_mut(&mut self) -> super::UuidMut<'_> {
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
  pub fn set_uuid(&mut self,
    val: impl ::protobuf::IntoProxied<super::Uuid>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // entity: optional message prost.minecraft_savedata.Entity
  pub fn has_entity(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_entity(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn entity_opt(&self) -> ::std::option::Option<super::EntityView<'_>> {
    self.has_entity().then(|| self.entity())
  }
  pub fn entity(&self) -> super::EntityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EntityView::default())
  }
  pub fn entity_mut(&mut self) -> super::EntityMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_entity(&mut self,
    val: impl ::protobuf::IntoProxied<super::Entity>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `VehicleMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for VehicleMut<'_> {}

// SAFETY:
// - `VehicleMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for VehicleMut<'_> {}

impl<'msg> ::protobuf::AsView for VehicleMut<'msg> {
  type Proxied = Vehicle;
  fn as_view(&self) -> ::protobuf::View<'_, Vehicle> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VehicleMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Vehicle>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for VehicleMut<'msg> {
  type MutProxied = Vehicle;
  fn as_mut(&mut self) -> VehicleMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for VehicleMut<'msg> {
  fn into_mut<'shorter>(self) -> VehicleMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Vehicle {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Vehicle> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> VehicleView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> VehicleMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // uuid: optional message prost.minecraft_savedata.Uuid
  pub fn has_uuid(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_uuid(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn uuid_opt(&self) -> ::std::option::Option<super::UuidView<'_>> {
    self.has_uuid().then(|| self.uuid())
  }
  pub fn uuid(&self) -> super::UuidView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::UuidView::default())
  }
  pub fn uuid_mut(&mut self) -> super::UuidMut<'_> {
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
  pub fn set_uuid(&mut self,
    val: impl ::protobuf::IntoProxied<super::Uuid>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // entity: optional message prost.minecraft_savedata.Entity
  pub fn has_entity(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_entity(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn entity_opt(&self) -> ::std::option::Option<super::EntityView<'_>> {
    self.has_entity().then(|| self.entity())
  }
  pub fn entity(&self) -> super::EntityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EntityView::default())
  }
  pub fn entity_mut(&mut self) -> super::EntityMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_entity(&mut self,
    val: impl ::protobuf::IntoProxied<super::Entity>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl Vehicle

impl ::std::ops::Drop for Vehicle {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Vehicle {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Vehicle {
  type Proxied = Self;
  fn as_view(&self) -> VehicleView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Vehicle {
  type MutProxied = Self;
  fn as_mut(&mut self) -> VehicleMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Vehicle {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__minecraft_0savedata__Vehicle_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__minecraft_0savedata__Vehicle_msg_init.0, &[<super::Uuid as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Entity as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__minecraft_0savedata__Vehicle_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Vehicle {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Vehicle {
  type Msg = Vehicle;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vehicle> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vehicle {
  type Msg = Vehicle;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vehicle> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for VehicleMut<'_> {
  type Msg = Vehicle;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vehicle> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VehicleMut<'_> {
  type Msg = Vehicle;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vehicle> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VehicleView<'_> {
  type Msg = Vehicle;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vehicle> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for VehicleMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__minecraft_0savedata__Player_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Player {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Player>
}

impl ::protobuf::Message for Player {
  type MessageView<'msg> = PlayerView<'msg>;
  type MessageMut<'msg> = PlayerMut<'msg>;
}

impl ::std::default::Default for Player {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Player {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Player` is `Sync` because it does not implement interior mutability.
//    Neither does `PlayerMut`.
unsafe impl ::std::marker::Sync for Player {}

// SAFETY:
// - `Player` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Player {}

impl ::protobuf::Proxied for Player {
  type View<'msg> = PlayerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Player {}

impl ::protobuf::MutProxied for Player {
  type Mut<'msg> = PlayerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PlayerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Player>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PlayerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PlayerView<'msg> {
  type Message = Player;
}

impl ::std::fmt::Debug for PlayerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PlayerView<'_> {
  fn default() -> PlayerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Player>> for PlayerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Player>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PlayerView<'msg> {

  pub fn to_owned(&self) -> Player {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // game_type: optional enum prost.minecraft_savedata.GameType
  pub fn game_type(self) -> super::GameType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::GameType::Survival).into()
      ).try_into().unwrap()
    }
  }

  // previous_game_type: optional enum prost.minecraft_savedata.GameType
  pub fn previous_game_type(self) -> super::GameType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::GameType::Survival).into()
      ).try_into().unwrap()
    }
  }

  // score: optional int64
  pub fn score(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // dimension: optional string
  pub fn dimension(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // selected_item_slot: optional uint32
  pub fn selected_item_slot(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // selected_item: optional message prost.minecraft_savedata.Item
  pub fn has_selected_item(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn selected_item_opt(self) -> ::std::option::Option<super::ItemView<'msg>> {
    self.has_selected_item().then(|| self.selected_item())
  }
  pub fn selected_item(self) -> super::ItemView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ItemView::default())
  }

  // spawn_dimension: optional string
  pub fn has_spawn_dimension(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn spawn_dimension_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_spawn_dimension().then(|| self.spawn_dimension())
  }
  pub fn spawn_dimension(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // spawn_x: optional int64
  pub fn spawn_x(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        7, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // spawn_y: optional int64
  pub fn spawn_y(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        8, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // spawn_z: optional int64
  pub fn spawn_z(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        9, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // spawn_forced: optional bool
  pub fn has_spawn_forced(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn spawn_forced_opt(self) -> ::std::option::Option<bool> {
    self.has_spawn_forced().then(|| self.spawn_forced())
  }
  pub fn spawn_forced(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }

  // sleep_timer: optional uint32
  pub fn sleep_timer(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        11, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // food_exhaustion_level: optional float
  pub fn food_exhaustion_level(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        12, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // food_saturation_level: optional float
  pub fn food_saturation_level(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        13, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // food_tick_timer: optional uint32
  pub fn food_tick_timer(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        14, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // xp_level: optional uint32
  pub fn xp_level(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        15, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // xp_p: optional float
  pub fn xp_p(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        16, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // xp_total: optional int32
  pub fn xp_total(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        17, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // xp_seed: optional int32
  pub fn xp_seed(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        18, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // inventory: repeated message prost.minecraft_savedata.Item
  pub fn inventory(self) -> ::protobuf::RepeatedView<'msg, super::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        19
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ender_items: repeated message prost.minecraft_savedata.Item
  pub fn ender_items(self) -> ::protobuf::RepeatedView<'msg, super::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // abilities: optional message prost.minecraft_savedata.Abilities
  pub fn has_abilities(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn abilities_opt(self) -> ::std::option::Option<super::AbilitiesView<'msg>> {
    self.has_abilities().then(|| self.abilities())
  }
  pub fn abilities(self) -> super::AbilitiesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AbilitiesView::default())
  }

  // entered_nether_position: optional message prost.minecraft_savedata.Vector3d
  pub fn has_entered_nether_position(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn entered_nether_position_opt(self) -> ::std::option::Option<super::Vector3dView<'msg>> {
    self.has_entered_nether_position().then(|| self.entered_nether_position())
  }
  pub fn entered_nether_position(self) -> super::Vector3dView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3dView::default())
  }

  // root_vehicle: optional message prost.minecraft_savedata.Vehicle
  pub fn has_root_vehicle(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn root_vehicle_opt(self) -> ::std::option::Option<super::VehicleView<'msg>> {
    self.has_root_vehicle().then(|| self.root_vehicle())
  }
  pub fn root_vehicle(self) -> super::VehicleView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehicleView::default())
  }

  // shoulder_entity_left: optional message prost.minecraft_savedata.Entity
  pub fn has_shoulder_entity_left(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn shoulder_entity_left_opt(self) -> ::std::option::Option<super::EntityView<'msg>> {
    self.has_shoulder_entity_left().then(|| self.shoulder_entity_left())
  }
  pub fn shoulder_entity_left(self) -> super::EntityView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EntityView::default())
  }

  // shoulder_entity_right: optional message prost.minecraft_savedata.Entity
  pub fn has_shoulder_entity_right(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn shoulder_entity_right_opt(self) -> ::std::option::Option<super::EntityView<'msg>> {
    self.has_shoulder_entity_right().then(|| self.shoulder_entity_right())
  }
  pub fn shoulder_entity_right(self) -> super::EntityView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EntityView::default())
  }

  // seen_credits: optional bool
  pub fn seen_credits(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }

  // recipe_book: optional message prost.minecraft_savedata.RecipeBook
  pub fn has_recipe_book(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(27)
    }
  }
  pub fn recipe_book_opt(self) -> ::std::option::Option<super::RecipeBookView<'msg>> {
    self.has_recipe_book().then(|| self.recipe_book())
  }
  pub fn recipe_book(self) -> super::RecipeBookView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(27)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RecipeBookView::default())
  }

}

// SAFETY:
// - `PlayerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PlayerView<'_> {}

// SAFETY:
// - `PlayerView` is `Send` because while its alive a `PlayerMut` cannot.
// - `PlayerView` does not use thread-local data.
unsafe impl ::std::marker::Send for PlayerView<'_> {}

impl<'msg> ::protobuf::AsView for PlayerView<'msg> {
  type Proxied = Player;
  fn as_view(&self) -> ::protobuf::View<'msg, Player> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PlayerView<'msg> {
  fn into_view<'shorter>(self) -> PlayerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Player> for PlayerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Player {
    let mut dst = Player::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Player> for PlayerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Player {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Player {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PlayerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PlayerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PlayerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Player>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PlayerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PlayerMut<'msg> {
  type Message = Player;
}

impl ::std::fmt::Debug for PlayerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Player>> for PlayerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Player>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PlayerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Player> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Player {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // game_type: optional enum prost.minecraft_savedata.GameType
  pub fn game_type(&self) -> super::GameType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::GameType::Survival).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_game_type(&mut self, val: super::GameType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // previous_game_type: optional enum prost.minecraft_savedata.GameType
  pub fn previous_game_type(&self) -> super::GameType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::GameType::Survival).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_previous_game_type(&mut self, val: super::GameType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // score: optional int64
  pub fn score(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_score(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // dimension: optional string
  pub fn dimension(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_dimension(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // selected_item_slot: optional uint32
  pub fn selected_item_slot(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_selected_item_slot(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        4, val.into()
      )
    }
  }

  // selected_item: optional message prost.minecraft_savedata.Item
  pub fn has_selected_item(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_selected_item(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn selected_item_opt(&self) -> ::std::option::Option<super::ItemView<'_>> {
    self.has_selected_item().then(|| self.selected_item())
  }
  pub fn selected_item(&self) -> super::ItemView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ItemView::default())
  }
  pub fn selected_item_mut(&mut self) -> super::ItemMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_selected_item(&mut self,
    val: impl ::protobuf::IntoProxied<super::Item>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // spawn_dimension: optional string
  pub fn has_spawn_dimension(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_spawn_dimension(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn spawn_dimension_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_spawn_dimension().then(|| self.spawn_dimension())
  }
  pub fn spawn_dimension(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_spawn_dimension(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // spawn_x: optional int64
  pub fn spawn_x(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        7, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_spawn_x(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        7, val.into()
      )
    }
  }

  // spawn_y: optional int64
  pub fn spawn_y(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        8, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_spawn_y(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        8, val.into()
      )
    }
  }

  // spawn_z: optional int64
  pub fn spawn_z(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        9, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_spawn_z(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        9, val.into()
      )
    }
  }

  // spawn_forced: optional bool
  pub fn has_spawn_forced(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_spawn_forced(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn spawn_forced_opt(&self) -> ::std::option::Option<bool> {
    self.has_spawn_forced().then(|| self.spawn_forced())
  }
  pub fn spawn_forced(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_spawn_forced(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        10, val.into()
      )
    }
  }

  // sleep_timer: optional uint32
  pub fn sleep_timer(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        11, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_sleep_timer(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        11, val.into()
      )
    }
  }

  // food_exhaustion_level: optional float
  pub fn food_exhaustion_level(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        12, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_food_exhaustion_level(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        12, val.into()
      )
    }
  }

  // food_saturation_level: optional float
  pub fn food_saturation_level(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        13, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_food_saturation_level(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        13, val.into()
      )
    }
  }

  // food_tick_timer: optional uint32
  pub fn food_tick_timer(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        14, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_food_tick_timer(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        14, val.into()
      )
    }
  }

  // xp_level: optional uint32
  pub fn xp_level(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        15, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_xp_level(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        15, val.into()
      )
    }
  }

  // xp_p: optional float
  pub fn xp_p(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        16, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_xp_p(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        16, val.into()
      )
    }
  }

  // xp_total: optional int32
  pub fn xp_total(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        17, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_xp_total(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        17, val.into()
      )
    }
  }

  // xp_seed: optional int32
  pub fn xp_seed(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        18, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_xp_seed(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        18, val.into()
      )
    }
  }

  // inventory: repeated message prost.minecraft_savedata.Item
  pub fn inventory(&self) -> ::protobuf::RepeatedView<'_, super::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        19
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn inventory_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Item> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        19,
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
  pub fn set_inventory(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        src);
    }
  }

  // ender_items: repeated message prost.minecraft_savedata.Item
  pub fn ender_items(&self) -> ::protobuf::RepeatedView<'_, super::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ender_items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Item> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        20,
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
  pub fn set_ender_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        src);
    }
  }

  // abilities: optional message prost.minecraft_savedata.Abilities
  pub fn has_abilities(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_abilities(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn abilities_opt(&self) -> ::std::option::Option<super::AbilitiesView<'_>> {
    self.has_abilities().then(|| self.abilities())
  }
  pub fn abilities(&self) -> super::AbilitiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AbilitiesView::default())
  }
  pub fn abilities_mut(&mut self) -> super::AbilitiesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         21, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_abilities(&mut self,
    val: impl ::protobuf::IntoProxied<super::Abilities>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val
      );
    }
  }

  // entered_nether_position: optional message prost.minecraft_savedata.Vector3d
  pub fn has_entered_nether_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_entered_nether_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn entered_nether_position_opt(&self) -> ::std::option::Option<super::Vector3dView<'_>> {
    self.has_entered_nether_position().then(|| self.entered_nether_position())
  }
  pub fn entered_nether_position(&self) -> super::Vector3dView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3dView::default())
  }
  pub fn entered_nether_position_mut(&mut self) -> super::Vector3dMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         22, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_entered_nether_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3d>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // root_vehicle: optional message prost.minecraft_savedata.Vehicle
  pub fn has_root_vehicle(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_root_vehicle(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn root_vehicle_opt(&self) -> ::std::option::Option<super::VehicleView<'_>> {
    self.has_root_vehicle().then(|| self.root_vehicle())
  }
  pub fn root_vehicle(&self) -> super::VehicleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehicleView::default())
  }
  pub fn root_vehicle_mut(&mut self) -> super::VehicleMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         23, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_root_vehicle(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vehicle>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val
      );
    }
  }

  // shoulder_entity_left: optional message prost.minecraft_savedata.Entity
  pub fn has_shoulder_entity_left(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_shoulder_entity_left(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn shoulder_entity_left_opt(&self) -> ::std::option::Option<super::EntityView<'_>> {
    self.has_shoulder_entity_left().then(|| self.shoulder_entity_left())
  }
  pub fn shoulder_entity_left(&self) -> super::EntityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EntityView::default())
  }
  pub fn shoulder_entity_left_mut(&mut self) -> super::EntityMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         24, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_shoulder_entity_left(&mut self,
    val: impl ::protobuf::IntoProxied<super::Entity>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // shoulder_entity_right: optional message prost.minecraft_savedata.Entity
  pub fn has_shoulder_entity_right(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn clear_shoulder_entity_right(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        25
      );
    }
  }
  pub fn shoulder_entity_right_opt(&self) -> ::std::option::Option<super::EntityView<'_>> {
    self.has_shoulder_entity_right().then(|| self.shoulder_entity_right())
  }
  pub fn shoulder_entity_right(&self) -> super::EntityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EntityView::default())
  }
  pub fn shoulder_entity_right_mut(&mut self) -> super::EntityMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         25, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_shoulder_entity_right(&mut self,
    val: impl ::protobuf::IntoProxied<super::Entity>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val
      );
    }
  }

  // seen_credits: optional bool
  pub fn seen_credits(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_seen_credits(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        26, val.into()
      )
    }
  }

  // recipe_book: optional message prost.minecraft_savedata.RecipeBook
  pub fn has_recipe_book(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(27)
    }
  }
  pub fn clear_recipe_book(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        27
      );
    }
  }
  pub fn recipe_book_opt(&self) -> ::std::option::Option<super::RecipeBookView<'_>> {
    self.has_recipe_book().then(|| self.recipe_book())
  }
  pub fn recipe_book(&self) -> super::RecipeBookView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(27)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RecipeBookView::default())
  }
  pub fn recipe_book_mut(&mut self) -> super::RecipeBookMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         27, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_recipe_book(&mut self,
    val: impl ::protobuf::IntoProxied<super::RecipeBook>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        27,
        val
      );
    }
  }

}

// SAFETY:
// - `PlayerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PlayerMut<'_> {}

// SAFETY:
// - `PlayerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PlayerMut<'_> {}

impl<'msg> ::protobuf::AsView for PlayerMut<'msg> {
  type Proxied = Player;
  fn as_view(&self) -> ::protobuf::View<'_, Player> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PlayerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Player>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PlayerMut<'msg> {
  type MutProxied = Player;
  fn as_mut(&mut self) -> PlayerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PlayerMut<'msg> {
  fn into_mut<'shorter>(self) -> PlayerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Player {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Player> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PlayerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PlayerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // game_type: optional enum prost.minecraft_savedata.GameType
  pub fn game_type(&self) -> super::GameType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (super::GameType::Survival).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_game_type(&mut self, val: super::GameType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // previous_game_type: optional enum prost.minecraft_savedata.GameType
  pub fn previous_game_type(&self) -> super::GameType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::GameType::Survival).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_previous_game_type(&mut self, val: super::GameType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // score: optional int64
  pub fn score(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        2, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_score(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        2, val.into()
      )
    }
  }

  // dimension: optional string
  pub fn dimension(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_dimension(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // selected_item_slot: optional uint32
  pub fn selected_item_slot(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_selected_item_slot(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        4, val.into()
      )
    }
  }

  // selected_item: optional message prost.minecraft_savedata.Item
  pub fn has_selected_item(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_selected_item(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn selected_item_opt(&self) -> ::std::option::Option<super::ItemView<'_>> {
    self.has_selected_item().then(|| self.selected_item())
  }
  pub fn selected_item(&self) -> super::ItemView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ItemView::default())
  }
  pub fn selected_item_mut(&mut self) -> super::ItemMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_selected_item(&mut self,
    val: impl ::protobuf::IntoProxied<super::Item>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // spawn_dimension: optional string
  pub fn has_spawn_dimension(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_spawn_dimension(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn spawn_dimension_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_spawn_dimension().then(|| self.spawn_dimension())
  }
  pub fn spawn_dimension(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_spawn_dimension(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // spawn_x: optional int64
  pub fn spawn_x(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        7, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_spawn_x(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        7, val.into()
      )
    }
  }

  // spawn_y: optional int64
  pub fn spawn_y(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        8, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_spawn_y(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        8, val.into()
      )
    }
  }

  // spawn_z: optional int64
  pub fn spawn_z(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        9, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_spawn_z(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        9, val.into()
      )
    }
  }

  // spawn_forced: optional bool
  pub fn has_spawn_forced(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_spawn_forced(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn spawn_forced_opt(&self) -> ::std::option::Option<bool> {
    self.has_spawn_forced().then(|| self.spawn_forced())
  }
  pub fn spawn_forced(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        10, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_spawn_forced(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        10, val.into()
      )
    }
  }

  // sleep_timer: optional uint32
  pub fn sleep_timer(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        11, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_sleep_timer(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        11, val.into()
      )
    }
  }

  // food_exhaustion_level: optional float
  pub fn food_exhaustion_level(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        12, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_food_exhaustion_level(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        12, val.into()
      )
    }
  }

  // food_saturation_level: optional float
  pub fn food_saturation_level(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        13, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_food_saturation_level(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        13, val.into()
      )
    }
  }

  // food_tick_timer: optional uint32
  pub fn food_tick_timer(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        14, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_food_tick_timer(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        14, val.into()
      )
    }
  }

  // xp_level: optional uint32
  pub fn xp_level(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        15, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_xp_level(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        15, val.into()
      )
    }
  }

  // xp_p: optional float
  pub fn xp_p(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        16, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_xp_p(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        16, val.into()
      )
    }
  }

  // xp_total: optional int32
  pub fn xp_total(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        17, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_xp_total(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        17, val.into()
      )
    }
  }

  // xp_seed: optional int32
  pub fn xp_seed(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        18, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_xp_seed(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        18, val.into()
      )
    }
  }

  // inventory: repeated message prost.minecraft_savedata.Item
  pub fn inventory(&self) -> ::protobuf::RepeatedView<'_, super::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        19
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn inventory_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Item> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        19,
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
  pub fn set_inventory(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        src);
    }
  }

  // ender_items: repeated message prost.minecraft_savedata.Item
  pub fn ender_items(&self) -> ::protobuf::RepeatedView<'_, super::Item> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        20
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Item>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn ender_items_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Item> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        20,
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
  pub fn set_ender_items(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Item>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        src);
    }
  }

  // abilities: optional message prost.minecraft_savedata.Abilities
  pub fn has_abilities(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(21)
    }
  }
  pub fn clear_abilities(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        21
      );
    }
  }
  pub fn abilities_opt(&self) -> ::std::option::Option<super::AbilitiesView<'_>> {
    self.has_abilities().then(|| self.abilities())
  }
  pub fn abilities(&self) -> super::AbilitiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(21)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AbilitiesView::default())
  }
  pub fn abilities_mut(&mut self) -> super::AbilitiesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         21, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_abilities(&mut self,
    val: impl ::protobuf::IntoProxied<super::Abilities>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val
      );
    }
  }

  // entered_nether_position: optional message prost.minecraft_savedata.Vector3d
  pub fn has_entered_nether_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(22)
    }
  }
  pub fn clear_entered_nether_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        22
      );
    }
  }
  pub fn entered_nether_position_opt(&self) -> ::std::option::Option<super::Vector3dView<'_>> {
    self.has_entered_nether_position().then(|| self.entered_nether_position())
  }
  pub fn entered_nether_position(&self) -> super::Vector3dView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(22)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3dView::default())
  }
  pub fn entered_nether_position_mut(&mut self) -> super::Vector3dMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         22, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_entered_nether_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3d>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        val
      );
    }
  }

  // root_vehicle: optional message prost.minecraft_savedata.Vehicle
  pub fn has_root_vehicle(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(23)
    }
  }
  pub fn clear_root_vehicle(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        23
      );
    }
  }
  pub fn root_vehicle_opt(&self) -> ::std::option::Option<super::VehicleView<'_>> {
    self.has_root_vehicle().then(|| self.root_vehicle())
  }
  pub fn root_vehicle(&self) -> super::VehicleView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(23)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehicleView::default())
  }
  pub fn root_vehicle_mut(&mut self) -> super::VehicleMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         23, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_root_vehicle(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vehicle>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        val
      );
    }
  }

  // shoulder_entity_left: optional message prost.minecraft_savedata.Entity
  pub fn has_shoulder_entity_left(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_shoulder_entity_left(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn shoulder_entity_left_opt(&self) -> ::std::option::Option<super::EntityView<'_>> {
    self.has_shoulder_entity_left().then(|| self.shoulder_entity_left())
  }
  pub fn shoulder_entity_left(&self) -> super::EntityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EntityView::default())
  }
  pub fn shoulder_entity_left_mut(&mut self) -> super::EntityMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         24, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_shoulder_entity_left(&mut self,
    val: impl ::protobuf::IntoProxied<super::Entity>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // shoulder_entity_right: optional message prost.minecraft_savedata.Entity
  pub fn has_shoulder_entity_right(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(25)
    }
  }
  pub fn clear_shoulder_entity_right(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        25
      );
    }
  }
  pub fn shoulder_entity_right_opt(&self) -> ::std::option::Option<super::EntityView<'_>> {
    self.has_shoulder_entity_right().then(|| self.shoulder_entity_right())
  }
  pub fn shoulder_entity_right(&self) -> super::EntityView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(25)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::EntityView::default())
  }
  pub fn shoulder_entity_right_mut(&mut self) -> super::EntityMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         25, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_shoulder_entity_right(&mut self,
    val: impl ::protobuf::IntoProxied<super::Entity>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        val
      );
    }
  }

  // seen_credits: optional bool
  pub fn seen_credits(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_seen_credits(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        26, val.into()
      )
    }
  }

  // recipe_book: optional message prost.minecraft_savedata.RecipeBook
  pub fn has_recipe_book(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(27)
    }
  }
  pub fn clear_recipe_book(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        27
      );
    }
  }
  pub fn recipe_book_opt(&self) -> ::std::option::Option<super::RecipeBookView<'_>> {
    self.has_recipe_book().then(|| self.recipe_book())
  }
  pub fn recipe_book(&self) -> super::RecipeBookView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(27)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::RecipeBookView::default())
  }
  pub fn recipe_book_mut(&mut self) -> super::RecipeBookMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         27, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_recipe_book(&mut self,
    val: impl ::protobuf::IntoProxied<super::RecipeBook>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        27,
        val
      );
    }
  }

}  // impl Player

impl ::std::ops::Drop for Player {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Player {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Player {
  type Proxied = Self;
  fn as_view(&self) -> PlayerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Player {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PlayerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Player {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__minecraft_0savedata__Player_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$.P.P+P1X)P31T+P+P+P/)P!P!P)P)P!P(P(PGG33333/P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__minecraft_0savedata__Player_msg_init.0, &[<super::Item as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Item as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Item as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Abilities as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Vector3d as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Vehicle as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Entity as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Entity as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::RecipeBook as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__minecraft_0savedata__Player_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Player {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Player {
  type Msg = Player;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Player> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Player {
  type Msg = Player;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Player> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PlayerMut<'_> {
  type Msg = Player;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Player> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PlayerMut<'_> {
  type Msg = Player;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Player> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PlayerView<'_> {
  type Msg = Player;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Player> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PlayerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__minecraft_0savedata__Players_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Players {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Players>
}

impl ::protobuf::Message for Players {
  type MessageView<'msg> = PlayersView<'msg>;
  type MessageMut<'msg> = PlayersMut<'msg>;
}

impl ::std::default::Default for Players {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Players {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Players` is `Sync` because it does not implement interior mutability.
//    Neither does `PlayersMut`.
unsafe impl ::std::marker::Sync for Players {}

// SAFETY:
// - `Players` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Players {}

impl ::protobuf::Proxied for Players {
  type View<'msg> = PlayersView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Players {}

impl ::protobuf::MutProxied for Players {
  type Mut<'msg> = PlayersMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PlayersView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Players>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PlayersView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PlayersView<'msg> {
  type Message = Players;
}

impl ::std::fmt::Debug for PlayersView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PlayersView<'_> {
  fn default() -> PlayersView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Players>> for PlayersView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Players>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PlayersView<'msg> {

  pub fn to_owned(&self) -> Players {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // players: repeated message prost.minecraft_savedata.Player
  pub fn players(self) -> ::protobuf::RepeatedView<'msg, super::Player> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Player>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `PlayersView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PlayersView<'_> {}

// SAFETY:
// - `PlayersView` is `Send` because while its alive a `PlayersMut` cannot.
// - `PlayersView` does not use thread-local data.
unsafe impl ::std::marker::Send for PlayersView<'_> {}

impl<'msg> ::protobuf::AsView for PlayersView<'msg> {
  type Proxied = Players;
  fn as_view(&self) -> ::protobuf::View<'msg, Players> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PlayersView<'msg> {
  fn into_view<'shorter>(self) -> PlayersView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Players> for PlayersView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Players {
    let mut dst = Players::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Players> for PlayersMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Players {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Players {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PlayersView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PlayersMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PlayersMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Players>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PlayersMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PlayersMut<'msg> {
  type Message = Players;
}

impl ::std::fmt::Debug for PlayersMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Players>> for PlayersMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Players>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PlayersMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Players> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Players {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // players: repeated message prost.minecraft_savedata.Player
  pub fn players(&self) -> ::protobuf::RepeatedView<'_, super::Player> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Player>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn players_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Player> {
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
  pub fn set_players(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Player>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `PlayersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PlayersMut<'_> {}

// SAFETY:
// - `PlayersMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PlayersMut<'_> {}

impl<'msg> ::protobuf::AsView for PlayersMut<'msg> {
  type Proxied = Players;
  fn as_view(&self) -> ::protobuf::View<'_, Players> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PlayersMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Players>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PlayersMut<'msg> {
  type MutProxied = Players;
  fn as_mut(&mut self) -> PlayersMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PlayersMut<'msg> {
  fn into_mut<'shorter>(self) -> PlayersMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Players {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Players> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PlayersView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PlayersMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // players: repeated message prost.minecraft_savedata.Player
  pub fn players(&self) -> ::protobuf::RepeatedView<'_, super::Player> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Player>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn players_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Player> {
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
  pub fn set_players(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Player>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Players

impl ::std::ops::Drop for Players {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Players {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Players {
  type Proxied = Self;
  fn as_view(&self) -> PlayersView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Players {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PlayersMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Players {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__minecraft_0savedata__Players_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__minecraft_0savedata__Players_msg_init.0, &[<super::Player as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__minecraft_0savedata__Players_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Players {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Players {
  type Msg = Players;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Players> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Players {
  type Msg = Players;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Players> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PlayersMut<'_> {
  type Msg = Players;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Players> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PlayersMut<'_> {
  type Msg = Players;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Players> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PlayersView<'_> {
  type Msg = Players;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Players> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PlayersMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GameType(i32);

#[allow(non_upper_case_globals)]
impl GameType {
  pub const Survival: GameType = GameType(0);
  pub const Creative: GameType = GameType(1);
  pub const Adventure: GameType = GameType(2);
  pub const Spectator: GameType = GameType(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Survival",
      1 => "Creative",
      2 => "Adventure",
      3 => "Spectator",
      _ => return None
    })
  }
}

impl ::std::convert::From<GameType> for i32 {
  fn from(val: GameType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for GameType {
  fn from(val: i32) -> GameType {
    Self(val)
  }
}

impl ::std::default::Default for GameType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for GameType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "GameType::{}", constant_name)
    } else {
      write!(f, "GameType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for GameType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for GameType {}

impl ::protobuf::Proxied for GameType {
  type View<'a> = GameType;
}

impl ::protobuf::AsView for GameType {
  type Proxied = GameType;

  fn as_view(&self) -> GameType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GameType {
  fn into_view<'shorter>(self) -> GameType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for GameType {
  const NAME: &'static str = "GameType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for GameType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


