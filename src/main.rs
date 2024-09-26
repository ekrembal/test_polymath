use ark_bn254::{Bn254, Fr};
use ark_circom::CircomBuilder;
use ark_circom::CircomConfig;
use ark_crypto_primitives::snark::CircuitSpecificSetupSNARK;
use ark_crypto_primitives::snark::SNARK;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, Field, SynthesisError},
};
use ark_std::{
    rand::{Rng, RngCore, SeedableRng},
    test_rng,
};
use num_bigint::BigInt;
use sigma0_polymath::merlin::MerlinFieldTranscript;
// use num_bigint::bigint::BigInt;

type Polymath = sigma0_polymath::Polymath<Bn254, MerlinFieldTranscript<Fr>>;

struct ExampleCircuit<F: Field> {
    a: Option<F>,
    b: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for ExampleCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;

        let c = self.a.map(|a| self.b.map(|b| a * b)).flatten();
        let c = cs.new_input_variable(|| c.ok_or(SynthesisError::AssignmentMissing))?;

        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)
    }
}
fn main() {
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());

    let cfg = CircomConfig::<Bn254>::new(
        "/Users/ekrembal/Developer/chainway/bitvm_experiments/test_polymath/mycircuit.wasm",
        "/Users/ekrembal/Developer/chainway/bitvm_experiments/test_polymath/mycircuit.r1cs",
    )
    .unwrap();

    // // Insert our public inputs as key value pairs
    let mut builder = CircomBuilder::new(cfg);


    // // Create an empty instance for setting it up
    let circom_circuit = builder.setup();
    let a:Fr = Fr::from(5);
    let b:Fr = Fr::from(6);


    builder.push_input("a", 5);
    builder.push_input("b", 6);

    let circom = builder.build().unwrap();

    let (pk, vk) = Polymath::setup(circom_circuit, &mut rng).unwrap();


    let product = a*b; // this is the public input

    let proof = Polymath::prove(&pk, circom, &mut rng).unwrap();
    println!("Proof generated");
    println!("Proof: {:?}", proof);
    assert!(Polymath::verify(&vk, &[Fr::from(30)], &proof).unwrap());
    assert!(!Polymath::verify(&vk, &[Fr::from(31)], &proof).is_err());
}
