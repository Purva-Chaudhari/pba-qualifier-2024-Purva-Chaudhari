// First, we are going to introduce some units of energy. For whatever reason, we prefer BTU above
// Joules and Calories, but we want to support all 3 of these in this module. Double check the
// conversion methods, and make sure you fully understand them.

use std::marker::PhantomData;

// You may uncomment and use the following import if you need it. You may also read its
// documentation at https://doc.rust-lang.org/std/cell/struct.RefCell.html
use std::cell::RefCell;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Joule(pub u32);
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Calorie(pub u32);

pub type BTU = u32;

impl From<Joule> for BTU {
	fn from(j: Joule) -> Self {
		j.0 / 1055
	}
}

impl From<BTU> for Joule {
	fn from(b: BTU) -> Self {
		Self(b * 1055)
	}
}

impl From<Calorie> for BTU {
	fn from(c: Calorie) -> Self {
		c.0 / 251
	}
}

impl From<BTU> for Calorie {
	fn from(b: BTU) -> Self {
		Calorie(b * 251)
	}
}

// Now, we start defining some types of fuel.

/// A technology for storing energy for later consumption.
pub trait Fuel {
	/// The output unit of the energy density.
	///
	/// Think about this: why did we chose this to be an associated type rather than a generic?
	type Output: Into<BTU> + From<BTU>;

	/// The amount of energy contained in a single unit of fuel.
	fn energy_density() -> Self::Output;
}

pub struct Diesel;
impl Fuel for Diesel {
	type Output = Joule;
	fn energy_density() -> Self::Output {
		Joule::from(100 as BTU)
	}
}

pub struct LithiumBattery;
impl Fuel for LithiumBattery {
	type Output = Calorie;
	fn energy_density() -> Self::Output {
		Calorie::from(200 as BTU)
	}
}

pub struct Uranium;
impl Fuel for Uranium {
	type Output = Joule;
	fn energy_density() -> Self::Output {
		Joule::from(1000 as BTU)
	}
}

/// A container for any fuel type.
pub struct FuelContainer<F: Fuel> {
	/// The amount of fuel.
	amount: u32,
	/// NOTE: Fuel doesn't really have any methods that require `&self` on it,
	/// so any information that we can get, we can get from `F` as **TYPE**, we don't really need
	/// to store an instance of `F`, like `fuel: F` as a struct field. But to satisfy the compiler,
	/// we must use `F` somewhere.
	/// Thus, this is the perfect use case of `PhantomData`.
	_marker: PhantomData<F>,
}

impl<F: Fuel> FuelContainer<F> {
	pub fn new(amount: u32) -> Self {
		Self {
			amount,
			_marker: Default::default(),
		}
	}
}

/// Something that can provide energy from a given `F` fuel type, like a power-plant.
pub trait ProvideEnergy<F: Fuel> {
	/// Consume the fuel container and return the created energy, based on the power density of the
	/// fuel and potentially other factors.
	///
	/// Some fuel providers might have some kind of decay or inefficiency, which should be reflected
	/// here. Otherwise, [ProvideEnergy::provide_energy_with_efficiency] or
	/// [ProvideEnergy::provide_energy_ideal] might be good enough.
	///
	/// Not all `ProvideEnergy` implementations need to have internal state. Therefore, this
	/// interface accepts `&self`, not `&mut self`. You might need to use special language features
	/// to overcome this.
	fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output;

	/// Convert the amount of fuel in `f` with an exact efficiency of `e`.
	///
	/// NOTE: all efficiencies are interpreted as u8 values that can be at most 100, and represent a
	/// percent. If an efficiency above 100 is supplied, the code should treat it as 100. That is to
	/// say that the efficiency is "saturating" at 100%.
	///
	/// This method must be provided as it will be the same in all implementations.
	fn provide_energy_with_efficiency(&self, f: FuelContainer<F>, e: u8) -> <F as Fuel>::Output {
		let efficiency = e.min(100) as f32 / 100.0;  // Convert to percentage and clamp to 100%
        let energy_density = F::energy_density();    // Get energy density from the Fuel trait
        let total_energy = BTU::from(energy_density.into()) * f.amount;  // Total energy in BTUs
        let adjusted_energy = (total_energy as f32 * efficiency).round() as u32; // Adjust for efficiency and round off

        // Convert back to the fuel's output unit
        <F as Fuel>::Output::from(adjusted_energy)
	}

	/// Same as [`ProvideEnergy::provide_energy_with_efficiency`], but with an efficiency of 100.
	///
	/// This method must be provided as it will be the same in all implementations.
	fn provide_energy_ideal(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
		self.provide_energy_with_efficiency(f, 100) 
	}
}

