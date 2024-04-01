
pub use crate::*;

pub use core_util::*;
pub use matrix::*;
pub use sys_io::*;
pub use vector::*;

pub use anyhow::{anyhow, bail, Context, Error, Result};
pub use bevy_reflect::Reflect;
pub use bytemuck::{Pod, Zeroable};
pub use eframe::{egui_wgpu, egui_wgpu::CallbackTrait, wgpu, wgpu::util::DeviceExt as _};
pub use egui::{Rect, Ui};
pub use image::{DynamicImage, GenericImageView};
pub use itertools::Itertools;
pub use num_traits::AsPrimitive;
pub use rand::Rng as _;
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use smallvec::SmallVec;
pub type LazyLock<T> = once_cell::sync::Lazy<T>; // TODO: replace by std::sync::LazyLock once stable ().
pub use rand_xoshiro::rand_core::SeedableRng;
pub type Rng = rand_xoshiro::Xoshiro256PlusPlus;
pub use web_time::{Duration, Instant, SystemTime};

// AHash hashmap compatible with bevy_reflect::Reflect.
pub type HashMap<K, V> = std::collections::HashMap<K, V, std::hash::BuildHasherDefault<ahash::AHasher>>;
pub type HashSet<T> = std::collections::HashSet<T, std::hash::BuildHasherDefault<ahash::AHasher>>;

pub type SmallStr = smol_str::SmolStr;

pub use std::cell::{Cell, RefCell};
pub use std::cmp::{PartialEq, PartialOrd};
pub use std::collections::BinaryHeap;
pub use std::f32::consts::PI;
pub use std::fmt;
pub use std::io::{Read, Write};
pub use std::iter;
pub use std::marker::PhantomData;
pub use std::mem;
pub use std::num::{NonZeroU32, NonZeroU8};
pub use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Range, Sub, SubAssign};
pub use std::path::{Path, PathBuf};
pub use std::sync::{Arc, Mutex, OnceLock};

/// 1 degree in radians.
pub const DEG: f32 = PI / 180.0;

#[inline]
pub fn default<T: Default>() -> T {
	T::default()
}
