use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::{BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::ffi::{CStr, CString, OsStr, OsString};
use std::marker::PhantomData;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, Wrapping};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime, Instant};

use crate::{Arity, Documentation, Flags, Node, StructDoc};

// TODO: Impls for other crates, if feature flags are turned on?

macro_rules! arity {
    ($container: ident, $arity: ident) => {
        impl<T: StructDoc> StructDoc for $container<T> {
            fn document() -> Documentation {
                Documentation(Node::Wrapper {
                    child: Box::new(T::document().0),
                    arity: Arity::$arity,
                    flags: Flags::empty(),
                })
            }
        }
    }
}

arity!(Vec, ManyOrdered);
arity!(LinkedList, ManyOrdered);
arity!(VecDeque, ManyOrdered);
arity!(BinaryHeap, ManyUnordered);
arity!(BTreeSet, ManyUnordered);

macro_rules! array {
    ($($len: expr,)*) => {
        $(
            impl<T: StructDoc> StructDoc for [T; $len] {
                fn document() -> Documentation {
                    Documentation(Node::Wrapper {
                        child: Box::new(T::document().0),
                        arity: Arity::ManyOrdered,
                        flags: Flags::empty(),
                    })
                }
            }
        )*
    }
}

array! {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
}

impl<T: StructDoc, S> StructDoc for HashSet<T, S> {
    fn document() -> Documentation {
        Documentation(Node::Wrapper {
            child: Box::new(T::document().0),
            arity: Arity::ManyUnordered,
            flags: Flags::empty(),
        })
    }
}

impl<T: StructDoc> StructDoc for [T] {
    fn document() -> Documentation {
        Documentation(Node::Wrapper {
            child: Box::new(T::document().0),
            arity: Arity::ManyOrdered,
            flags: Flags::empty(),
        })
    }
}

impl<T: StructDoc> StructDoc for Option<T> {
    fn document() -> Documentation {
        Documentation(Node::Wrapper {
            child: Box::new(T::document().0),
            arity: Arity::One,
            flags: Flags::OPTIONAL,
        })
    }
}

impl<K: StructDoc, V: StructDoc, S> StructDoc for HashMap<K, V, S> {
    fn document() -> Documentation {
        Documentation(Node::Map {
            key: Box::new(K::document().0),
            value: Box::new(K::document().0),
        })
    }
}

impl<K: StructDoc, V: StructDoc> StructDoc for BTreeMap<K, V> {
    fn document() -> Documentation {
        Documentation(Node::Map {
            key: Box::new(K::document().0),
            value: Box::new(K::document().0),
        })
    }
}

macro_rules! transparent {
    ($($ty: ident,)*) => {
        $(
            impl<T: StructDoc> StructDoc for $ty<T> {
                fn document() -> Documentation {
                    T::document()
                }
            }
        )*
    }
}

transparent! {
    Arc,
    Box,
    Cell,
    Mutex,
    // Hmm, is transparent the right thing to do here?
    PhantomData,
    Rc,
    RefCell,
    RwLock,
    Wrapping,
}

impl<T: StructDoc> StructDoc for &T {
    fn document() -> Documentation {
        T::document()
    }
}

impl<T: StructDoc> StructDoc for &mut T {
    fn document() -> Documentation {
        T::document()
    }
}

impl<T: Clone + StructDoc> StructDoc for Cow<'_, T> {
    fn document() -> Documentation {
        T::document()
    }
}

macro_rules! leaf {
    ($($ty: ty,)*) => {
        $(
            impl StructDoc for $ty {
                fn document() -> Documentation {
                    Documentation::leaf()
                }
            }
        )*
    }
}

leaf! {
    u8, u16, u32, u64, u128, usize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
    i8, i16, i32, i64, i128, isize,
    char, str, String, CString, CStr,
    bool, (),
    f32, f64,
    IpAddr, Ipv4Addr, Ipv6Addr,
    SocketAddr, SocketAddrV4, SocketAddrV6,
    Path, PathBuf,
    OsStr, OsString,
    Duration, SystemTime, Instant,
}