/// A nuclear reactor that can only consume `Uranium` and provide energy with 99% efficiency.
pub struct NuclearReactor;
impl ProvideEnergy<Uranium> for NuclearReactor {
	fn provide_energy(&self, f: FuelContainer<Uranium>) -> <Uranium as Fuel>::Output {
		//todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
		let efficiency = 99; // 99% efficiency for the nuclear reactor
        self.provide_energy_with_efficiency(f, efficiency)
	}
}

/// A combustion engine that can only consume `Diesel`.
///
/// The `DECAY` const must be interpreted as such: per every `DECAY` times `provide_energy` is
/// called on an instance of this type, the efficiency should reduce by one. The initial efficiency
/// must be configurable with a `fn new(efficiency: u8) -> Self`.
pub struct InternalCombustion<const DECAY: u32>{/* Fill the fields as needed */
	efficiency: RefCell<u8>,
    call_count: RefCell<u32>,
}

impl<const DECAY: u32> InternalCombustion<DECAY> {
	pub fn new(efficiency: u8) -> Self {
		InternalCombustion {
            efficiency: RefCell::new(efficiency.min(100)),
            call_count: RefCell::new(0),
        }
	}
}

impl<const DECAY: u32> ProvideEnergy<Diesel> for InternalCombustion<DECAY> {
	fn provide_energy(&self, f: FuelContainer<Diesel>) -> <Diesel as Fuel>::Output {
		//todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
		let mut current_count = self.call_count.borrow_mut();
		let mut current_efficiency = self.efficiency.borrow_mut();
		//println!("{}", *current_efficiency);

        if *current_count % DECAY == 0 && *current_efficiency > 1 && *current_count>=DECAY{
                *current_efficiency -= 1;
				//println!("Inside {}", *current_efficiency);
            
        }
		*current_count += 1;
        
        self.provide_energy_with_efficiency(f, *current_efficiency)
	}
}

/// A hypothetical device that can, unlike the `InternalCombustion`, consume **any fuel** that's of
/// type `trait Fuel`. It can provide a fixed efficiency regardless of fuel type. As before,
/// EFFICIENCY is a u8 whose value should not exceed 100, is interpreted as a percent, and should
/// saturate at 100% when a higher value is supplied.
pub struct OmniGenerator<const EFFICIENCY: u8>;

// NOTE: implement `ProvideEnergy` for `OmniGenerator` using only one `impl` block.
impl<const EFFICIENCY: u8, F: Fuel> ProvideEnergy<F> for OmniGenerator<EFFICIENCY> {
	fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
		//todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
		self.provide_energy_with_efficiency(f, EFFICIENCY)
	}
}

/// A type that can wrap two different fuel types and mix them together.
///
/// The energy density of the new fuel type is the average of the two given, once converted to BTU.
/// The output unit should also be BTU.
///
/// This can represent a new fuel type, thus it must implement `Fuel`.
pub struct Mixed<F1: Fuel, F2: Fuel>(PhantomData<(F1, F2)>);

impl<F1: Fuel, F2: Fuel> Fuel for Mixed<F1, F2> {
	type Output = BTU;

	fn energy_density() -> Self::Output {
		//todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
		let energy_density1 = BTU::from(F1::energy_density().into());
        let energy_density2 = BTU::from(F2::energy_density().into());

        (energy_density1 + energy_density2) / 2
	}
}

// Now think about how you can make the mixer configurable, such that it would produce a new fuel
// with an energy density that is more influences by one type than the other.
//
// For example, you have a mixer of F1, F2, and some coefficient C1, where the energy density of the
// mixture is `F1 * C1 + F2 * (1 - C1) )` where `C1` is a ratio (which you have to represent again
// with a u8 percent).
//
// The main trick is to overcome the fact that `fn energy_density` does not take in a `self`, so the
// coefficients need to be incorporated in some other way (you've already seen examples of that in
// this file ;)).
pub struct CustomMixed<const C: u8, F1, F2>(PhantomData<(F1, F2)>);
impl<const C: u8, F1: Fuel, F2: Fuel> Fuel for CustomMixed<C, F1, F2> {
	type Output = BTU;

	fn energy_density() -> Self::Output {
		assert!(C <= 100, "C is not between 0 and 100");

        let ratio = C as f32 / 100.0;
        let inverse_ratio = 1.0 - ratio;

        let energy_density1 = BTU::from(F1::energy_density().into()) as f32 * ratio;
        let energy_density2 = BTU::from(F2::energy_density().into()) as f32 * inverse_ratio;

        (energy_density1 + energy_density2) as u32
	}
}

// Now, any of our existing energy providers can be used with a mix fuel.

