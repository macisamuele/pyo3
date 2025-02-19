use crate::ffi3::pyport::{Py_hash_t, Py_ssize_t};
#[cfg(PyPy)]
use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[cfg(not(PyPy))]
pub struct PyObject {
    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    _ob_next: *mut PyObject,
    #[cfg(py_sys_config = "Py_TRACE_REFS")]
    _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg(PyPy)]
pub struct PyObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_pypy_link: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
}

#[cfg(py_sys_config = "Py_TRACE_REFS")]
#[cfg(not(PyPy))]
pub const PyObject_HEAD_INIT: PyObject = PyObject {
    _ob_next: ::std::ptr::null_mut(),
    _ob_prev: ::std::ptr::null_mut(),
    ob_refcnt: 1,
    ob_type: ::std::ptr::null_mut(),
};

#[cfg(not(py_sys_config = "Py_TRACE_REFS"))]
#[cfg(not(PyPy))]
pub const PyObject_HEAD_INIT: PyObject = PyObject {
    ob_refcnt: 1,
    ob_type: ::std::ptr::null_mut(),
};

#[cfg(py_sys_config = "Py_TRACE_REFS")]
#[cfg(PyPy)]
pub const PyObject_HEAD_INIT: PyObject = PyObject {
    _ob_next: ::std::ptr::null_mut(),
    _ob_prev: ::std::ptr::null_mut(),
    ob_refcnt: 1,
    ob_pypy_link: 0,
    ob_type: ::std::ptr::null_mut(),
};

#[cfg(not(py_sys_config = "Py_TRACE_REFS"))]
#[cfg(PyPy)]
pub const PyObject_HEAD_INIT: PyObject = PyObject {
    ob_refcnt: 1,
    ob_pypy_link: 0,
    ob_type: ::std::ptr::null_mut(),
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PyVarObject {
    pub ob_base: PyObject,
    pub ob_size: Py_ssize_t,
}

#[inline]
pub unsafe fn Py_REFCNT(ob: *mut PyObject) -> Py_ssize_t {
    if ob.is_null() {
        panic!();
    }
    (*ob).ob_refcnt
}

#[cfg(PyPy)]
pub unsafe fn _PyObject_NextNotImplemented(arg1: *mut PyObject) -> *mut PyObject {
    return crate::ffi3::pyerrors::PyErr_Format(
        crate::ffi3::pyerrors::PyExc_TypeError,
        CStr::from_bytes_with_nul(b"'%.200s' object is not iterable\0")
            .unwrap()
            .as_ptr(),
        Py_TYPE((*(arg1 as *mut PyTypeObject)).tp_name as *mut PyObject),
    );
}

#[inline]
pub unsafe fn Py_TYPE(ob: *mut PyObject) -> *mut PyTypeObject {
    (*ob).ob_type
}

#[inline]
pub unsafe fn Py_SIZE(ob: *mut PyObject) -> Py_ssize_t {
    (*(ob as *mut PyVarObject)).ob_size
}

pub type unaryfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject;

pub type binaryfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;

pub type ternaryfunc = unsafe extern "C" fn(
    arg1: *mut PyObject,
    arg2: *mut PyObject,
    arg3: *mut PyObject,
) -> *mut PyObject;

pub type inquiry = unsafe extern "C" fn(arg1: *mut PyObject) -> c_int;

pub type lenfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> Py_ssize_t;

pub type ssizeargfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: Py_ssize_t) -> *mut PyObject;

pub type ssizessizeargfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: Py_ssize_t, arg3: Py_ssize_t) -> *mut PyObject;

pub type ssizeobjargproc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: Py_ssize_t, arg3: *mut PyObject) -> c_int;

pub type ssizessizeobjargproc = unsafe extern "C" fn(
    arg1: *mut PyObject,
    arg2: Py_ssize_t,
    arg3: Py_ssize_t,
    arg4: *mut PyObject,
) -> c_int;

pub type objobjargproc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> c_int;

#[cfg(not(Py_LIMITED_API))]
mod bufferinfo {
    use crate::ffi3::pyport::Py_ssize_t;
    use std::mem;
    use std::os::raw::{c_char, c_int, c_void};

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Py_buffer {
        pub buf: *mut c_void,
        pub obj: *mut crate::ffi3::PyObject,
        pub len: Py_ssize_t,
        pub itemsize: Py_ssize_t,
        pub readonly: c_int,
        pub ndim: c_int,
        pub format: *mut c_char,
        pub shape: *mut Py_ssize_t,
        pub strides: *mut Py_ssize_t,
        pub suboffsets: *mut Py_ssize_t,
        pub internal: *mut c_void,
    }

