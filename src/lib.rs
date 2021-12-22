#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Template
 {
	#[init]
	fn init(
		&self,
	) {}

}