/// A function that returns the energy produced by the `OmniGenerator` with efficiency of 80%, when
/// the fuel type is an even a mix of `Diesel` as `LithiumBattery`;
pub fn omni_80_energy(amount: u32) -> BTU {
	let fuel_container: FuelContainer<Mixed<Diesel, LithiumBattery>> = FuelContainer::new(amount);
	let og = OmniGenerator::<80>;
	og.provide_energy(fuel_container).into()
}

// Finally, let's consider marker traits, and some trait bounds.

/// Some traits are just markers. They don't bring any additional functionality anything, other than
/// marking a type with some trait.
pub trait IsRenewable {}
impl IsRenewable for LithiumBattery {}

/// Define the following struct such that it only provides energy if the fuel is `IsRenewable`.
///
/// It has perfect efficiency.
pub struct GreenEngine<F: Fuel + IsRenewable>(pub PhantomData<F>);
impl<F: Fuel + IsRenewable> ProvideEnergy<F> for GreenEngine<F> {
	fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
		//todo!("complete the implementation; no te that you might need to change the trait bounds and generics of the `impl` line");
		self.provide_energy_ideal(f)
	}
}

/// Define the following struct such that it only provides energy if the fuel's output type is
/// `BTU`.
///
/// It has perfect efficiency.
pub struct BritishEngine<F: Fuel<Output = BTU>>(pub PhantomData<F>);
impl<F: Fuel<Output = BTU>> ProvideEnergy<F> for BritishEngine<F> {
	fn provide_energy(&self, f: FuelContainer<F>) -> <F as Fuel>::Output {
		//todo!("complete the implementation; note that you might need to change the trait bounds and generics of the `impl` line");
		self.provide_energy_ideal(f)
	}
}

// Congratulations! you have finished the advance trait section.
//
// Disclaimer: the types and traits that you are asked to implement in this module are by no means
// designed to be sensible. Instead, they are chosen to represent a typical, often difficult,
// pattern. Some are intentionally slightly convoluted to challenge you :). I am sure if we actually
// wanted to design a fuel system, we would do better.

/// This function is not graded. It is just for collecting feedback.
/// On a scale from 0 - 255, with zero being extremely easy and 255 being extremely hard,
/// how hard did you find this section of the exam.
pub fn how_hard_was_this_section() -> u8 {
	3
}

/// This function is not graded. It is just for collecting feedback.
/// How much time (in hours) did you spend on this section of the exam?
pub fn how_many_hours_did_you_spend_on_this_section() -> u8 {
	215
}

#[cfg(test)]
mod tests {
	use super::*;

	trait ToBTU {
		fn to_btu(self) -> BTU;
	}

	impl<T: Into<BTU>> ToBTU for T {
		fn to_btu(self) -> BTU {
			self.into()
		}
	}

	#[test]
	fn nuclear() {
		let nr = NuclearReactor;
		assert_eq!(
			nr.provide_energy(FuelContainer::<Uranium>::new(10))
				.to_btu(),
			9900
		);
		assert_eq!(
			nr.provide_energy(FuelContainer::<Uranium>::new(10))
				.to_btu(),
			9900
		);
	}

	#[test]
	fn ic_1() {
		let ic = InternalCombustion::<3>::new(120);
		assert_eq!(
			ic.provide_energy(FuelContainer::<Diesel>::new(10)).to_btu(),
			1000
		);
		assert_eq!(
			ic.provide_energy(FuelContainer::<Diesel>::new(10)).to_btu(),
			1000
		);
		assert_eq!(
			ic.provide_energy(FuelContainer::<Diesel>::new(10)).to_btu(),
			1000
		);
		assert_eq!(
			ic.provide_energy(FuelContainer::<Diesel>::new(10)).to_btu(),
			990
		);
	}

	#[test]
	fn omni_1() {
		let og = OmniGenerator::<100>;
		assert_eq!(
			og.provide_energy(FuelContainer::<Uranium>::new(10))
				.to_btu(),
			10000
		);
		assert_eq!(
			og.provide_energy(FuelContainer::<Diesel>::new(10)).to_btu(),
			1000
		);
		assert_eq!(
			og.provide_energy(FuelContainer::<LithiumBattery>::new(10))
				.to_btu(),
			2000
		);
	}

	#[test]
	fn mixed_1() {
		assert_eq!(
			Mixed::<Diesel, LithiumBattery>::energy_density().to_btu(),
			150
		);
	}

	#[test]
	fn custom_mixed_1() {
		// custom with 50 is the same as Mixed.
		assert_eq!(
			CustomMixed::<50, Diesel, LithiumBattery>::energy_density().to_btu(),
			Mixed::<Diesel, LithiumBattery>::energy_density()
		);
	}

	#[test]
	fn omni_80_1() {
		let amount = 10; 
		let expected_btu_output = 1200; 
		let btu_output = omni_80_energy(amount);
		assert_eq!(btu_output, expected_btu_output)
	}
}