    impl Default for Py_buffer {
        #[inline]
        fn default() -> Self {
            unsafe { mem::zeroed() }
        }
    }

    pub type getbufferproc = unsafe extern "C" fn(
        arg1: *mut crate::ffi3::PyObject,
        arg2: *mut Py_buffer,
        arg3: c_int,
    ) -> c_int;
    pub type releasebufferproc =
        unsafe extern "C" fn(arg1: *mut crate::ffi3::PyObject, arg2: *mut Py_buffer) -> ();

    /// Maximum number of dimensions
    pub const PyBUF_MAX_NDIM: c_int = 64;

    /* Flags for getting buffers */
    pub const PyBUF_SIMPLE: c_int = 0;
    pub const PyBUF_WRITABLE: c_int = 0x0001;
    /*  we used to include an E, backwards compatible alias  */
    pub const PyBUF_WRITEABLE: c_int = PyBUF_WRITABLE;
    pub const PyBUF_FORMAT: c_int = 0x0004;
    pub const PyBUF_ND: c_int = 0x0008;
    pub const PyBUF_STRIDES: c_int = (0x0010 | PyBUF_ND);
    pub const PyBUF_C_CONTIGUOUS: c_int = (0x0020 | PyBUF_STRIDES);
    pub const PyBUF_F_CONTIGUOUS: c_int = (0x0040 | PyBUF_STRIDES);
    pub const PyBUF_ANY_CONTIGUOUS: c_int = (0x0080 | PyBUF_STRIDES);
    pub const PyBUF_INDIRECT: c_int = (0x0100 | PyBUF_STRIDES);

    pub const PyBUF_CONTIG: c_int = (PyBUF_ND | PyBUF_WRITABLE);
    pub const PyBUF_CONTIG_RO: c_int = (PyBUF_ND);

    pub const PyBUF_STRIDED: c_int = (PyBUF_STRIDES | PyBUF_WRITABLE);
    pub const PyBUF_STRIDED_RO: c_int = (PyBUF_STRIDES);

    pub const PyBUF_RECORDS: c_int = (PyBUF_STRIDES | PyBUF_WRITABLE | PyBUF_FORMAT);
    pub const PyBUF_RECORDS_RO: c_int = (PyBUF_STRIDES | PyBUF_FORMAT);

    pub const PyBUF_FULL: c_int = (PyBUF_INDIRECT | PyBUF_WRITABLE | PyBUF_FORMAT);
    pub const PyBUF_FULL_RO: c_int = (PyBUF_INDIRECT | PyBUF_FORMAT);

    pub const PyBUF_READ: c_int = 0x100;
    pub const PyBUF_WRITE: c_int = 0x200;
}
#[cfg(not(Py_LIMITED_API))]
pub use self::bufferinfo::*;

pub type objobjproc = unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> c_int;
pub type visitproc = unsafe extern "C" fn(object: *mut PyObject, arg: *mut c_void) -> c_int;
pub type traverseproc =
    unsafe extern "C" fn(slf: *mut PyObject, visit: visitproc, arg: *mut c_void) -> c_int;

