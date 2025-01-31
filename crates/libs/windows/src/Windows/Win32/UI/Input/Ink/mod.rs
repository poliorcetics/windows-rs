#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkCommitRequestHandler(::windows::core::IUnknown);
impl IInkCommitRequestHandler {
    pub unsafe fn OnCommitRequested(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnCommitRequested)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IInkCommitRequestHandler> for ::windows::core::IUnknown {
    fn from(value: IInkCommitRequestHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IInkCommitRequestHandler> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IInkCommitRequestHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkCommitRequestHandler> for ::windows::core::IUnknown {
    fn from(value: &IInkCommitRequestHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IInkCommitRequestHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkCommitRequestHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkCommitRequestHandler {}
impl ::core::fmt::Debug for IInkCommitRequestHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkCommitRequestHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkCommitRequestHandler {
    type Vtable = IInkCommitRequestHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfabea3fc_b108_45b6_a9fc_8d08fa9f85cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCommitRequestHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnCommitRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkD2DRenderer(::windows::core::IUnknown);
impl IInkD2DRenderer {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Draw<'a, P0, P1, P2>(&self, pd2d1devicecontext: P0, pinkstrokeiterable: P1, fhighcontrast: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Draw)(::windows::core::Interface::as_raw(self), pd2d1devicecontext.into().abi(), pinkstrokeiterable.into().abi(), fhighcontrast.into()).ok()
    }
}
impl ::core::convert::From<IInkD2DRenderer> for ::windows::core::IUnknown {
    fn from(value: IInkD2DRenderer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IInkD2DRenderer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IInkD2DRenderer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkD2DRenderer> for ::windows::core::IUnknown {
    fn from(value: &IInkD2DRenderer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IInkD2DRenderer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkD2DRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkD2DRenderer {}
impl ::core::fmt::Debug for IInkD2DRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkD2DRenderer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkD2DRenderer {
    type Vtable = IInkD2DRenderer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x407fb1de_f85a_4150_97cf_b7fb274fb4f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Draw: usize,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkD2DRenderer2(::windows::core::IUnknown);
impl IInkD2DRenderer2 {
    pub unsafe fn Draw<'a, P0, P1>(&self, pd2d1devicecontext: P0, pinkstrokeiterable: P1, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).Draw)(::windows::core::Interface::as_raw(self), pd2d1devicecontext.into().abi(), pinkstrokeiterable.into().abi(), highcontrastadjustment).ok()
    }
}
impl ::core::convert::From<IInkD2DRenderer2> for ::windows::core::IUnknown {
    fn from(value: IInkD2DRenderer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IInkD2DRenderer2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IInkD2DRenderer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkD2DRenderer2> for ::windows::core::IUnknown {
    fn from(value: &IInkD2DRenderer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IInkD2DRenderer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkD2DRenderer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkD2DRenderer2 {}
impl ::core::fmt::Debug for IInkD2DRenderer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkD2DRenderer2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkD2DRenderer2 {
    type Vtable = IInkD2DRenderer2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a95dcd9_4578_4b71_b20b_bf664d4bfeee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkD2DRenderer2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkDesktopHost(::windows::core::IUnknown);
impl IInkDesktopHost {
    pub unsafe fn QueueWorkItem<'a, P0>(&self, workitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IInkHostWorkItem>>,
    {
        (::windows::core::Interface::vtable(self).QueueWorkItem)(::windows::core::Interface::as_raw(self), workitem.into().abi()).ok()
    }
    pub unsafe fn CreateInkPresenter<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateInkPresenter)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn CreateAndInitializeInkPresenter<'a, P0, T>(&self, rootvisual: P0, width: f32, height: f32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateAndInitializeInkPresenter)(::windows::core::Interface::as_raw(self), rootvisual.into().abi(), width, height, &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IInkDesktopHost> for ::windows::core::IUnknown {
    fn from(value: IInkDesktopHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IInkDesktopHost> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IInkDesktopHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkDesktopHost> for ::windows::core::IUnknown {
    fn from(value: &IInkDesktopHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IInkDesktopHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkDesktopHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkDesktopHost {}
impl ::core::fmt::Debug for IInkDesktopHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkDesktopHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkDesktopHost {
    type Vtable = IInkDesktopHost_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ce7d875_a981_4140_a1ff_ad93258e8d59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDesktopHost_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub QueueWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAndInitializeInkPresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkHostWorkItem(::windows::core::IUnknown);
impl IInkHostWorkItem {
    pub unsafe fn Invoke(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Invoke)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IInkHostWorkItem> for ::windows::core::IUnknown {
    fn from(value: IInkHostWorkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IInkHostWorkItem> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IInkHostWorkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkHostWorkItem> for ::windows::core::IUnknown {
    fn from(value: &IInkHostWorkItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IInkHostWorkItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkHostWorkItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkHostWorkItem {}
impl ::core::fmt::Debug for IInkHostWorkItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkHostWorkItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkHostWorkItem {
    type Vtable = IInkHostWorkItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccda0a9a_1b78_4632_bb96_97800662e26c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkHostWorkItem_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
pub struct IInkPresenterDesktop(::windows::core::IUnknown);
impl IInkPresenterDesktop {
    pub unsafe fn SetRootVisual<'a, P0, P1>(&self, rootvisual: P0, device: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetRootVisual)(::windows::core::Interface::as_raw(self), rootvisual.into().abi(), device.into().abi()).ok()
    }
    pub unsafe fn SetCommitRequestHandler<'a, P0>(&self, handler: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IInkCommitRequestHandler>>,
    {
        (::windows::core::Interface::vtable(self).SetCommitRequestHandler)(::windows::core::Interface::as_raw(self), handler.into().abi()).ok()
    }
    pub unsafe fn GetSize(&self, width: &mut f32, height: &mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    pub unsafe fn SetSize(&self, width: f32, height: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSize)(::windows::core::Interface::as_raw(self), width, height).ok()
    }
    pub unsafe fn OnHighContrastChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnHighContrastChanged)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IInkPresenterDesktop> for ::windows::core::IUnknown {
    fn from(value: IInkPresenterDesktop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IInkPresenterDesktop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IInkPresenterDesktop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkPresenterDesktop> for ::windows::core::IUnknown {
    fn from(value: &IInkPresenterDesktop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IInkPresenterDesktop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkPresenterDesktop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkPresenterDesktop {}
impl ::core::fmt::Debug for IInkPresenterDesktop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkPresenterDesktop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInkPresenterDesktop {
    type Vtable = IInkPresenterDesktop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73f3c0d9_2e8b_48f3_895e_20cbd27b723b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterDesktop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetRootVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCommitRequestHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: *mut f32, height: *mut f32) -> ::windows::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: f32, height: f32) -> ::windows::core::HRESULT,
    pub OnHighContrastChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INK_HIGH_CONTRAST_ADJUSTMENT(pub i32);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_SYSTEM_COLORS_WHEN_NECESSARY: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(0i32);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_SYSTEM_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(1i32);
#[doc = "*Required features: `\"Win32_UI_Input_Ink\"`*"]
pub const USE_ORIGINAL_COLORS: INK_HIGH_CONTRAST_ADJUSTMENT = INK_HIGH_CONTRAST_ADJUSTMENT(2i32);
impl ::core::marker::Copy for INK_HIGH_CONTRAST_ADJUSTMENT {}
impl ::core::clone::Clone for INK_HIGH_CONTRAST_ADJUSTMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INK_HIGH_CONTRAST_ADJUSTMENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INK_HIGH_CONTRAST_ADJUSTMENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for INK_HIGH_CONTRAST_ADJUSTMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INK_HIGH_CONTRAST_ADJUSTMENT").field(&self.0).finish()
    }
}
pub const InkD2DRenderer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4044e60c_7b01_4671_a97c_04e0210a07a5);
pub const InkDesktopHost: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x062584a6_f830_4bdc_a4d2_0a10ab062b1d);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
