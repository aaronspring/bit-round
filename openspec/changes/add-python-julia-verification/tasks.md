## 1. Python Verification Setup
- [ ] 1.1 Create Python virtual environment
- [ ] 1.2 Install numcodecs and dependencies
- [ ] 1.3 Create Python test script for bitround verification
- [ ] 1.4 Generate reference test data (arrays with known values)
- [ ] 1.5 Save Python outputs as reference test data

## 2. Julia Verification Setup
- [ ] 2.1 Install Julia and bitround.jl
- [ ] 2.2 Create Julia test script for bitround verification
- [ ] 2.3 Generate reference test data with Julia
- [ ] 2.4 Save Julia outputs as reference test data

## 3. Rust Verification Tests
- [ ] 3.1 Add integration tests comparing Rust output to Python reference
- [ ] 3.2 Add integration tests comparing Rust output to Julia reference
- [ ] 3.3 Test with various nbits values (1-24 for f32, 1-53 for f64)
- [ ] 3.4 Test with various data patterns (zeros, constants, random, edge values)
- [ ] 3.5 Test NaN, Inf, -Inf handling

## 4. Verification Scripts
- [ ] 4.1 Create `scripts/verify_python.py` for running Python verification
- [ ] 4.2 Create `scripts/verify_julia.jl` for running Julia verification
- [ ] 4.3 Create `scripts/run_all_verification.sh` to run all verifications
- [ ] 4.4 Document verification results