pub type freefunc = unsafe extern "C" fn(arg1: *mut c_void);
pub type destructor = unsafe extern "C" fn(arg1: *mut PyObject);
#[cfg(not(Py_LIMITED_API))]
pub type printfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut ::libc::FILE, arg3: c_int) -> c_int;
pub type getattrfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut c_char) -> *mut PyObject;
pub type getattrofunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;
pub type setattrfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut c_char, arg3: *mut PyObject) -> c_int;
pub type setattrofunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> c_int;
pub type reprfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject;
pub type hashfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> Py_hash_t;
pub type richcmpfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject, arg3: c_int) -> *mut PyObject;
pub type getiterfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject;
pub type iternextfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> *mut PyObject;
pub type descrgetfunc = unsafe extern "C" fn(
    arg1: *mut PyObject,
    arg2: *mut PyObject,
    arg3: *mut PyObject,
) -> *mut PyObject;
pub type descrsetfunc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> c_int;
pub type initproc =
    unsafe extern "C" fn(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> c_int;
pub type newfunc = unsafe extern "C" fn(
    arg1: *mut PyTypeObject,
    arg2: *mut PyObject,
    arg3: *mut PyObject,
) -> *mut PyObject;
pub type allocfunc =
    unsafe extern "C" fn(arg1: *mut PyTypeObject, arg2: Py_ssize_t) -> *mut PyObject;

#[cfg(Py_LIMITED_API)]
mod typeobject {
    pub enum PyTypeObject {}
}

#[cfg(not(Py_LIMITED_API))]
mod typeobject {
    use crate::ffi3::pyport::Py_ssize_t;
    use crate::ffi3::{self, object};
    use std::mem;
    use std::os::raw::{c_char, c_uint, c_ulong, c_void};
    use std::ptr;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PyNumberMethods {
        pub nb_add: Option<object::binaryfunc>,
        pub nb_subtract: Option<object::binaryfunc>,
        pub nb_multiply: Option<object::binaryfunc>,
        pub nb_remainder: Option<object::binaryfunc>,
        pub nb_divmod: Option<object::binaryfunc>,
        pub nb_power: Option<object::ternaryfunc>,
        pub nb_negative: Option<object::unaryfunc>,
        pub nb_positive: Option<object::unaryfunc>,
        pub nb_absolute: Option<object::unaryfunc>,
        pub nb_bool: Option<object::inquiry>,
        pub nb_invert: Option<object::unaryfunc>,
        pub nb_lshift: Option<object::binaryfunc>,
        pub nb_rshift: Option<object::binaryfunc>,
        pub nb_and: Option<object::binaryfunc>,
        pub nb_xor: Option<object::binaryfunc>,
        pub nb_or: Option<object::binaryfunc>,
        pub nb_int: Option<object::unaryfunc>,
        pub nb_reserved: *mut c_void,
        pub nb_float: Option<object::unaryfunc>,
        pub nb_inplace_add: Option<object::binaryfunc>,
        pub nb_inplace_subtract: Option<object::binaryfunc>,
        pub nb_inplace_multiply: Option<object::binaryfunc>,
        pub nb_inplace_remainder: Option<object::binaryfunc>,
        pub nb_inplace_power: Option<object::ternaryfunc>,
        pub nb_inplace_lshift: Option<object::binaryfunc>,
        pub nb_inplace_rshift: Option<object::binaryfunc>,
        pub nb_inplace_and: Option<object::binaryfunc>,
        pub nb_inplace_xor: Option<object::binaryfunc>,
        pub nb_inplace_or: Option<object::binaryfunc>,
        pub nb_floor_divide: Option<object::binaryfunc>,
        pub nb_true_divide: Option<object::binaryfunc>,
        pub nb_inplace_floor_divide: Option<object::binaryfunc>,
        pub nb_inplace_true_divide: Option<object::binaryfunc>,
        pub nb_index: Option<object::unaryfunc>,
        pub nb_matrix_multiply: Option<object::binaryfunc>,
        pub nb_inplace_matrix_multiply: Option<object::binaryfunc>,
    }

    impl Default for PyNumberMethods {
        #[inline]
        fn default() -> Self {
            PyNumberMethods_INIT
        }
    }
    macro_rules! as_expr {
        ($e:expr) => {
            $e
        };
    }

    macro_rules! py_number_methods_init {
        ($($tail:tt)*) => {
            as_expr! {
                PyNumberMethods {
                    nb_add: None,
                    nb_subtract: None,
                    nb_multiply: None,
                    nb_remainder: None,
                    nb_divmod: None,
                    nb_power: None,
                    nb_negative: None,
                    nb_positive: None,
                    nb_absolute: None,
                    nb_bool: None,
                    nb_invert: None,
                    nb_lshift: None,
                    nb_rshift: None,
                    nb_and: None,
                    nb_xor: None,
                    nb_or: None,
                    nb_int: None,
                    nb_reserved: ::std::ptr::null_mut(),
                    nb_float: None,
                    nb_inplace_add: None,
                    nb_inplace_subtract: None,
                    nb_inplace_multiply: None,
                    nb_inplace_remainder: None,
                    nb_inplace_power: None,
                    nb_inplace_lshift: None,
                    nb_inplace_rshift: None,
                    nb_inplace_and: None,
                    nb_inplace_xor: None,
                    nb_inplace_or: None,
                    nb_floor_divide: None,
                    nb_true_divide: None,
                    nb_inplace_floor_divide: None,
                    nb_inplace_true_divide: None,
                    nb_index: None,
                    $($tail)*
                }
            }
        }
    }

    pub const PyNumberMethods_INIT: PyNumberMethods = py_number_methods_init! {
        nb_matrix_multiply: None,
        nb_inplace_matrix_multiply: None,
    };

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PySequenceMethods {
        pub sq_length: Option<object::lenfunc>,
        pub sq_concat: Option<object::binaryfunc>,
        pub sq_repeat: Option<object::ssizeargfunc>,
        pub sq_item: Option<object::ssizeargfunc>,
        pub was_sq_slice: *mut c_void,
        pub sq_ass_item: Option<object::ssizeobjargproc>,
        pub was_sq_ass_slice: *mut c_void,
        pub sq_contains: Option<object::objobjproc>,
        pub sq_inplace_concat: Option<object::binaryfunc>,
        pub sq_inplace_repeat: Option<object::ssizeargfunc>,
    }

    impl Default for PySequenceMethods {
        #[inline]
        fn default() -> Self {
            unsafe { mem::zeroed() }
        }
    }
    pub const PySequenceMethods_INIT: PySequenceMethods = PySequenceMethods {
        sq_length: None,
        sq_concat: None,
        sq_repeat: None,
        sq_item: None,
        was_sq_slice: ptr::null_mut(),
        sq_ass_item: None,
        was_sq_ass_slice: ptr::null_mut(),
        sq_contains: None,
        sq_inplace_concat: None,
        sq_inplace_repeat: None,
    };
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PyMappingMethods {
        pub mp_length: Option<object::lenfunc>,
        pub mp_subscript: Option<object::binaryfunc>,
        pub mp_ass_subscript: Option<object::objobjargproc>,
    }

    impl Default for PyMappingMethods {
        #[inline]
        fn default() -> Self {
            unsafe { mem::zeroed() }
        }
    }
    pub const PyMappingMethods_INIT: PyMappingMethods = PyMappingMethods {
        mp_length: None,
        mp_subscript: None,
        mp_ass_subscript: None,
    };
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PyAsyncMethods {
        pub am_await: Option<object::unaryfunc>,
        pub am_aiter: Option<object::unaryfunc>,
        pub am_anext: Option<object::unaryfunc>,
    }

    impl Default for PyAsyncMethods {
        #[inline]
        fn default() -> Self {
            unsafe { mem::zeroed() }
        }
    }
    pub const PyAsyncMethods_INIT: PyAsyncMethods = PyAsyncMethods {
        am_await: None,
        am_aiter: None,
        am_anext: None,
    };
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct PyBufferProcs {
        pub bf_getbuffer: Option<object::getbufferproc>,
        pub bf_releasebuffer: Option<object::releasebufferproc>,
    }

    impl Default for PyBufferProcs {
        #[inline]
        fn default() -> Self {
            unsafe { mem::zeroed() }
        }
    }
    pub const PyBufferProcs_INIT: PyBufferProcs = PyBufferProcs {
        bf_getbuffer: None,
        bf_releasebuffer: None,
    };

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct PyTypeObject {
        #[cfg(PyPy)]
        pub ob_refcnt: Py_ssize_t,
        #[cfg(PyPy)]
        pub ob_pypy_link: Py_ssize_t,
        #[cfg(PyPy)]
        pub ob_type: *mut PyTypeObject,
        #[cfg(PyPy)]
        pub ob_size: Py_ssize_t,
        #[cfg(not(PyPy))]
        pub ob_base: object::PyVarObject,
        pub tp_name: *const c_char,
        pub tp_basicsize: Py_ssize_t,
        pub tp_itemsize: Py_ssize_t,
        pub tp_dealloc: Option<object::destructor>,
        pub tp_print: Option<object::printfunc>,
        pub tp_getattr: Option<object::getattrfunc>,
        pub tp_setattr: Option<object::setattrfunc>,
        pub tp_as_async: *mut PyAsyncMethods,
        pub tp_repr: Option<object::reprfunc>,
        pub tp_as_number: *mut PyNumberMethods,
        pub tp_as_sequence: *mut PySequenceMethods,
        pub tp_as_mapping: *mut PyMappingMethods,
        pub tp_hash: Option<object::hashfunc>,
        pub tp_call: Option<object::ternaryfunc>,
        pub tp_str: Option<object::reprfunc>,
        pub tp_getattro: Option<object::getattrofunc>,
        pub tp_setattro: Option<object::setattrofunc>,
        pub tp_as_buffer: *mut PyBufferProcs,
        pub tp_flags: c_ulong,
        pub tp_doc: *const c_char,
        pub tp_traverse: Option<object::traverseproc>,
        pub tp_clear: Option<object::inquiry>,
        pub tp_richcompare: Option<object::richcmpfunc>,
        pub tp_weaklistoffset: Py_ssize_t,
        pub tp_iter: Option<object::getiterfunc>,
        pub tp_iternext: Option<object::iternextfunc>,
        pub tp_methods: *mut ffi3::methodobject::PyMethodDef,
        pub tp_members: *mut ffi3::structmember::PyMemberDef,
        pub tp_getset: *mut ffi3::descrobject::PyGetSetDef,
        pub tp_base: *mut PyTypeObject,
        pub tp_dict: *mut ffi3::object::PyObject,
        pub tp_descr_get: Option<ffi3::object::descrgetfunc>,
        pub tp_descr_set: Option<ffi3::object::descrsetfunc>,
        pub tp_dictoffset: Py_ssize_t,
        pub tp_init: Option<ffi3::object::initproc>,
        pub tp_alloc: Option<ffi3::object::allocfunc>,
        pub tp_new: Option<ffi3::object::newfunc>,
        pub tp_free: Option<ffi3::object::freefunc>,
        pub tp_is_gc: Option<ffi3::object::inquiry>,
        pub tp_bases: *mut ffi3::object::PyObject,
        pub tp_mro: *mut ffi3::object::PyObject,
        pub tp_cache: *mut ffi3::object::PyObject,
        pub tp_subclasses: *mut ffi3::object::PyObject,
        pub tp_weaklist: *mut ffi3::object::PyObject,
        pub tp_del: Option<ffi3::object::destructor>,
        pub tp_version_tag: c_uint,
        pub tp_finalize: Option<ffi3::object::destructor>,
        #[cfg(PyPy)]
        pub tp_pypy_flags: ::std::os::raw::c_long,
        #[cfg(py_sys_config = "COUNT_ALLOCS")]
        pub tp_allocs: Py_ssize_t,
        #[cfg(py_sys_config = "COUNT_ALLOCS")]
        pub tp_frees: Py_ssize_t,
        #[cfg(py_sys_config = "COUNT_ALLOCS")]
        pub tp_maxalloc: Py_ssize_t,
        #[cfg(py_sys_config = "COUNT_ALLOCS")]
        pub tp_prev: *mut PyTypeObject,
        #[cfg(py_sys_config = "COUNT_ALLOCS")]
        pub tp_next: *mut PyTypeObject,
    }

    macro_rules! _type_object_init {
        ({$($head:tt)*} $tp_as_async:ident, $($tail:tt)*) => {
            as_expr! {
                PyTypeObject {
                    $($head)*
                    tp_name: ptr::null(),
                    tp_basicsize: 0,
                    tp_itemsize: 0,
                    tp_dealloc: None,
                    tp_print: None,
                    tp_getattr: None,
                    tp_setattr: None,
                    $tp_as_async: ptr::null_mut(),
                    tp_repr: None,
                    tp_as_number: ptr::null_mut(),
                    tp_as_sequence: ptr::null_mut(),
                    tp_as_mapping: ptr::null_mut(),
                    tp_hash: None,
                    tp_call: None,
                    tp_str: None,
                    tp_getattro: None,
                    tp_setattro: None,
                    tp_as_buffer: ptr::null_mut(),
                    tp_flags: ffi3::object::Py_TPFLAGS_DEFAULT,
                    tp_doc: ptr::null(),
                    tp_traverse: None,
                    tp_clear: None,
                    tp_richcompare: None,
                    tp_weaklistoffset: 0,
                    tp_iter: None,
                    tp_iternext: None,
                    tp_methods: ptr::null_mut(),
                    tp_members: ptr::null_mut(),
                    tp_getset: ptr::null_mut(),
                    tp_base: ptr::null_mut(),
                    tp_dict: ptr::null_mut(),
                    tp_descr_get: None,
                    tp_descr_set: None,
                    tp_dictoffset: 0,
                    tp_init: None,
                    tp_alloc: None,
                    tp_new: None,
                    tp_free: None,
                    tp_is_gc: None,
                    tp_bases: ptr::null_mut(),
                    tp_mro: ptr::null_mut(),
                    tp_cache: ptr::null_mut(),
                    tp_subclasses: ptr::null_mut(),
                    tp_weaklist: ptr::null_mut(),
                    tp_del: None,
                    tp_version_tag: 0,
                    $($tail)*
                }
            }
        }
    }

    #[cfg(PyPy)]
    macro_rules! py_type_object_init {
        ($tp_as_async:ident, $($tail:tt)*) => {
            _type_object_init!({
                    ob_refcnt: 1,
                    ob_pypy_link: 0,
                    ob_type: ptr::null_mut(),
                    ob_size: 0,
                }
                $tp_as_async,
                tp_pypy_flags: 0,
                $($tail)*
            )
        }
    }

    #[cfg(not(PyPy))]
    macro_rules! py_type_object_init {
        ($tp_as_async:ident, $($tail:tt)*) => {
            _type_object_init!({
                ob_base: ffi3::object::PyVarObject {
                    ob_base: ffi3::object::PyObject_HEAD_INIT,
                    ob_size: 0
                },}
                $tp_as_async,
                $($tail)*
            )
        }
    }

    #[cfg(py_sys_config = "COUNT_ALLOCS")]
    macro_rules! py_type_object_init_with_count_allocs {
        ($tp_as_async:ident, $($tail:tt)*) => {
            py_type_object_init!($tp_as_async,
                $($tail)*
                tp_allocs: 0,
                tp_frees: 0,
                tp_maxalloc: 0,
                tp_prev: ptr::null_mut(),
                tp_next: ptr::null_mut(),
            )
        }
    }

    #[cfg(not(py_sys_config = "COUNT_ALLOCS"))]
    macro_rules! py_type_object_init_with_count_allocs {
        ($tp_as_async:ident, $($tail:tt)*) => {
            py_type_object_init!($tp_as_async, $($tail)*)
        }
    }

    pub const PyTypeObject_INIT: PyTypeObject =
        py_type_object_init_with_count_allocs!(tp_as_async, tp_finalize: None,);

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PyHeapTypeObject {
        pub ht_type: PyTypeObject,
        pub as_async: PyAsyncMethods,
        pub as_number: PyNumberMethods,
        pub as_mapping: PyMappingMethods,
        pub as_sequence: PySequenceMethods,
        pub as_buffer: PyBufferProcs,
        pub ht_name: *mut ffi3::object::PyObject,
        pub ht_slots: *mut ffi3::object::PyObject,
        pub ht_qualname: *mut ffi3::object::PyObject,
        pub ht_cached_keys: *mut c_void,
    }

    impl Default for PyHeapTypeObject {
        #[inline]
        fn default() -> Self {
            unsafe { mem::zeroed() }
        }
    }

    #[inline]
    pub unsafe fn PyHeapType_GET_MEMBERS(
        etype: *mut PyHeapTypeObject,
    ) -> *mut ffi3::structmember::PyMemberDef {
        let py_type = ffi3::object::Py_TYPE(etype as *mut ffi3::object::PyObject);
        let ptr = etype.offset((*py_type).tp_basicsize);
        ptr as *mut ffi3::structmember::PyMemberDef
    }
}

// The exported types depend on whether Py_LIMITED_API is set
pub use self::typeobject::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyType_Slot {
    pub slot: c_int,
    pub pfunc: *mut c_void,
}

impl Default for PyType_Slot {
    fn default() -> PyType_Slot {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyType_Spec {
    pub name: *const c_char,
    pub basicsize: c_int,
    pub itemsize: c_int,
    pub flags: c_uint,
    pub slots: *mut PyType_Slot,
}

impl Default for PyType_Spec {
    fn default() -> PyType_Spec {
        unsafe { mem::zeroed() }
    }
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyType_FromSpec")]
    pub fn PyType_FromSpec(arg1: *mut PyType_Spec) -> *mut PyObject;

    #[cfg_attr(PyPy, link_name = "PyPyType_FromSpecWithBases")]
    pub fn PyType_FromSpecWithBases(arg1: *mut PyType_Spec, arg2: *mut PyObject) -> *mut PyObject;

    pub fn PyType_GetSlot(arg1: *mut PyTypeObject, arg2: c_int) -> *mut c_void;
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyType_IsSubtype")]
    pub fn PyType_IsSubtype(a: *mut PyTypeObject, b: *mut PyTypeObject) -> c_int;
}

#[inline]
pub unsafe fn PyObject_TypeCheck(ob: *mut PyObject, tp: *mut PyTypeObject) -> c_int {
    (Py_TYPE(ob) == tp || PyType_IsSubtype(Py_TYPE(ob), tp) != 0) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    /// built-in 'type'
    #[cfg_attr(PyPy, link_name = "PyPyType_Type")]
    pub static mut PyType_Type: PyTypeObject;
    /// built-in 'object'
    #[cfg_attr(PyPy, link_name = "PyPyBaseObject_Type")]
    pub static mut PyBaseObject_Type: PyTypeObject;
    /// built-in 'super'
    pub static mut PySuper_Type: PyTypeObject;

    pub fn PyType_GetFlags(arg1: *mut PyTypeObject) -> c_ulong;
}

#[inline]
pub unsafe fn PyType_Check(op: *mut PyObject) -> c_int {
    PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_TYPE_SUBCLASS)
}

#[inline]
pub unsafe fn PyType_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyType_Type) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyType_Ready")]
    pub fn PyType_Ready(t: *mut PyTypeObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyType_GenericAlloc")]
    pub fn PyType_GenericAlloc(t: *mut PyTypeObject, nitems: Py_ssize_t) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyType_GenericNew")]
    pub fn PyType_GenericNew(
        t: *mut PyTypeObject,
        args: *mut PyObject,
        kwds: *mut PyObject,
    ) -> *mut PyObject;
    pub fn PyType_ClearCache() -> c_uint;
    #[cfg_attr(PyPy, link_name = "PyPyType_Modified")]
    pub fn PyType_Modified(t: *mut PyTypeObject);

    #[cfg(not(Py_LIMITED_API))]
    #[cfg_attr(PyPy, link_name = "PyPyObject_Print")]
    pub fn PyObject_Print(o: *mut PyObject, fp: *mut ::libc::FILE, flags: c_int) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Repr")]
    pub fn PyObject_Repr(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Str")]
    pub fn PyObject_Str(o: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_ASCII")]
    pub fn PyObject_ASCII(arg1: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Bytes")]
    pub fn PyObject_Bytes(arg1: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_RichCompare")]
    pub fn PyObject_RichCompare(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: c_int,
    ) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_RichCompareBool")]
    pub fn PyObject_RichCompareBool(arg1: *mut PyObject, arg2: *mut PyObject, arg3: c_int)
        -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_GetAttrString")]
    pub fn PyObject_GetAttrString(arg1: *mut PyObject, arg2: *const c_char) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_SetAttrString")]
    pub fn PyObject_SetAttrString(
        arg1: *mut PyObject,
        arg2: *const c_char,
        arg3: *mut PyObject,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_HasAttrString")]
    pub fn PyObject_HasAttrString(arg1: *mut PyObject, arg2: *const c_char) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_GetAttr")]
    pub fn PyObject_GetAttr(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_SetAttr")]
    pub fn PyObject_SetAttr(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject)
        -> c_int;
    pub fn PyObject_HasAttr(arg1: *mut PyObject, arg2: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_SelfIter")]
    pub fn PyObject_SelfIter(arg1: *mut PyObject) -> *mut PyObject;

    #[cfg(not(Py_LIMITED_API))]
    #[cfg(not(PyPy))]
    pub fn _PyObject_NextNotImplemented(arg1: *mut PyObject) -> *mut PyObject;

    #[cfg_attr(PyPy, link_name = "PyPyObject_GenericGetAttr")]
    pub fn PyObject_GenericGetAttr(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;
    #[cfg_attr(PyPy, link_name = "PyPyObject_GenericSetAttr")]
    pub fn PyObject_GenericSetAttr(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut PyObject,
    ) -> c_int;
    pub fn PyObject_GenericGetDict(arg1: *mut PyObject, arg2: *mut c_void) -> *mut PyObject;
    pub fn PyObject_GenericSetDict(
        arg1: *mut PyObject,
        arg2: *mut PyObject,
        arg3: *mut c_void,
    ) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Hash")]
    pub fn PyObject_Hash(arg1: *mut PyObject) -> Py_hash_t;
    #[cfg_attr(PyPy, link_name = "PyPyObject_HashNotImplemented")]
    pub fn PyObject_HashNotImplemented(arg1: *mut PyObject) -> Py_hash_t;
    #[cfg_attr(PyPy, link_name = "PyPyObject_IsTrue")]
    pub fn PyObject_IsTrue(arg1: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_Not")]
    pub fn PyObject_Not(arg1: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyCallable_Check")]
    pub fn PyCallable_Check(arg1: *mut PyObject) -> c_int;
    #[cfg_attr(PyPy, link_name = "PyPyObject_ClearWeakRefs")]
    pub fn PyObject_ClearWeakRefs(arg1: *mut PyObject) -> ();
    #[cfg(not(Py_LIMITED_API))]
    pub fn PyObject_CallFinalizer(arg1: *mut PyObject) -> ();
    #[cfg(not(Py_LIMITED_API))]
    #[cfg_attr(PyPy, link_name = "PyPyObject_CallFinalizerFromDealloc")]
    pub fn PyObject_CallFinalizerFromDealloc(arg1: *mut PyObject) -> c_int;

    #[cfg_attr(PyPy, link_name = "PyPyObject_Dir")]
    pub fn PyObject_Dir(arg1: *mut PyObject) -> *mut PyObject;
    pub fn Py_ReprEnter(arg1: *mut PyObject) -> c_int;
    pub fn Py_ReprLeave(arg1: *mut PyObject) -> ();
}

// Flag bits for printing:
pub const Py_PRINT_RAW: c_int = 1; // No string quotes etc.

/// Set if the type object is dynamically allocated
pub const Py_TPFLAGS_HEAPTYPE: c_ulong = (1 << 9);

/// Set if the type allows subclassing
pub const Py_TPFLAGS_BASETYPE: c_ulong = (1 << 10);

/// Set if the type is 'ready' -- fully initialized
pub const Py_TPFLAGS_READY: c_ulong = (1 << 12);

/// Set while the type is being 'readied', to prevent recursive ready calls
pub const Py_TPFLAGS_READYING: c_ulong = (1 << 13);

/// Objects support garbage collection (see objimp.h)
pub const Py_TPFLAGS_HAVE_GC: c_ulong = (1 << 14);

const Py_TPFLAGS_HAVE_STACKLESS_EXTENSION: c_ulong = 0;

/// Objects support type attribute cache
pub const Py_TPFLAGS_HAVE_VERSION_TAG: c_ulong = (1 << 18);
pub const Py_TPFLAGS_VALID_VERSION_TAG: c_ulong = (1 << 19);

/* Type is abstract and cannot be instantiated */
pub const Py_TPFLAGS_IS_ABSTRACT: c_ulong = (1 << 20);

/* These flags are used to determine if a type is a subclass. */
pub const Py_TPFLAGS_LONG_SUBCLASS: c_ulong = (1 << 24);
pub const Py_TPFLAGS_LIST_SUBCLASS: c_ulong = (1 << 25);
pub const Py_TPFLAGS_TUPLE_SUBCLASS: c_ulong = (1 << 26);
pub const Py_TPFLAGS_BYTES_SUBCLASS: c_ulong = (1 << 27);
pub const Py_TPFLAGS_UNICODE_SUBCLASS: c_ulong = (1 << 28);
pub const Py_TPFLAGS_DICT_SUBCLASS: c_ulong = (1 << 29);
pub const Py_TPFLAGS_BASE_EXC_SUBCLASS: c_ulong = (1 << 30);
pub const Py_TPFLAGS_TYPE_SUBCLASS: c_ulong = (1 << 31);

pub const Py_TPFLAGS_DEFAULT: c_ulong =
    Py_TPFLAGS_HAVE_STACKLESS_EXTENSION | Py_TPFLAGS_HAVE_VERSION_TAG;

pub const Py_TPFLAGS_HAVE_FINALIZE: c_ulong = 1;

#[inline]
#[cfg(Py_LIMITED_API)]
pub unsafe fn PyType_HasFeature(t: *mut PyTypeObject, f: c_ulong) -> c_int {
    ((PyType_GetFlags(t) & f) != 0) as c_int
}

#[inline]
#[cfg(not(Py_LIMITED_API))]
pub unsafe fn PyType_HasFeature(t: *mut PyTypeObject, f: c_ulong) -> c_int {
    (((*t).tp_flags & f) != 0) as c_int
}

#[inline]
pub unsafe fn PyType_FastSubclass(t: *mut PyTypeObject, f: c_ulong) -> c_int {
    PyType_HasFeature(t, f)
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "_PyPy_Dealloc")]
    pub fn _Py_Dealloc(arg1: *mut PyObject) -> ();
}

// Reference counting macros.
#[inline]
pub unsafe fn Py_INCREF(op: *mut PyObject) {
    if cfg!(py_sys_config = "Py_REF_DEBUG") {
        Py_IncRef(op)
    } else {
        (*op).ob_refcnt += 1
    }
}

#[inline]
pub unsafe fn Py_DECREF(op: *mut PyObject) {
    if cfg!(py_sys_config = "Py_REF_DEBUG") {
        Py_DecRef(op)
    } else {
        (*op).ob_refcnt -= 1;
        if (*op).ob_refcnt == 0 {
            _Py_Dealloc(op)
        }
    }
}

#[inline]
pub unsafe fn Py_CLEAR(op: &mut *mut PyObject) {
    let tmp = *op;
    if !tmp.is_null() {
        *op = ptr::null_mut();
        Py_DECREF(tmp);
    }
}

#[inline]
pub unsafe fn Py_XINCREF(op: *mut PyObject) {
    if !op.is_null() {
        Py_INCREF(op)
    }
}

#[inline]
pub unsafe fn Py_XDECREF(op: *mut PyObject) {
    if !op.is_null() {
        Py_DECREF(op)
    }
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPy_IncRef")]
    pub fn Py_IncRef(o: *mut PyObject);
    #[cfg_attr(PyPy, link_name = "PyPy_DecRef")]
    pub fn Py_DecRef(o: *mut PyObject);

    #[cfg_attr(PyPy, link_name = "_PyPy_NoneStruct")]
    static mut _Py_NoneStruct: PyObject;
    #[cfg_attr(PyPy, link_name = "_PyPy_NotImplementedStruct")]
    static mut _Py_NotImplementedStruct: PyObject;
}

#[inline]
pub unsafe fn Py_None() -> *mut PyObject {
    &mut _Py_NoneStruct
}

#[inline]
pub unsafe fn Py_NotImplemented() -> *mut PyObject {
    &mut _Py_NotImplementedStruct
}

/* Rich comparison opcodes */
pub const Py_LT: c_int = 0;
pub const Py_LE: c_int = 1;
pub const Py_EQ: c_int = 2;
pub const Py_NE: c_int = 3;
pub const Py_GT: c_int = 4;
pub const Py_GE: c_int = 5;

#[inline]
pub fn PyObject_Check(_arg1: *mut PyObject) -> c_int {
    1
}

#[inline]
pub fn PySuper_Check(_arg1: *mut PyObject) -> c_int {
    0
}
