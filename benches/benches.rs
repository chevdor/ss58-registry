#![feature(bench_black_box)]
#![feature(test)]
extern crate test;
use std::hint::black_box;

use ss58_registry;
use test::Bencher;
static BENCH_SIZE: u16 = 100;
use ss58_registry::{from_address_format, Ss58AddressFormat, Ss58AddressFormatRegistry};
use std::convert::TryInto;

#[bench]
fn new(b: &mut Bencher) {
	b.iter(|| {
		for i in 0..BENCH_SIZE {
			let _ = ss58_registry::Ss58AddressFormat::custom(black_box(i));
		}
	})
}

#[bench]
fn is_custom(b: &mut Bencher) {
	let v: Vec<Ss58AddressFormat> =
		(0..BENCH_SIZE).map(|i| ss58_registry::Ss58AddressFormat::custom(i)).collect();
	b.iter(|| {
		for i in v.iter() {
			let _ = i.is_custom();
		}
	})
}

#[bench]
fn is_reserved(b: &mut Bencher) {
	let v: Vec<Ss58AddressFormat> =
		(0..BENCH_SIZE).map(|i| ss58_registry::Ss58AddressFormat::custom(i)).collect();
	b.iter(|| {
		for i in v.iter() {
			let _ = i.is_reserved();
		}
	})
}

#[bench]
fn to_string(b: &mut Bencher) {
	let v: Vec<Ss58AddressFormat> =
		(0..BENCH_SIZE).map(|i| ss58_registry::Ss58AddressFormat::custom(i)).collect();
	b.iter(|| {
		for i in v.iter() {
			let _ = i.to_string();
		}
	})
}

#[bench]
fn known_to_prefix(b: &mut Bencher) {
	b.iter(|| {
		for i in ss58_registry::Ss58AddressFormat::all() {
			let i: Ss58AddressFormat = (*i).into();
			let _ii: u16 = from_address_format(i);
		}
	})
}

#[bench]
fn name_to_enum(b: &mut Bencher) {
	b.iter(|| {
		for name in ss58_registry::Ss58AddressFormat::all_names() {
			let _: Ss58AddressFormatRegistry = (*name).try_into().expect(&format!("{}", name));
		}
	})
}

#[bench]
fn prefix_to_known(b: &mut Bencher) {
	b.iter(|| {
		for i in 0_u16..100 {
			let i: Ss58AddressFormat = i.into();
			let _: Result<Ss58AddressFormatRegistry, _> = i.try_into();
		}
	})
}
