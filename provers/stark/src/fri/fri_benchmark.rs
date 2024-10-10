use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::polynomial::Polynomial;
use lambdaworks_math::field::fields::fft_friendly::babybear::Babybear31PrimeField;
//use lambdaworks_math::field::fields::mersenne31::field::Mersenne31Field;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;
use crate::fri::{commit_phase_for_benchmark, query_phase};

#[test]
pub fn fri_benchmark() {
    type F = Babybear31PrimeField;
    type FE = FieldElement<F>;

    for num_layers in 16..21 {
        let n = 1 << num_layers;

        let mut rng = ChaCha8Rng::seed_from_u64(0);

        // Generate a random polynomial. Requires N random field elements.
        let mut coeffs = Vec::with_capacity(n);
        for _ in 0..n {
            coeffs.push(FE::new(rng.gen::<u64>().into()));
        }
        let polynomial = Polynomial::new(&coeffs);

        let mut iotas = Vec::with_capacity(n);

        // Any higher, and we will encounter an out-of-bounds error
        for i in 0..n / 2 {
            iotas.push(i);
        }

        // Not sure what this does
        let coset_offset = FE::from(0);

        let sw = stopwatch::Stopwatch::start_new();
        let (_last_value, fri_layer_list) = commit_phase_for_benchmark(num_layers, polynomial, rng, &coset_offset, n);
        let commit_elapsed = sw.elapsed_ms();
        println!("FRI with a polynomial containing {} BabyBear coefficients (1024 kb) took {}ms", n, commit_elapsed);
        
        let sw = stopwatch::Stopwatch::start_new();
        let _d = query_phase(&fri_layer_list, &iotas);
        let query_elapsed = sw.elapsed_ms();
        println!("query_phase() with {} indices took {}ms", iotas.len(), query_elapsed);
        println!("\n");
    }
}
