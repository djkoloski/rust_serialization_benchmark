const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__mk48__Vector2f_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
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
        super::prost__mk48__Vector2f_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$!P!P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__mk48__Vector2f_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__mk48__Vector2f_msg_init.0)
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
pub(crate) static mut prost__mk48__Transform_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Transform {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Transform>
}

impl ::protobuf::Message for Transform {
  type MessageView<'msg> = TransformView<'msg>;
  type MessageMut<'msg> = TransformMut<'msg>;
}

impl ::std::default::Default for Transform {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Transform {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Transform` is `Sync` because it does not implement interior mutability.
//    Neither does `TransformMut`.
unsafe impl ::std::marker::Sync for Transform {}

// SAFETY:
// - `Transform` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Transform {}

impl ::protobuf::Proxied for Transform {
  type View<'msg> = TransformView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Transform {}

impl ::protobuf::MutProxied for Transform {
  type Mut<'msg> = TransformMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TransformView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Transform>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TransformView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TransformView<'msg> {
  type Message = Transform;
}

impl ::std::fmt::Debug for TransformView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TransformView<'_> {
  fn default() -> TransformView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Transform>> for TransformView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Transform>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TransformView<'msg> {

  pub fn to_owned(&self) -> Transform {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // altitude: optional int32
  pub fn altitude(self) -> i32 {
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

  // angle: optional uint32
  pub fn angle(self) -> u32 {
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

  // position: optional message prost.mk48.Vector2f
  pub fn has_position(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn position_opt(self) -> ::std::option::Option<super::Vector2fView<'msg>> {
    self.has_position().then(|| self.position())
  }
  pub fn position(self) -> super::Vector2fView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector2fView::default())
  }

  // velocity: optional int32
  pub fn velocity(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `TransformView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TransformView<'_> {}

// SAFETY:
// - `TransformView` is `Send` because while its alive a `TransformMut` cannot.
// - `TransformView` does not use thread-local data.
unsafe impl ::std::marker::Send for TransformView<'_> {}

impl<'msg> ::protobuf::AsView for TransformView<'msg> {
  type Proxied = Transform;
  fn as_view(&self) -> ::protobuf::View<'msg, Transform> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TransformView<'msg> {
  fn into_view<'shorter>(self) -> TransformView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Transform> for TransformView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Transform {
    let mut dst = Transform::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Transform> for TransformMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Transform {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Transform {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TransformView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TransformMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TransformMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Transform>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TransformMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TransformMut<'msg> {
  type Message = Transform;
}

impl ::std::fmt::Debug for TransformMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Transform>> for TransformMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Transform>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TransformMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Transform> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Transform {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // altitude: optional int32
  pub fn altitude(&self) -> i32 {
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
  pub fn set_altitude(&mut self, val: i32) {
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

  // angle: optional uint32
  pub fn angle(&self) -> u32 {
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
  pub fn set_angle(&mut self, val: u32) {
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

  // position: optional message prost.mk48.Vector2f
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn position_opt(&self) -> ::std::option::Option<super::Vector2fView<'_>> {
    self.has_position().then(|| self.position())
  }
  pub fn position(&self) -> super::Vector2fView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector2fView::default())
  }
  pub fn position_mut(&mut self) -> super::Vector2fMut<'_> {
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
  pub fn set_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector2f>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // velocity: optional int32
  pub fn velocity(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_velocity(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `TransformMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TransformMut<'_> {}

// SAFETY:
// - `TransformMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TransformMut<'_> {}

impl<'msg> ::protobuf::AsView for TransformMut<'msg> {
  type Proxied = Transform;
  fn as_view(&self) -> ::protobuf::View<'_, Transform> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TransformMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Transform>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TransformMut<'msg> {
  type MutProxied = Transform;
  fn as_mut(&mut self) -> TransformMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TransformMut<'msg> {
  fn into_mut<'shorter>(self) -> TransformMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Transform {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Transform> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TransformView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TransformMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // altitude: optional int32
  pub fn altitude(&self) -> i32 {
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
  pub fn set_altitude(&mut self, val: i32) {
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

  // angle: optional uint32
  pub fn angle(&self) -> u32 {
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
  pub fn set_angle(&mut self, val: u32) {
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

  // position: optional message prost.mk48.Vector2f
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn position_opt(&self) -> ::std::option::Option<super::Vector2fView<'_>> {
    self.has_position().then(|| self.position())
  }
  pub fn position(&self) -> super::Vector2fView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector2fView::default())
  }
  pub fn position_mut(&mut self) -> super::Vector2fMut<'_> {
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
  pub fn set_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector2f>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // velocity: optional int32
  pub fn velocity(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_velocity(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

}  // impl Transform

impl ::std::ops::Drop for Transform {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Transform {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Transform {
  type Proxied = Self;
  fn as_view(&self) -> TransformView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Transform {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TransformMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Transform {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__mk48__Transform_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P)P3(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__mk48__Transform_msg_init.0, &[<super::Vector2f as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__mk48__Transform_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Transform {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Transform {
  type Msg = Transform;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Transform> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Transform {
  type Msg = Transform;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Transform> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TransformMut<'_> {
  type Msg = Transform;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Transform> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TransformMut<'_> {
  type Msg = Transform;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Transform> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TransformView<'_> {
  type Msg = Transform;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Transform> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TransformMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__mk48__Guidance_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Guidance {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Guidance>
}

impl ::protobuf::Message for Guidance {
  type MessageView<'msg> = GuidanceView<'msg>;
  type MessageMut<'msg> = GuidanceMut<'msg>;
}

impl ::std::default::Default for Guidance {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Guidance {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Guidance` is `Sync` because it does not implement interior mutability.
//    Neither does `GuidanceMut`.
unsafe impl ::std::marker::Sync for Guidance {}

// SAFETY:
// - `Guidance` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Guidance {}

impl ::protobuf::Proxied for Guidance {
  type View<'msg> = GuidanceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Guidance {}

impl ::protobuf::MutProxied for Guidance {
  type Mut<'msg> = GuidanceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GuidanceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Guidance>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GuidanceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GuidanceView<'msg> {
  type Message = Guidance;
}

impl ::std::fmt::Debug for GuidanceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GuidanceView<'_> {
  fn default() -> GuidanceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Guidance>> for GuidanceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Guidance>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GuidanceView<'msg> {

  pub fn to_owned(&self) -> Guidance {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // angle: optional uint32
  pub fn angle(self) -> u32 {
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

  // submerge: optional bool
  pub fn submerge(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

  // velocity: optional int32
  pub fn velocity(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `GuidanceView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GuidanceView<'_> {}

// SAFETY:
// - `GuidanceView` is `Send` because while its alive a `GuidanceMut` cannot.
// - `GuidanceView` does not use thread-local data.
unsafe impl ::std::marker::Send for GuidanceView<'_> {}

impl<'msg> ::protobuf::AsView for GuidanceView<'msg> {
  type Proxied = Guidance;
  fn as_view(&self) -> ::protobuf::View<'msg, Guidance> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GuidanceView<'msg> {
  fn into_view<'shorter>(self) -> GuidanceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Guidance> for GuidanceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Guidance {
    let mut dst = Guidance::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Guidance> for GuidanceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Guidance {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Guidance {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GuidanceView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GuidanceMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GuidanceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Guidance>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GuidanceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GuidanceMut<'msg> {
  type Message = Guidance;
}

impl ::std::fmt::Debug for GuidanceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Guidance>> for GuidanceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Guidance>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GuidanceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Guidance> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Guidance {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // angle: optional uint32
  pub fn angle(&self) -> u32 {
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
  pub fn set_angle(&mut self, val: u32) {
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

  // submerge: optional bool
  pub fn submerge(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_submerge(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // velocity: optional int32
  pub fn velocity(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_velocity(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

}

// SAFETY:
// - `GuidanceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GuidanceMut<'_> {}

// SAFETY:
// - `GuidanceMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GuidanceMut<'_> {}

impl<'msg> ::protobuf::AsView for GuidanceMut<'msg> {
  type Proxied = Guidance;
  fn as_view(&self) -> ::protobuf::View<'_, Guidance> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GuidanceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Guidance>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GuidanceMut<'msg> {
  type MutProxied = Guidance;
  fn as_mut(&mut self) -> GuidanceMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GuidanceMut<'msg> {
  fn into_mut<'shorter>(self) -> GuidanceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Guidance {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Guidance> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GuidanceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GuidanceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // angle: optional uint32
  pub fn angle(&self) -> u32 {
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
  pub fn set_angle(&mut self, val: u32) {
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

  // submerge: optional bool
  pub fn submerge(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_submerge(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // velocity: optional int32
  pub fn velocity(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_velocity(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

}  // impl Guidance

impl ::std::ops::Drop for Guidance {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Guidance {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Guidance {
  type Proxied = Self;
  fn as_view(&self) -> GuidanceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Guidance {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GuidanceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Guidance {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__mk48__Guidance_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P/P(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__mk48__Guidance_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__mk48__Guidance_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Guidance {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Guidance {
  type Msg = Guidance;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Guidance> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Guidance {
  type Msg = Guidance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Guidance> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GuidanceMut<'_> {
  type Msg = Guidance;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Guidance> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GuidanceMut<'_> {
  type Msg = Guidance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Guidance> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GuidanceView<'_> {
  type Msg = Guidance;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Guidance> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GuidanceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__mk48__Contact_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Contact {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Contact>
}

impl ::protobuf::Message for Contact {
  type MessageView<'msg> = ContactView<'msg>;
  type MessageMut<'msg> = ContactMut<'msg>;
}

impl ::std::default::Default for Contact {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Contact {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Contact` is `Sync` because it does not implement interior mutability.
//    Neither does `ContactMut`.
unsafe impl ::std::marker::Sync for Contact {}

// SAFETY:
// - `Contact` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Contact {}

impl ::protobuf::Proxied for Contact {
  type View<'msg> = ContactView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Contact {}

impl ::protobuf::MutProxied for Contact {
  type Mut<'msg> = ContactMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ContactView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Contact>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ContactView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ContactView<'msg> {
  type Message = Contact;
}

impl ::std::fmt::Debug for ContactView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ContactView<'_> {
  fn default() -> ContactView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Contact>> for ContactView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Contact>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ContactView<'msg> {

  pub fn to_owned(&self) -> Contact {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // damage: optional uint32
  pub fn damage(self) -> u32 {
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

  // entity_id: optional uint32
  pub fn entity_id(self) -> u32 {
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

  // entity_type: optional enum prost.mk48.EntityType
  pub fn has_entity_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn entity_type_opt(self) -> ::std::option::Option<super::EntityType> {
    self.has_entity_type().then(|| self.entity_type())
  }
  pub fn entity_type(self) -> super::EntityType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::EntityType::ArleighBurke).into()
      ).try_into().unwrap()
    }
  }

  // guidance: optional message prost.mk48.Guidance
  pub fn has_guidance(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn guidance_opt(self) -> ::std::option::Option<super::GuidanceView<'msg>> {
    self.has_guidance().then(|| self.guidance())
  }
  pub fn guidance(self) -> super::GuidanceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GuidanceView::default())
  }

  // player_id: optional uint32
  pub fn has_player_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn player_id_opt(self) -> ::std::option::Option<u32> {
    self.has_player_id().then(|| self.player_id())
  }
  pub fn player_id(self) -> u32 {
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

  // reloads: repeated bool
  pub fn reloads(self) -> ::protobuf::RepeatedView<'msg, bool> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<bool>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // transform: optional message prost.mk48.Transform
  pub fn has_transform(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn transform_opt(self) -> ::std::option::Option<super::TransformView<'msg>> {
    self.has_transform().then(|| self.transform())
  }
  pub fn transform(self) -> super::TransformView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TransformView::default())
  }

  // turret_angles: repeated uint32
  pub fn turret_angles(self) -> ::protobuf::RepeatedView<'msg, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ContactView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ContactView<'_> {}

// SAFETY:
// - `ContactView` is `Send` because while its alive a `ContactMut` cannot.
// - `ContactView` does not use thread-local data.
unsafe impl ::std::marker::Send for ContactView<'_> {}

impl<'msg> ::protobuf::AsView for ContactView<'msg> {
  type Proxied = Contact;
  fn as_view(&self) -> ::protobuf::View<'msg, Contact> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ContactView<'msg> {
  fn into_view<'shorter>(self) -> ContactView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Contact> for ContactView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Contact {
    let mut dst = Contact::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Contact> for ContactMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Contact {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Contact {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ContactView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ContactMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ContactMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Contact>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ContactMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ContactMut<'msg> {
  type Message = Contact;
}

impl ::std::fmt::Debug for ContactMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Contact>> for ContactMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Contact>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ContactMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Contact> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Contact {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // damage: optional uint32
  pub fn damage(&self) -> u32 {
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
  pub fn set_damage(&mut self, val: u32) {
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

  // entity_id: optional uint32
  pub fn entity_id(&self) -> u32 {
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
  pub fn set_entity_id(&mut self, val: u32) {
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

  // entity_type: optional enum prost.mk48.EntityType
  pub fn has_entity_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_entity_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn entity_type_opt(&self) -> ::std::option::Option<super::EntityType> {
    self.has_entity_type().then(|| self.entity_type())
  }
  pub fn entity_type(&self) -> super::EntityType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::EntityType::ArleighBurke).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_entity_type(&mut self, val: super::EntityType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // guidance: optional message prost.mk48.Guidance
  pub fn has_guidance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_guidance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn guidance_opt(&self) -> ::std::option::Option<super::GuidanceView<'_>> {
    self.has_guidance().then(|| self.guidance())
  }
  pub fn guidance(&self) -> super::GuidanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GuidanceView::default())
  }
  pub fn guidance_mut(&mut self) -> super::GuidanceMut<'_> {
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
  pub fn set_guidance(&mut self,
    val: impl ::protobuf::IntoProxied<super::Guidance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // player_id: optional uint32
  pub fn has_player_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_player_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn player_id_opt(&self) -> ::std::option::Option<u32> {
    self.has_player_id().then(|| self.player_id())
  }
  pub fn player_id(&self) -> u32 {
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
  pub fn set_player_id(&mut self, val: u32) {
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

  // reloads: repeated bool
  pub fn reloads(&self) -> ::protobuf::RepeatedView<'_, bool> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<bool>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn reloads_mut(&mut self) -> ::protobuf::RepeatedMut<'_, bool> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_reloads(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<bool>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // transform: optional message prost.mk48.Transform
  pub fn has_transform(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_transform(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn transform_opt(&self) -> ::std::option::Option<super::TransformView<'_>> {
    self.has_transform().then(|| self.transform())
  }
  pub fn transform(&self) -> super::TransformView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TransformView::default())
  }
  pub fn transform_mut(&mut self) -> super::TransformMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_transform(&mut self,
    val: impl ::protobuf::IntoProxied<super::Transform>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // turret_angles: repeated uint32
  pub fn turret_angles(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn turret_angles_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_turret_angles(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

}

// SAFETY:
// - `ContactMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ContactMut<'_> {}

// SAFETY:
// - `ContactMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ContactMut<'_> {}

impl<'msg> ::protobuf::AsView for ContactMut<'msg> {
  type Proxied = Contact;
  fn as_view(&self) -> ::protobuf::View<'_, Contact> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ContactMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Contact>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ContactMut<'msg> {
  type MutProxied = Contact;
  fn as_mut(&mut self) -> ContactMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ContactMut<'msg> {
  fn into_mut<'shorter>(self) -> ContactMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Contact {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Contact> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ContactView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ContactMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // damage: optional uint32
  pub fn damage(&self) -> u32 {
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
  pub fn set_damage(&mut self, val: u32) {
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

  // entity_id: optional uint32
  pub fn entity_id(&self) -> u32 {
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
  pub fn set_entity_id(&mut self, val: u32) {
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

  // entity_type: optional enum prost.mk48.EntityType
  pub fn has_entity_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_entity_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn entity_type_opt(&self) -> ::std::option::Option<super::EntityType> {
    self.has_entity_type().then(|| self.entity_type())
  }
  pub fn entity_type(&self) -> super::EntityType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::EntityType::ArleighBurke).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_entity_type(&mut self, val: super::EntityType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // guidance: optional message prost.mk48.Guidance
  pub fn has_guidance(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_guidance(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn guidance_opt(&self) -> ::std::option::Option<super::GuidanceView<'_>> {
    self.has_guidance().then(|| self.guidance())
  }
  pub fn guidance(&self) -> super::GuidanceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::GuidanceView::default())
  }
  pub fn guidance_mut(&mut self) -> super::GuidanceMut<'_> {
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
  pub fn set_guidance(&mut self,
    val: impl ::protobuf::IntoProxied<super::Guidance>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // player_id: optional uint32
  pub fn has_player_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_player_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn player_id_opt(&self) -> ::std::option::Option<u32> {
    self.has_player_id().then(|| self.player_id())
  }
  pub fn player_id(&self) -> u32 {
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
  pub fn set_player_id(&mut self, val: u32) {
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

  // reloads: repeated bool
  pub fn reloads(&self) -> ::protobuf::RepeatedView<'_, bool> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<bool>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn reloads_mut(&mut self) -> ::protobuf::RepeatedMut<'_, bool> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
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
  pub fn set_reloads(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<bool>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // transform: optional message prost.mk48.Transform
  pub fn has_transform(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_transform(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn transform_opt(&self) -> ::std::option::Option<super::TransformView<'_>> {
    self.has_transform().then(|| self.transform())
  }
  pub fn transform(&self) -> super::TransformView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TransformView::default())
  }
  pub fn transform_mut(&mut self) -> super::TransformMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_transform(&mut self,
    val: impl ::protobuf::IntoProxied<super::Transform>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // turret_angles: repeated uint32
  pub fn turret_angles(&self) -> ::protobuf::RepeatedView<'_, u32> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        7
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<u32>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn turret_angles_mut(&mut self) -> ::protobuf::RepeatedMut<'_, u32> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        7,
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
  pub fn set_turret_angles(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<u32>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        src);
    }
  }

}  // impl Contact

impl ::std::ops::Drop for Contact {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Contact {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Contact {
  type Proxied = Self;
  fn as_view(&self) -> ContactView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Contact {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ContactMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Contact {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__mk48__Contact_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$N)P)P.3)C3=");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__mk48__Contact_msg_init.0, &[<super::Guidance as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Transform as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__mk48__Contact_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Contact {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Contact {
  type Msg = Contact;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Contact> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Contact {
  type Msg = Contact;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Contact> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ContactMut<'_> {
  type Msg = Contact;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Contact> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ContactMut<'_> {
  type Msg = Contact;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Contact> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ContactView<'_> {
  type Msg = Contact;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Contact> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ContactMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__mk48__ChunkId_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ChunkId {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ChunkId>
}

impl ::protobuf::Message for ChunkId {
  type MessageView<'msg> = ChunkIdView<'msg>;
  type MessageMut<'msg> = ChunkIdMut<'msg>;
}

impl ::std::default::Default for ChunkId {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ChunkId {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ChunkId` is `Sync` because it does not implement interior mutability.
//    Neither does `ChunkIdMut`.
unsafe impl ::std::marker::Sync for ChunkId {}

// SAFETY:
// - `ChunkId` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ChunkId {}

impl ::protobuf::Proxied for ChunkId {
  type View<'msg> = ChunkIdView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ChunkId {}

impl ::protobuf::MutProxied for ChunkId {
  type Mut<'msg> = ChunkIdMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ChunkIdView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ChunkId>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ChunkIdView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ChunkIdView<'msg> {
  type Message = ChunkId;
}

impl ::std::fmt::Debug for ChunkIdView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ChunkIdView<'_> {
  fn default() -> ChunkIdView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ChunkId>> for ChunkIdView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ChunkId>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ChunkIdView<'msg> {

  pub fn to_owned(&self) -> ChunkId {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // x: optional int32
  pub fn x(self) -> i32 {
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

  // y: optional int32
  pub fn y(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ChunkIdView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ChunkIdView<'_> {}

// SAFETY:
// - `ChunkIdView` is `Send` because while its alive a `ChunkIdMut` cannot.
// - `ChunkIdView` does not use thread-local data.
unsafe impl ::std::marker::Send for ChunkIdView<'_> {}

impl<'msg> ::protobuf::AsView for ChunkIdView<'msg> {
  type Proxied = ChunkId;
  fn as_view(&self) -> ::protobuf::View<'msg, ChunkId> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ChunkIdView<'msg> {
  fn into_view<'shorter>(self) -> ChunkIdView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ChunkId> for ChunkIdView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ChunkId {
    let mut dst = ChunkId::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ChunkId> for ChunkIdMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ChunkId {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ChunkId {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ChunkIdView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ChunkIdMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ChunkIdMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ChunkId>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ChunkIdMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ChunkIdMut<'msg> {
  type Message = ChunkId;
}

impl ::std::fmt::Debug for ChunkIdMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ChunkId>> for ChunkIdMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ChunkId>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ChunkIdMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ChunkId> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ChunkId {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // x: optional int32
  pub fn x(&self) -> i32 {
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
  pub fn set_x(&mut self, val: i32) {
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

  // y: optional int32
  pub fn y(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_y(&mut self, val: i32) {
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

}

// SAFETY:
// - `ChunkIdMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ChunkIdMut<'_> {}

// SAFETY:
// - `ChunkIdMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ChunkIdMut<'_> {}

impl<'msg> ::protobuf::AsView for ChunkIdMut<'msg> {
  type Proxied = ChunkId;
  fn as_view(&self) -> ::protobuf::View<'_, ChunkId> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ChunkIdMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ChunkId>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ChunkIdMut<'msg> {
  type MutProxied = ChunkId;
  fn as_mut(&mut self) -> ChunkIdMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ChunkIdMut<'msg> {
  fn into_mut<'shorter>(self) -> ChunkIdMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ChunkId {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ChunkId> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ChunkIdView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ChunkIdMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // x: optional int32
  pub fn x(&self) -> i32 {
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
  pub fn set_x(&mut self, val: i32) {
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

  // y: optional int32
  pub fn y(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_y(&mut self, val: i32) {
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

}  // impl ChunkId

impl ::std::ops::Drop for ChunkId {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ChunkId {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ChunkId {
  type Proxied = Self;
  fn as_view(&self) -> ChunkIdView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ChunkId {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ChunkIdMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ChunkId {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__mk48__ChunkId_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$(P(P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__mk48__ChunkId_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__mk48__ChunkId_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ChunkId {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ChunkId {
  type Msg = ChunkId;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChunkId> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ChunkId {
  type Msg = ChunkId;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChunkId> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ChunkIdMut<'_> {
  type Msg = ChunkId;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChunkId> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ChunkIdMut<'_> {
  type Msg = ChunkId;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChunkId> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ChunkIdView<'_> {
  type Msg = ChunkId;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ChunkId> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ChunkIdMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__mk48__TerrainUpdate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TerrainUpdate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TerrainUpdate>
}

impl ::protobuf::Message for TerrainUpdate {
  type MessageView<'msg> = TerrainUpdateView<'msg>;
  type MessageMut<'msg> = TerrainUpdateMut<'msg>;
}

impl ::std::default::Default for TerrainUpdate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TerrainUpdate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TerrainUpdate` is `Sync` because it does not implement interior mutability.
//    Neither does `TerrainUpdateMut`.
unsafe impl ::std::marker::Sync for TerrainUpdate {}

// SAFETY:
// - `TerrainUpdate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TerrainUpdate {}

impl ::protobuf::Proxied for TerrainUpdate {
  type View<'msg> = TerrainUpdateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TerrainUpdate {}

impl ::protobuf::MutProxied for TerrainUpdate {
  type Mut<'msg> = TerrainUpdateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TerrainUpdateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TerrainUpdate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TerrainUpdateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TerrainUpdateView<'msg> {
  type Message = TerrainUpdate;
}

impl ::std::fmt::Debug for TerrainUpdateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TerrainUpdateView<'_> {
  fn default() -> TerrainUpdateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TerrainUpdate>> for TerrainUpdateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TerrainUpdate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TerrainUpdateView<'msg> {

  pub fn to_owned(&self) -> TerrainUpdate {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // chunk_id: optional message prost.mk48.ChunkId
  pub fn has_chunk_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn chunk_id_opt(self) -> ::std::option::Option<super::ChunkIdView<'msg>> {
    self.has_chunk_id().then(|| self.chunk_id())
  }
  pub fn chunk_id(self) -> super::ChunkIdView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ChunkIdView::default())
  }

  // data: optional bytes
  pub fn data(self) -> ::protobuf::View<'msg, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }

}

// SAFETY:
// - `TerrainUpdateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TerrainUpdateView<'_> {}

// SAFETY:
// - `TerrainUpdateView` is `Send` because while its alive a `TerrainUpdateMut` cannot.
// - `TerrainUpdateView` does not use thread-local data.
unsafe impl ::std::marker::Send for TerrainUpdateView<'_> {}

impl<'msg> ::protobuf::AsView for TerrainUpdateView<'msg> {
  type Proxied = TerrainUpdate;
  fn as_view(&self) -> ::protobuf::View<'msg, TerrainUpdate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TerrainUpdateView<'msg> {
  fn into_view<'shorter>(self) -> TerrainUpdateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TerrainUpdate> for TerrainUpdateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TerrainUpdate {
    let mut dst = TerrainUpdate::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TerrainUpdate> for TerrainUpdateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TerrainUpdate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TerrainUpdate {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TerrainUpdateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TerrainUpdateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TerrainUpdateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TerrainUpdate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TerrainUpdateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TerrainUpdateMut<'msg> {
  type Message = TerrainUpdate;
}

impl ::std::fmt::Debug for TerrainUpdateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TerrainUpdate>> for TerrainUpdateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TerrainUpdate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TerrainUpdateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TerrainUpdate> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TerrainUpdate {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // chunk_id: optional message prost.mk48.ChunkId
  pub fn has_chunk_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_chunk_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn chunk_id_opt(&self) -> ::std::option::Option<super::ChunkIdView<'_>> {
    self.has_chunk_id().then(|| self.chunk_id())
  }
  pub fn chunk_id(&self) -> super::ChunkIdView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ChunkIdView::default())
  }
  pub fn chunk_id_mut(&mut self) -> super::ChunkIdMut<'_> {
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
  pub fn set_chunk_id(&mut self,
    val: impl ::protobuf::IntoProxied<super::ChunkId>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // data: optional bytes
  pub fn data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `TerrainUpdateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TerrainUpdateMut<'_> {}

// SAFETY:
// - `TerrainUpdateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TerrainUpdateMut<'_> {}

impl<'msg> ::protobuf::AsView for TerrainUpdateMut<'msg> {
  type Proxied = TerrainUpdate;
  fn as_view(&self) -> ::protobuf::View<'_, TerrainUpdate> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TerrainUpdateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TerrainUpdate>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TerrainUpdateMut<'msg> {
  type MutProxied = TerrainUpdate;
  fn as_mut(&mut self) -> TerrainUpdateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TerrainUpdateMut<'msg> {
  fn into_mut<'shorter>(self) -> TerrainUpdateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TerrainUpdate {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TerrainUpdate> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TerrainUpdateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TerrainUpdateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // chunk_id: optional message prost.mk48.ChunkId
  pub fn has_chunk_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_chunk_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn chunk_id_opt(&self) -> ::std::option::Option<super::ChunkIdView<'_>> {
    self.has_chunk_id().then(|| self.chunk_id())
  }
  pub fn chunk_id(&self) -> super::ChunkIdView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ChunkIdView::default())
  }
  pub fn chunk_id_mut(&mut self) -> super::ChunkIdMut<'_> {
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
  pub fn set_chunk_id(&mut self,
    val: impl ::protobuf::IntoProxied<super::ChunkId>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // data: optional bytes
  pub fn data(&self) -> ::protobuf::View<'_, ::protobuf::ProtoBytes> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    unsafe { str_view.as_ref() }
  }
  pub fn set_data(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoBytes>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_bytes_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl TerrainUpdate

impl ::std::ops::Drop for TerrainUpdate {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TerrainUpdate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TerrainUpdate {
  type Proxied = Self;
  fn as_view(&self) -> TerrainUpdateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TerrainUpdate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TerrainUpdateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TerrainUpdate {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__mk48__TerrainUpdate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$30P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__mk48__TerrainUpdate_msg_init.0, &[<super::ChunkId as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__mk48__TerrainUpdate_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TerrainUpdate {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TerrainUpdate {
  type Msg = TerrainUpdate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TerrainUpdate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TerrainUpdate {
  type Msg = TerrainUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TerrainUpdate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TerrainUpdateMut<'_> {
  type Msg = TerrainUpdate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TerrainUpdate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TerrainUpdateMut<'_> {
  type Msg = TerrainUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TerrainUpdate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TerrainUpdateView<'_> {
  type Msg = TerrainUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TerrainUpdate> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TerrainUpdateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__mk48__Update_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Update {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Update>
}

impl ::protobuf::Message for Update {
  type MessageView<'msg> = UpdateView<'msg>;
  type MessageMut<'msg> = UpdateMut<'msg>;
}

impl ::std::default::Default for Update {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Update {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Update` is `Sync` because it does not implement interior mutability.
//    Neither does `UpdateMut`.
unsafe impl ::std::marker::Sync for Update {}

// SAFETY:
// - `Update` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Update {}

impl ::protobuf::Proxied for Update {
  type View<'msg> = UpdateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Update {}

impl ::protobuf::MutProxied for Update {
  type Mut<'msg> = UpdateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpdateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Update>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpdateView<'msg> {
  type Message = Update;
}

impl ::std::fmt::Debug for UpdateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpdateView<'_> {
  fn default() -> UpdateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Update>> for UpdateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Update>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdateView<'msg> {

  pub fn to_owned(&self) -> Update {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // contacts: repeated message prost.mk48.Contact
  pub fn contacts(self) -> ::protobuf::RepeatedView<'msg, super::Contact> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Contact>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // score: optional uint32
  pub fn score(self) -> u32 {
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

  // world_radius: optional float
  pub fn world_radius(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // terrain_updates: repeated message prost.mk48.TerrainUpdate
  pub fn terrain_updates(self) -> ::protobuf::RepeatedView<'msg, super::TerrainUpdate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TerrainUpdate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `UpdateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UpdateView<'_> {}

// SAFETY:
// - `UpdateView` is `Send` because while its alive a `UpdateMut` cannot.
// - `UpdateView` does not use thread-local data.
unsafe impl ::std::marker::Send for UpdateView<'_> {}

impl<'msg> ::protobuf::AsView for UpdateView<'msg> {
  type Proxied = Update;
  fn as_view(&self) -> ::protobuf::View<'msg, Update> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdateView<'msg> {
  fn into_view<'shorter>(self) -> UpdateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Update> for UpdateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Update {
    let mut dst = Update::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Update> for UpdateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Update {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Update {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpdateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpdateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpdateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Update>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpdateMut<'msg> {
  type Message = Update;
}

impl ::std::fmt::Debug for UpdateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Update>> for UpdateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Update>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Update> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Update {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // contacts: repeated message prost.mk48.Contact
  pub fn contacts(&self) -> ::protobuf::RepeatedView<'_, super::Contact> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Contact>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn contacts_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Contact> {
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
  pub fn set_contacts(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Contact>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // score: optional uint32
  pub fn score(&self) -> u32 {
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
  pub fn set_score(&mut self, val: u32) {
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

  // world_radius: optional float
  pub fn world_radius(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_world_radius(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        2, val.into()
      )
    }
  }

  // terrain_updates: repeated message prost.mk48.TerrainUpdate
  pub fn terrain_updates(&self) -> ::protobuf::RepeatedView<'_, super::TerrainUpdate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TerrainUpdate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn terrain_updates_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TerrainUpdate> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
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
  pub fn set_terrain_updates(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TerrainUpdate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `UpdateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UpdateMut<'_> {}

// SAFETY:
// - `UpdateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UpdateMut<'_> {}

impl<'msg> ::protobuf::AsView for UpdateMut<'msg> {
  type Proxied = Update;
  fn as_view(&self) -> ::protobuf::View<'_, Update> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Update>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UpdateMut<'msg> {
  type MutProxied = Update;
  fn as_mut(&mut self) -> UpdateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpdateMut<'msg> {
  fn into_mut<'shorter>(self) -> UpdateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Update {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Update> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpdateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpdateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // contacts: repeated message prost.mk48.Contact
  pub fn contacts(&self) -> ::protobuf::RepeatedView<'_, super::Contact> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Contact>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn contacts_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Contact> {
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
  pub fn set_contacts(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Contact>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // score: optional uint32
  pub fn score(&self) -> u32 {
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
  pub fn set_score(&mut self, val: u32) {
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

  // world_radius: optional float
  pub fn world_radius(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_world_radius(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        2, val.into()
      )
    }
  }

  // terrain_updates: repeated message prost.mk48.TerrainUpdate
  pub fn terrain_updates(&self) -> ::protobuf::RepeatedView<'_, super::TerrainUpdate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TerrainUpdate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn terrain_updates_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TerrainUpdate> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
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
  pub fn set_terrain_updates(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TerrainUpdate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl Update

impl ::std::ops::Drop for Update {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Update {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Update {
  type Proxied = Self;
  fn as_view(&self) -> UpdateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Update {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpdateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Update {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__mk48__Update_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G)P!PG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__mk48__Update_msg_init.0, &[<super::Contact as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TerrainUpdate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__mk48__Update_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Update {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Update {
  type Msg = Update;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Update> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Update {
  type Msg = Update;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Update> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpdateMut<'_> {
  type Msg = Update;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Update> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateMut<'_> {
  type Msg = Update;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Update> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdateView<'_> {
  type Msg = Update;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Update> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpdateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__mk48__Updates_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Updates {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Updates>
}

impl ::protobuf::Message for Updates {
  type MessageView<'msg> = UpdatesView<'msg>;
  type MessageMut<'msg> = UpdatesMut<'msg>;
}

impl ::std::default::Default for Updates {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Updates {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Updates` is `Sync` because it does not implement interior mutability.
//    Neither does `UpdatesMut`.
unsafe impl ::std::marker::Sync for Updates {}

// SAFETY:
// - `Updates` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Updates {}

impl ::protobuf::Proxied for Updates {
  type View<'msg> = UpdatesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Updates {}

impl ::protobuf::MutProxied for Updates {
  type Mut<'msg> = UpdatesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpdatesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Updates>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdatesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for UpdatesView<'msg> {
  type Message = Updates;
}

impl ::std::fmt::Debug for UpdatesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for UpdatesView<'_> {
  fn default() -> UpdatesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Updates>> for UpdatesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Updates>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdatesView<'msg> {

  pub fn to_owned(&self) -> Updates {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // updates: repeated message prost.mk48.Update
  pub fn updates(self) -> ::protobuf::RepeatedView<'msg, super::Update> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Update>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `UpdatesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for UpdatesView<'_> {}

// SAFETY:
// - `UpdatesView` is `Send` because while its alive a `UpdatesMut` cannot.
// - `UpdatesView` does not use thread-local data.
unsafe impl ::std::marker::Send for UpdatesView<'_> {}

impl<'msg> ::protobuf::AsView for UpdatesView<'msg> {
  type Proxied = Updates;
  fn as_view(&self) -> ::protobuf::View<'msg, Updates> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdatesView<'msg> {
  fn into_view<'shorter>(self) -> UpdatesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Updates> for UpdatesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Updates {
    let mut dst = Updates::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Updates> for UpdatesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Updates {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Updates {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpdatesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for UpdatesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpdatesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Updates>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for UpdatesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for UpdatesMut<'msg> {
  type Message = Updates;
}

impl ::std::fmt::Debug for UpdatesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Updates>> for UpdatesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Updates>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> UpdatesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Updates> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Updates {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // updates: repeated message prost.mk48.Update
  pub fn updates(&self) -> ::protobuf::RepeatedView<'_, super::Update> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Update>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn updates_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Update> {
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
  pub fn set_updates(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Update>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `UpdatesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for UpdatesMut<'_> {}

// SAFETY:
// - `UpdatesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for UpdatesMut<'_> {}

impl<'msg> ::protobuf::AsView for UpdatesMut<'msg> {
  type Proxied = Updates;
  fn as_view(&self) -> ::protobuf::View<'_, Updates> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for UpdatesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Updates>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for UpdatesMut<'msg> {
  type MutProxied = Updates;
  fn as_mut(&mut self) -> UpdatesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for UpdatesMut<'msg> {
  fn into_mut<'shorter>(self) -> UpdatesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Updates {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Updates> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> UpdatesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> UpdatesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // updates: repeated message prost.mk48.Update
  pub fn updates(&self) -> ::protobuf::RepeatedView<'_, super::Update> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Update>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn updates_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Update> {
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
  pub fn set_updates(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Update>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Updates

impl ::std::ops::Drop for Updates {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Updates {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Updates {
  type Proxied = Self;
  fn as_view(&self) -> UpdatesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Updates {
  type MutProxied = Self;
  fn as_mut(&mut self) -> UpdatesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Updates {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__mk48__Updates_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__mk48__Updates_msg_init.0, &[<super::Update as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__mk48__Updates_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Updates {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Updates {
  type Msg = Updates;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Updates> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Updates {
  type Msg = Updates;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Updates> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for UpdatesMut<'_> {
  type Msg = Updates;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Updates> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdatesMut<'_> {
  type Msg = Updates;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Updates> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for UpdatesView<'_> {
  type Msg = Updates;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Updates> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for UpdatesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityType(i32);

#[allow(non_upper_case_globals)]
impl EntityType {
  pub const ArleighBurke: EntityType = EntityType(0);
  pub const Bismarck: EntityType = EntityType(1);
  pub const Clemenceau: EntityType = EntityType(2);
  pub const Fletcher: EntityType = EntityType(3);
  pub const G5: EntityType = EntityType(4);
  pub const Iowa: EntityType = EntityType(5);
  pub const Kolkata: EntityType = EntityType(6);
  pub const Osa: EntityType = EntityType(7);
  pub const Yasen: EntityType = EntityType(8);
  pub const Zubr: EntityType = EntityType(9);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "ArleighBurke",
      1 => "Bismarck",
      2 => "Clemenceau",
      3 => "Fletcher",
      4 => "G5",
      5 => "Iowa",
      6 => "Kolkata",
      7 => "Osa",
      8 => "Yasen",
      9 => "Zubr",
      _ => return None
    })
  }
}

impl ::std::convert::From<EntityType> for i32 {
  fn from(val: EntityType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for EntityType {
  fn from(val: i32) -> EntityType {
    Self(val)
  }
}

impl ::std::default::Default for EntityType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for EntityType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "EntityType::{}", constant_name)
    } else {
      write!(f, "EntityType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for EntityType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for EntityType {}

impl ::protobuf::Proxied for EntityType {
  type View<'a> = EntityType;
}

impl ::protobuf::AsView for EntityType {
  type Proxied = EntityType;

  fn as_view(&self) -> EntityType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EntityType {
  fn into_view<'shorter>(self) -> EntityType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for EntityType {
  const NAME: &'static str = "EntityType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6|7|8|9)
  }
}

impl ::protobuf::__internal::EntityType for EntityType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


