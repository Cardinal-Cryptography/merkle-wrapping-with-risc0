# merkle-wrapping-with-risc0

Results with `cargo run --release` on `12th Gen Intel® Core™ i7-12800H × 20` with `16GB` RAM:

```
==== Running the Stark pipeline =====
Proving time: 12.488467202s
Verifying time (composite proof): 19.00726ms
Compression time: 12.700139027s
Verifying time (succinct proof): 12.387433ms

Composite proof size: 232kB, Succinct proof size: 217kB

==== Running the Stark+Groth pipeline =====
Proving time (STARK): 13.87643627s
Compression time (composite STARK -> succinct STARK): 12.198505356s
Conversion time (succinct STARK -> recursion proof over Poseidon): 41.245110953s
Wrapping (recursion proof -> Groth16): 25.93166916s
Verifying time (Groth16): 5.419359ms

Recursion proof size: 217kB, Groth16 proof size: 256 bytes
```
