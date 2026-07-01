const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__mesh__Vector3_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Vector3 {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Vector3>
}

impl ::protobuf::Message for Vector3 {
  type MessageView<'msg> = Vector3View<'msg>;
  type MessageMut<'msg> = Vector3Mut<'msg>;
}

impl ::std::default::Default for Vector3 {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Vector3 {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Vector3` is `Sync` because it does not implement interior mutability.
//    Neither does `Vector3Mut`.
unsafe impl ::std::marker::Sync for Vector3 {}

// SAFETY:
// - `Vector3` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Vector3 {}

impl ::protobuf::Proxied for Vector3 {
  type View<'msg> = Vector3View<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Vector3 {}

impl ::protobuf::MutProxied for Vector3 {
  type Mut<'msg> = Vector3Mut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Vector3View<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vector3>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Vector3View<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for Vector3View<'msg> {
  type Message = Vector3;
}

impl ::std::fmt::Debug for Vector3View<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for Vector3View<'_> {
  fn default() -> Vector3View<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Vector3>> for Vector3View<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Vector3>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Vector3View<'msg> {

  pub fn to_owned(&self) -> Vector3 {
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

  // z: optional float
  pub fn z(self) -> f32 {
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

}

// SAFETY:
// - `Vector3View` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for Vector3View<'_> {}

// SAFETY:
// - `Vector3View` is `Send` because while its alive a `Vector3Mut` cannot.
// - `Vector3View` does not use thread-local data.
unsafe impl ::std::marker::Send for Vector3View<'_> {}

impl<'msg> ::protobuf::AsView for Vector3View<'msg> {
  type Proxied = Vector3;
  fn as_view(&self) -> ::protobuf::View<'msg, Vector3> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Vector3View<'msg> {
  fn into_view<'shorter>(self) -> Vector3View<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Vector3> for Vector3View<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vector3 {
    let mut dst = Vector3::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Vector3> for Vector3Mut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Vector3 {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Vector3 {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Vector3View<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for Vector3Mut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct Vector3Mut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector3>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for Vector3Mut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for Vector3Mut<'msg> {
  type Message = Vector3;
}

impl ::std::fmt::Debug for Vector3Mut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Vector3>> for Vector3Mut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector3>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> Vector3Mut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Vector3> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Vector3 {
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

  // z: optional float
  pub fn z(&self) -> f32 {
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
  pub fn set_z(&mut self, val: f32) {
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

}

// SAFETY:
// - `Vector3Mut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for Vector3Mut<'_> {}

// SAFETY:
// - `Vector3Mut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for Vector3Mut<'_> {}

impl<'msg> ::protobuf::AsView for Vector3Mut<'msg> {
  type Proxied = Vector3;
  fn as_view(&self) -> ::protobuf::View<'_, Vector3> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Vector3Mut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Vector3>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for Vector3Mut<'msg> {
  type MutProxied = Vector3;
  fn as_mut(&mut self) -> Vector3Mut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for Vector3Mut<'msg> {
  fn into_mut<'shorter>(self) -> Vector3Mut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Vector3 {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Vector3> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> Vector3View<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> Vector3Mut<'_> {
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

  // z: optional float
  pub fn z(&self) -> f32 {
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
  pub fn set_z(&mut self, val: f32) {
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

}  // impl Vector3

impl ::std::ops::Drop for Vector3 {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Vector3 {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Vector3 {
  type Proxied = Self;
  fn as_view(&self) -> Vector3View<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Vector3 {
  type MutProxied = Self;
  fn as_mut(&mut self) -> Vector3Mut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Vector3 {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__mesh__Vector3_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$!P!P!P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__mesh__Vector3_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__mesh__Vector3_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Vector3 {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Vector3 {
  type Msg = Vector3;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector3 {
  type Msg = Vector3;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Vector3Mut<'_> {
  type Msg = Vector3;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector3Mut<'_> {
  type Msg = Vector3;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Vector3View<'_> {
  type Msg = Vector3;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Vector3> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Vector3Mut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__mesh__Triangle_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Triangle {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Triangle>
}

impl ::protobuf::Message for Triangle {
  type MessageView<'msg> = TriangleView<'msg>;
  type MessageMut<'msg> = TriangleMut<'msg>;
}

impl ::std::default::Default for Triangle {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Triangle {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Triangle` is `Sync` because it does not implement interior mutability.
//    Neither does `TriangleMut`.
unsafe impl ::std::marker::Sync for Triangle {}

// SAFETY:
// - `Triangle` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Triangle {}

impl ::protobuf::Proxied for Triangle {
  type View<'msg> = TriangleView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Triangle {}

impl ::protobuf::MutProxied for Triangle {
  type Mut<'msg> = TriangleMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TriangleView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Triangle>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TriangleView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TriangleView<'msg> {
  type Message = Triangle;
}

impl ::std::fmt::Debug for TriangleView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TriangleView<'_> {
  fn default() -> TriangleView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Triangle>> for TriangleView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Triangle>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TriangleView<'msg> {

  pub fn to_owned(&self) -> Triangle {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // v0: optional message prost.mesh.Vector3
  pub fn has_v0(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn v0_opt(self) -> ::std::option::Option<super::Vector3View<'msg>> {
    self.has_v0().then(|| self.v0())
  }
  pub fn v0(self) -> super::Vector3View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }

  // v1: optional message prost.mesh.Vector3
  pub fn has_v1(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn v1_opt(self) -> ::std::option::Option<super::Vector3View<'msg>> {
    self.has_v1().then(|| self.v1())
  }
  pub fn v1(self) -> super::Vector3View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }

  // v2: optional message prost.mesh.Vector3
  pub fn has_v2(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn v2_opt(self) -> ::std::option::Option<super::Vector3View<'msg>> {
    self.has_v2().then(|| self.v2())
  }
  pub fn v2(self) -> super::Vector3View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }

  // normal: optional message prost.mesh.Vector3
  pub fn has_normal(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn normal_opt(self) -> ::std::option::Option<super::Vector3View<'msg>> {
    self.has_normal().then(|| self.normal())
  }
  pub fn normal(self) -> super::Vector3View<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }

}

// SAFETY:
// - `TriangleView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TriangleView<'_> {}

// SAFETY:
// - `TriangleView` is `Send` because while its alive a `TriangleMut` cannot.
// - `TriangleView` does not use thread-local data.
unsafe impl ::std::marker::Send for TriangleView<'_> {}

impl<'msg> ::protobuf::AsView for TriangleView<'msg> {
  type Proxied = Triangle;
  fn as_view(&self) -> ::protobuf::View<'msg, Triangle> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TriangleView<'msg> {
  fn into_view<'shorter>(self) -> TriangleView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Triangle> for TriangleView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Triangle {
    let mut dst = Triangle::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Triangle> for TriangleMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Triangle {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Triangle {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TriangleView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TriangleMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TriangleMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Triangle>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TriangleMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TriangleMut<'msg> {
  type Message = Triangle;
}

impl ::std::fmt::Debug for TriangleMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Triangle>> for TriangleMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Triangle>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TriangleMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Triangle> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Triangle {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // v0: optional message prost.mesh.Vector3
  pub fn has_v0(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_v0(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn v0_opt(&self) -> ::std::option::Option<super::Vector3View<'_>> {
    self.has_v0().then(|| self.v0())
  }
  pub fn v0(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }
  pub fn v0_mut(&mut self) -> super::Vector3Mut<'_> {
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
  pub fn set_v0(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // v1: optional message prost.mesh.Vector3
  pub fn has_v1(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_v1(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn v1_opt(&self) -> ::std::option::Option<super::Vector3View<'_>> {
    self.has_v1().then(|| self.v1())
  }
  pub fn v1(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }
  pub fn v1_mut(&mut self) -> super::Vector3Mut<'_> {
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
  pub fn set_v1(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // v2: optional message prost.mesh.Vector3
  pub fn has_v2(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_v2(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn v2_opt(&self) -> ::std::option::Option<super::Vector3View<'_>> {
    self.has_v2().then(|| self.v2())
  }
  pub fn v2(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }
  pub fn v2_mut(&mut self) -> super::Vector3Mut<'_> {
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
  pub fn set_v2(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // normal: optional message prost.mesh.Vector3
  pub fn has_normal(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_normal(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn normal_opt(&self) -> ::std::option::Option<super::Vector3View<'_>> {
    self.has_normal().then(|| self.normal())
  }
  pub fn normal(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }
  pub fn normal_mut(&mut self) -> super::Vector3Mut<'_> {
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
  pub fn set_normal(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}

// SAFETY:
// - `TriangleMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TriangleMut<'_> {}

// SAFETY:
// - `TriangleMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TriangleMut<'_> {}

impl<'msg> ::protobuf::AsView for TriangleMut<'msg> {
  type Proxied = Triangle;
  fn as_view(&self) -> ::protobuf::View<'_, Triangle> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TriangleMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Triangle>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TriangleMut<'msg> {
  type MutProxied = Triangle;
  fn as_mut(&mut self) -> TriangleMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TriangleMut<'msg> {
  fn into_mut<'shorter>(self) -> TriangleMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Triangle {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Triangle> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TriangleView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TriangleMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // v0: optional message prost.mesh.Vector3
  pub fn has_v0(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_v0(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn v0_opt(&self) -> ::std::option::Option<super::Vector3View<'_>> {
    self.has_v0().then(|| self.v0())
  }
  pub fn v0(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }
  pub fn v0_mut(&mut self) -> super::Vector3Mut<'_> {
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
  pub fn set_v0(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // v1: optional message prost.mesh.Vector3
  pub fn has_v1(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_v1(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn v1_opt(&self) -> ::std::option::Option<super::Vector3View<'_>> {
    self.has_v1().then(|| self.v1())
  }
  pub fn v1(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }
  pub fn v1_mut(&mut self) -> super::Vector3Mut<'_> {
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
  pub fn set_v1(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // v2: optional message prost.mesh.Vector3
  pub fn has_v2(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_v2(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn v2_opt(&self) -> ::std::option::Option<super::Vector3View<'_>> {
    self.has_v2().then(|| self.v2())
  }
  pub fn v2(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }
  pub fn v2_mut(&mut self) -> super::Vector3Mut<'_> {
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
  pub fn set_v2(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // normal: optional message prost.mesh.Vector3
  pub fn has_normal(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_normal(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn normal_opt(&self) -> ::std::option::Option<super::Vector3View<'_>> {
    self.has_normal().then(|| self.normal())
  }
  pub fn normal(&self) -> super::Vector3View<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::Vector3View::default())
  }
  pub fn normal_mut(&mut self) -> super::Vector3Mut<'_> {
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
  pub fn set_normal(&mut self,
    val: impl ::protobuf::IntoProxied<super::Vector3>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}  // impl Triangle

impl ::std::ops::Drop for Triangle {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Triangle {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Triangle {
  type Proxied = Self;
  fn as_view(&self) -> TriangleView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Triangle {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TriangleMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Triangle {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__mesh__Triangle_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__mesh__Triangle_msg_init.0, &[<super::Vector3 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Vector3 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Vector3 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Vector3 as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__mesh__Triangle_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Triangle {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Triangle {
  type Msg = Triangle;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Triangle> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Triangle {
  type Msg = Triangle;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Triangle> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TriangleMut<'_> {
  type Msg = Triangle;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Triangle> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TriangleMut<'_> {
  type Msg = Triangle;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Triangle> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TriangleView<'_> {
  type Msg = Triangle;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Triangle> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TriangleMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut prost__mesh__Mesh_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Mesh {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Mesh>
}

impl ::protobuf::Message for Mesh {
  type MessageView<'msg> = MeshView<'msg>;
  type MessageMut<'msg> = MeshMut<'msg>;
}

impl ::std::default::Default for Mesh {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Mesh {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Mesh` is `Sync` because it does not implement interior mutability.
//    Neither does `MeshMut`.
unsafe impl ::std::marker::Sync for Mesh {}

// SAFETY:
// - `Mesh` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Mesh {}

impl ::protobuf::Proxied for Mesh {
  type View<'msg> = MeshView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Mesh {}

impl ::protobuf::MutProxied for Mesh {
  type Mut<'msg> = MeshMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MeshView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Mesh>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MeshView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MeshView<'msg> {
  type Message = Mesh;
}

impl ::std::fmt::Debug for MeshView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MeshView<'_> {
  fn default() -> MeshView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Mesh>> for MeshView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Mesh>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MeshView<'msg> {

  pub fn to_owned(&self) -> Mesh {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // triangles: repeated message prost.mesh.Triangle
  pub fn triangles(self) -> ::protobuf::RepeatedView<'msg, super::Triangle> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Triangle>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `MeshView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MeshView<'_> {}

// SAFETY:
// - `MeshView` is `Send` because while its alive a `MeshMut` cannot.
// - `MeshView` does not use thread-local data.
unsafe impl ::std::marker::Send for MeshView<'_> {}

impl<'msg> ::protobuf::AsView for MeshView<'msg> {
  type Proxied = Mesh;
  fn as_view(&self) -> ::protobuf::View<'msg, Mesh> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MeshView<'msg> {
  fn into_view<'shorter>(self) -> MeshView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Mesh> for MeshView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Mesh {
    let mut dst = Mesh::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Mesh> for MeshMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Mesh {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Mesh {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MeshView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MeshMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MeshMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Mesh>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MeshMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MeshMut<'msg> {
  type Message = Mesh;
}

impl ::std::fmt::Debug for MeshMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Mesh>> for MeshMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Mesh>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MeshMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Mesh> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Mesh {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // triangles: repeated message prost.mesh.Triangle
  pub fn triangles(&self) -> ::protobuf::RepeatedView<'_, super::Triangle> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Triangle>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn triangles_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Triangle> {
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
  pub fn set_triangles(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Triangle>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `MeshMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MeshMut<'_> {}

// SAFETY:
// - `MeshMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MeshMut<'_> {}

impl<'msg> ::protobuf::AsView for MeshMut<'msg> {
  type Proxied = Mesh;
  fn as_view(&self) -> ::protobuf::View<'_, Mesh> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MeshMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Mesh>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MeshMut<'msg> {
  type MutProxied = Mesh;
  fn as_mut(&mut self) -> MeshMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MeshMut<'msg> {
  fn into_mut<'shorter>(self) -> MeshMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Mesh {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Mesh> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MeshView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MeshMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // triangles: repeated message prost.mesh.Triangle
  pub fn triangles(&self) -> ::protobuf::RepeatedView<'_, super::Triangle> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Triangle>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn triangles_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Triangle> {
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
  pub fn set_triangles(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Triangle>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl Mesh

impl ::std::ops::Drop for Mesh {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Mesh {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Mesh {
  type Proxied = Self;
  fn as_view(&self) -> MeshView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Mesh {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MeshMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Mesh {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::prost__mesh__Mesh_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::prost__mesh__Mesh_msg_init.0, &[<super::Triangle as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::prost__mesh__Mesh_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Mesh {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Mesh {
  type Msg = Mesh;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Mesh> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Mesh {
  type Msg = Mesh;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Mesh> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MeshMut<'_> {
  type Msg = Mesh;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Mesh> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MeshMut<'_> {
  type Msg = Mesh;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Mesh> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MeshView<'_> {
  type Msg = Mesh;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Mesh> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MeshMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



