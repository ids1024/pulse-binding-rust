//! Routines for controlling module-stream-restore.

// This file is part of the PulseAudio Rust language binding.
//
// Copyright (c) 2017 Lyndon Brown
//
// This library is free software; you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation; either version
// 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this library;
// if not, see <http://www.gnu.org/licenses/>.

use std;
use capi;
use std::os::raw::{c_char, c_void};
use std::ffi::CString;
use std::ptr::{null, null_mut};
use super::{ContextInternal, Context};
use callbacks::ListResult;
use capi::pa_ext_stream_restore_info as InfoInternal;

/// Stores information about one entry in the stream database that is maintained by
/// module-stream-restore.
#[repr(C)]
pub struct Info {
    /// Identifier string of the stream. A string like "sink-input-by-role:" or similar followed by
    /// some arbitrary property value.
    pub name: *const c_char,
    /// The channel map for the volume field, if applicable.
    pub channel_map: ::channelmap::Map,
    /// The volume of the stream when it was seen last, if applicable and saved.
    pub volume: ::volume::ChannelVolumes,
    /// The sink/source of the stream when it was last seen, if applicable and saved.
    pub device: *const c_char,
    /// The boolean mute state of the stream when it was last seen, if applicable and saved.
    pub mute: i32,
}

impl From<InfoInternal> for Info {
    fn from(p: InfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

/// A wrapper object providing stream restore routines to a context.
/// Note: Saves a copy of active multi-use closure callbacks, which it frees on drop.
pub struct StreamRestore {
    context: *mut ContextInternal,
    /// Multi-use callback closure pointers
    cb_ptrs: CallbackPointers,
}

/// Holds copies of callback closure pointers, for those that are "multi-use" (may be fired multiple
/// times), for freeing at the appropriate time.
#[derive(Default)]
struct CallbackPointers {
    subscribe: SubscribeCb,
}

type SubscribeCb = ::callbacks::MultiUseCallback<FnMut(),
    extern "C" fn(*mut ContextInternal, *mut c_void)>;

impl Context {
    /// Returns a stream restore object linked to the current context, giving access to stream
    /// restore routines. See [`::context::ext_stream_restore`](ext_stream_restore/index.html).
    pub fn stream_restore(&self) -> StreamRestore {
        unsafe { capi::pa_context_ref(self.ptr) };
        StreamRestore::from_raw(self.ptr)
    }
}

impl StreamRestore {
    /// Create a new `DeviceManager` from an existing
    /// [`ContextInternal`](../struct.ContextInternal.html) pointer.
    fn from_raw(context: *mut ContextInternal) -> Self {
        Self { context: context, cb_ptrs: Default::default() }
    }

    /// Test if this extension module is available in the server.
    pub fn test<F>(&mut self, callback: F) -> ::operation::Operation
        where F: FnMut(u32) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(u32)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_ext_stream_restore_test(self.context,
            Some(super::ext_test_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Read all entries from the stream database.
    pub fn read<F>(&mut self, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const Info>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const Info>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_ext_stream_restore_read(self.context, Some(read_list_cb_proxy),
            cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Store entries in the stream database.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn write<F>(&mut self, mode: ::proplist::UpdateMode, data: &[&Info],
        apply_immediately: bool, callback: F) -> ::operation::Operation
        where F: FnMut(bool) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe {
            capi::pa_ext_stream_restore_write(self.context, mode,
                std::mem::transmute(data.as_ptr()), data.len() as u32, apply_immediately as i32,
                Some(super::success_cb_proxy), cb_data)
        };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Delete entries from the stream database.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn delete<F>(&mut self, streams: &[&str], callback: F) -> ::operation::Operation
        where F: FnMut(bool) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let mut c_streams: Vec<CString> = Vec::with_capacity(streams.len());
        for stream in streams {
            c_streams.push(CString::new(stream.clone()).unwrap());
        }

        // Capture array of pointers to the above CString values.
        // We also add a `NULL` pointer entry on the end, as expected by the C function called here.
        let mut c_stream_ptrs: Vec<*const c_char> = Vec::with_capacity(c_streams.len() + 1);
        for c_stream in c_streams {
            c_stream_ptrs.push(c_stream.as_ptr());
        }
        c_stream_ptrs.push(null());

        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_ext_stream_restore_delete(self.context, c_stream_ptrs.as_ptr(),
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Subscribe to changes in the stream database.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn subscribe<F>(&mut self, enable: bool, callback: F) -> ::operation::Operation
        where F: FnMut(bool) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_ext_stream_restore_subscribe(self.context, enable as i32,
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the subscription callback that is called when [`subscribe`](#method.subscribe) was
    /// called.
    pub fn set_subscribe_cb<F>(&mut self, callback: F)
        where F: FnMut() + 'static
    {
        let saved = &mut self.cb_ptrs.subscribe;
        *saved = SubscribeCb::new(Some(Box::new(callback)));
        let (cb_fn, cb_data) = saved.get_capi_params(super::ext_subscribe_cb_proxy);
        unsafe { capi::pa_ext_stream_restore_set_subscribe_cb(self.context, cb_fn, cb_data); }
    }
}

impl Drop for StreamRestore {
    fn drop(&mut self) {
        unsafe { capi::pa_context_unref(self.context) };
        self.context = null_mut::<ContextInternal>();
    }
}

/// Proxy for read list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn read_list_cb_proxy(_: *mut ContextInternal, i: *const InfoInternal, eol: i32,
    userdata: *mut c_void)
{
    assert!(!userdata.is_null());
    match eol {
        0 => { // Give item to real callback, do NOT destroy it
            assert!(!i.is_null());
            let callback = unsafe { &mut *(userdata as *mut Box<FnMut(ListResult<*const Info>)>) };
            callback(ListResult::Item(i as *const Info));
        },
        _ => { // End-of-list or failure, signal to real callback, do now destroy it
            let mut callback = unsafe {
                Box::from_raw(userdata as *mut Box<FnMut(ListResult<*const Info>)>)
            };
            callback(match eol < 0 { false => ListResult::End, true => ListResult::Error });
        },
    }
}
