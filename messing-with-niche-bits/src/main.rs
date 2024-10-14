#![feature(pattern_types, core_pattern_type)]
#![allow(unused, internal_features)]
use std::{
	mem::{align_of, offset_of},
	num::NonZero,
};

type Positive = std::pat::pattern_type!(i32 is -0x7fffffff..0x7fffffff);

#[repr(packed)]
struct B {
	x: u128,
}

#[repr(packed)]
struct A {
	v1: Option<NonZero<u128>>,
	v2: Option<NonZero<u128>>,
	v3: Option<NonZero<u128>>,
	v4: Option<NonZero<u128>>,
}

type T = Positive;

fn main() {
	println!("align = {}", align_of::<T>());
	println!(" size = {}", size_of::<T>());
	println!("    ?   {}", size_of::<Option<T>>());
	println!("   ??   {}", size_of::<Option<Option<T>>>());
	println!("  ???   {}", size_of::<Option<Option<Option<T>>>>());
	println!(" ????   {}", size_of::<Option<Option<Option<Option<T>>>>>());
	let x: i32 = 123;
	let x = NonZero::new(x).unwrap();
	println!("x?.size = {}", size_of_val(&Some(x)));
	println!("x = {}", x);
}
