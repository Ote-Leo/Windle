mod windows;

pub use windows::{Win32WindowHandle, WinRTWindowHandle, WindowsDisplayHandle};

/// Window that wraps around a raw window handle.
///
/// # Safety
///
/// Users can safely assume that non-`null`/`0` field are valid handles, and it is up to the
/// implementer of this trait to ensure that condition is upheld.
///
/// Despite that qualification, implementers should still make a best-effort attempt to fill in all
/// available fields. If an implementation doesn't, and a downstream user needs the field, it should
/// try to derive the field from other fields the implementrer *does* provide via whatever methods the
/// platform provides.
///
/// The exact handles returned by `raw_window_handle` must reamin consistent between multiple calls
/// to `raw_window_handle` as long as not indicated otherwise by platform specific events.
pub trait HasRawWindowHandle {
    fn raw_window_handle(&self) -> RawWindowHandle;
}

impl<'a, T: HasRawWindowHandle + ?Sized> HasRawWindowHandle for &'a T {
    fn raw_window_handle(&self) -> RawWindowHandle {
        (**self).raw_window_handle()
    }
}

/// A window handle for a particular windowing system.
///
/// Each variant contains a struct with fields to that windowing syste
/// (e.g. [`Win32WindowHandle`] will include a [HWND], [`WaylandWindowHandle`] uses [wl_surface],
/// etc.)
///
/// [HWND]: https://learn.microsoft.com/en-us/windows/win32/winmsg/about-windows#window-handle
/// [wl_surface]: https://wayland.freedesktop.org/docs/html/apa.html#protocol-spec-wl_surface
///
/// # Variant Availability
///
/// Note that all variants are present on all targets (non are disabled behind
/// `#[cfg]`s), but see the "Availability Hints" section on each variant for
/// some hints on where this variant might be expected.
///
/// Note that these "Availability Hints" are not normative. That is to say, a
/// [`HasRawWindowHandle`] implementer is completely allowed to return something
/// unexpected. (For example, it's legal for someone to return a
/// [`RawWindowHandle::Xlib`] on macOS, it would just be weird, and probably
/// requires something like XQuartz be used).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawWindowHandle {
    /// A raw window handle for Win32.
    ///
    /// ## Availability Hints
    /// This variant is used on Windows systems.
    Win32(Win32WindowHandle),
    /// A raw window handle for WinRT.
    ///
    /// ## Availability Hints
    /// This variant is used on Windows systems.
    WinRT(WinRTWindowHandle),
}

/// Display that wraps around a raw display handle.
///
/// # Safety
///
/// Users can safely assume that non-`null`/`0` fields are valid handles, and it is up to the
/// implementer of this trait to ensure that condition is upheld.
///
/// Despite that qualification, implementers should still make a best-effort attempt to fill in all
/// available fields. If an implementation doesn't, and a downstream user needs the field, it should
/// try to derive the field from other fields the implementer *does* provide via whatever methods the
/// platform provides.
///
/// The exact handles returned by `raw_display_handle` must remain consistent between multiple calls
/// to `raw_display_handle` as long as not indicated otherwise by platform specific events.
pub trait HasRawDisplayHandle {
    fn raw_display_handle(&self) -> RawDisplayHandle;
}

impl<'a, T: HasRawDisplayHandle + ?Sized> HasRawDisplayHandle for &'a T {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        (*self).raw_display_handle()
    }
}

/// A display server handle for a particular windowing system.
///
/// The display usually represents a connection to some display server, but it is not necessarily
/// tied to a particular window. Some APIs can use the dispaly handle without ever creating a window
/// handle (e.g. offscreen rendering, headless event handling).
///
/// Each variant contains a struct with fields specific to that windowing system
/// (e.g. [`XlibDisplayHandle`] contains a [Display] connection to an X Servera,
/// [`WaylandDisplayHandle`] uses [wl_display] to connect to a compositor). Not al windowing
/// systems have a separate display handle (or they haven't been implemented yet) and their variats
/// contain empty structs.
///
/// [Display]: https://www.x.org/releases/current/doc/libX11/libX11/libX11.html#Display_Functions
/// [wl_display]: https://wayland.freedesktop.org/docs/html/apb.html#Client-classwl__display
///
/// # Variant Availability
///
/// Note that these "Availability Hints" are not normative. That is to say, a
/// [`HasRawWindowHandle`] implementer is completely allowed to return something
/// unexpected. (For example, it's legal for someone to return a
/// [`RawWindowHandle::Xlib`] on macOS, it would just be weird, and probably
/// requires something like XQuartz be used).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawDisplayHandle {
    /// A raw dispaly handle for Win32.
    ///
    /// ## Availability Hints
    /// This variant is used on Windows systems.
    Windows(WindowsDisplayHandle),
}

macro_rules! from_impl {
    ($($to: ident, $enum: ident, $from: ty)*) => ($(
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                $to::$enum(value)
            }
        }
    )*)
}

from_impl!(RawDisplayHandle, Windows, WindowsDisplayHandle);

from_impl!(RawWindowHandle, Win32, Win32WindowHandle);
from_impl!(RawWindowHandle, WinRT, WinRTWindowHandle);
