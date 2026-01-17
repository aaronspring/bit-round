## ADDED Requirements

### Requirement: IEEE Round to Nearest Tie to Even
The system SHALL implement proper IEEE 754 round-to-nearest-tie-to-even for floating-point numbers.

#### Scenario: Scalar IEEE rounding f32
- **WHEN** a scalar f32 value is rounded with nbits=k
- **THEN** the result SHALL follow IEEE 754 round-to-nearest-tie-to-even
- **AND** trailing mantissa bits SHALL be zeroed according to the mask

#### Scenario: Scalar IEEE rounding f64
- **WHEN** a scalar f64 value is rounded with nbits=k
- **THEN** the result SHALL follow IEEE 754 round-to-nearest-tie-to-even
- **AND** trailing mantissa bits SHALL be zeroed according to the mask

#### Scenario: Tie-breaking to even
- **WHEN** a value is exactly halfway between two representable numbers
- **THEN** the result SHALL round to the even mantissa (lsb=0)

### Requirement: IEEE Rounding Parameters
The system SHALL provide functions to compute IEEE rounding parameters.

#### Scenario: Compute shift value
- **WHEN** get_shift is called with keepbits=k
- **THEN** it SHALL return the correct bit shift for the operation
- **AND** handle the special case when keepbits equals significand_bits

#### Scenario: Compute ULP half
- **WHEN** get_ulp_half is called with keepbits=k
- **THEN** it SHALL return half of the unit in last place as an unsigned integer

#### Scenario: Compute keep mask
- **WHEN** get_keep_mask is called with keepbits=k
- **THEN** it SHALL return a mask with 1s for bits to keep and 0s for trailing bits

#### Scenario: Compute bit mask
- **WHEN** get_bit_mask is called with mantissabit=m
- **THEN** it SHALL return a mask with 1 at the specified bit position

### Requirement: Bit Shaving
The system SHALL provide bit shaving functions that round towards zero.

#### Scenario: Shave f32 scalar
- **WHEN** shave is called on an f32 value with keepbits=k
- **THEN** trailing mantissa bits SHALL be set to 0
- **AND** the result SHALL be closer to zero than the input

#### Scenario: Shave f64 scalar
- **WHEN** shave is called on an f64 value with keepbits=k
- **THEN** trailing mantissa bits SHALL be set to 0
- **AND** the result SHALL be closer to zero than the input

#### Scenario: Shave f32 array in-place
- **WHEN** shave_f32_inplace is called on a slice
- **THEN** all elements SHALL be modified in-place
- **AND** each element SHALL have trailing bits set to 0

#### Scenario: Shave f64 array in-place
- **WHEN** shave_f64_inplace is called on a slice
- **THEN** all elements SHALL be modified in-place
- **AND** each element SHALL have trailing bits set to 0

### Requirement: Bit Halfshaving
The system SHALL provide halfshave functions that round to halfway between shave and IEEE round.

#### Scenario: Halfshave f32 scalar
- **WHEN** halfshave is called on an f32 value with keepbits=k
- **THEN** trailing bits SHALL be replaced with 1000... pattern
- **AND** the result SHALL be halfway between shave and IEEE round

#### Scenario: Halfshave f64 scalar
- **WHEN** halfshave is called on an f64 value with keepbits=k
- **THEN** trailing bits SHALL be replaced with 1000... pattern
- **AND** the result SHALL be halfway between shave and IEEE round

#### Scenario: Halfshave f32 array in-place
- **WHEN** halfshave_f32_inplace is called on a slice
- **THEN** all elements SHALL have the halfshave pattern applied

#### Scenario: Halfshave f64 array in-place
- **WHEN** halfshave_f64_inplace is called on a slice
- **THEN** all elements SHALL have the halfshave pattern applied

### Requirement: Bit Setting
The system SHALL provide bit setting functions that round away from zero.

#### Scenario: Set one f32 scalar
- **WHEN** set_one is called on an f32 value with keepbits=k
- **THEN** trailing mantissa bits SHALL be set to 1

#### Scenario: Set one f64 scalar
- **WHEN** set_one is called on an f64 value with keepbits=k
- **THEN** trailing mantissa bits SHALL be set to 1

#### Scenario: Set one f32 array in-place
- **WHEN** set_one_f32_inplace is called on a slice
- **THEN** all elements SHALL have trailing bits set to 1

#### Scenario: Set one f64 array in-place
- **WHEN** set_one_f64_inplace is called on a slice
- **THEN** all elements SHALL have trailing bits set to 1

### Requirement: Bit Grooming
The system SHALL provide bit grooming functions with alternating shave/set pattern.

#### Scenario: Groom f32 array
- **WHEN** groom_f32_inplace is called on an array
- **THEN** elements at even indices SHALL be shaved
- **AND** elements at odd indices SHALL have bits set to 1

#### Scenario: Groom f64 array
- **WHEN** groom_f64_inplace is called on an array
- **THEN** elements at even indices SHALL be shaved
- **AND** elements at odd indices SHALL have bits set to 1

#### Scenario: Groom uneven length array
- **WHEN** grooming an array with odd length
- **THEN** the last element SHALL be shaved

### Requirement: Bit Transposition
The system SHALL provide bit transpose (bit shuffle) functions.

#### Scenario: Transpose f32 bits
- **WHEN** bittranspose is called on an f32 value
- **THEN** the bit order SHALL be reversed (MSB becomes LSB)

#### Scenario: Transpose f64 bits
- **WHEN** bittranspose is called on an f64 value
- **THEN** the bit order SHALL be reversed (MSB becomes LSB)

### Requirement: XOR Delta Transformation
The system SHALL provide XOR delta encoding/decoding functions.

#### Scenario: Encode XOR delta f32
- **WHEN** xor_delta is called on an f32 array
- **THEN** output SHALL be successive XOR differences between adjacent values

#### Scenario: Decode XOR delta f32
- **WHEN** unxor_delta is called on XOR delta encoded data
- **THEN** original values SHALL be restored

#### Scenario: Encode XOR delta f64
- **WHEN** xor_delta is called on an f64 array
- **THEN** output SHALL be successive XOR differences

#### Scenario: Decode XOR delta f64
- **WHEN** unxor_delta is called on XOR delta encoded f64 data
- **THEN** original values SHALL be restored

### Requirement: Exponent Transformations
The system SHALL provide signed and biased exponent extraction functions.

#### Scenario: Signed exponent f32
- **WHEN** signed_exponent is called on an f32 value
- **THEN** output SHALL be the unbiased exponent as signed integer

#### Scenario: Biased exponent f32
- **WHEN** biased_exponent is called on an f32 value
- **THEN** output SHALL be the biased exponent as unsigned integer

#### Scenario: Signed exponent f64
- **WHEN** signed_exponent is called on an f64 value
- **THEN** output SHALL be the unbiased exponent

#### Scenario: Biased exponent f64
- **WHEN** biased_exponent is called on an f64 value
- **THEN** output SHALL be the biased exponent

### Requirement: Bit Counting
The system SHALL provide functions to count bits across arrays.

#### Scenario: Count bits at position f32
- **WHEN** bitcount_u32 is called on an array with bit_position=i
- **THEN** the count SHALL equal the number of 1-bits at that position

#### Scenario: Count all bit positions f32
- **WHEN** bitcount_array_f32 is called on an array
- **THEN** output SHALL be a 32-element array with counts for each bit position

#### Scenario: Count all bit positions f64
- **WHEN** bitcount_array_f64 is called on an array
- **THEN** output SHALL be a 64-element array with counts for each bit position

### Requirement: Bit Count Entropy
The system SHALL provide entropy calculation from bit counts.

#### Scenario: Entropy per bit position f32
- **WHEN** bitcount_entropy_f32 is called on an array
- **THEN** output SHALL contain binary entropy for each bit position
- **AND** entropy SHALL be calculated using the formula H = -p*log(p) - (1-p)*log(1-p)

#### Scenario: Entropy per bit position f64
- **WHEN** bitcount_entropy_f64 is called on an array
- **THEN** output SHALL contain binary entropy for each bit position

### Requirement: Bitpair Counting
The system SHALL provide joint bit counting for adjacent elements.

#### Scenario: Count bitpairs f32
- **WHEN** bitpair_count_f32 is called on two arrays
- **THEN** output SHALL be a 32x2x2 array of joint counts
- **AND** counts SHALL represent 00, 01, 10, 11 combinations at each position

#### Scenario: Count bitpairs f64
- **WHEN** bitpair_count_f64 is called on two arrays
- **THEN** output SHALL be a 64x2x2 array of joint counts

### Requirement: Mutual Information
The system SHALL provide bitwise mutual information calculation.

#### Scenario: Mutual information f32
- **WHEN** mutual_information_f32 is called on two arrays
- **THEN** output SHALL contain mutual information for each bit position
- **AND** calculation SHALL use I(X;Y) = Σ p(x,y) * log(p(x,y) / (p(x)p(y)))

#### Scenario: Mutual information f64
- **WHEN** mutual_information_f64 is called on two arrays
- **THEN** output SHALL contain mutual information for each bit position

### Requirement: Bit Information
The system SHALL provide bitwise information content from adjacent array entries.

#### Scenario: Bit information f32
- **WHEN** bitinformation_f32 is called on an array
- **THEN** output SHALL contain information content from mutual information of adjacent entries

#### Scenario: Bit information f64
- **WHEN** bitinformation_f64 is called on an array
- **THEN** output SHALL contain information content from mutual information of adjacent entries

### Requirement: Redundancy
The system SHALL provide normalized mutual information (redundancy) calculation.

#### Scenario: Redundancy f32
- **WHEN** redundancy_f32 is called on two arrays
- **THEN** output SHALL be normalized mutual information in range [0,1]
- **AND** calculation SHALL use R = 2*I(X;Y) / (H(X) + H(Y))

#### Scenario: Redundancy f64
- **WHEN** redundancy_f64 is called on two arrays
- **THEN** output SHALL be normalized mutual information in range [0,1]

### Requirement: Bit Pattern Entropy
The system SHALL provide entropy from unique bit patterns.

#### Scenario: Bit pattern entropy f32
- **WHEN** bitpattern_entropy_f32 is called on an array
- **THEN** output SHALL be the Shannon entropy of unique bit patterns
- **AND** array SHALL be sorted in-place for counting

#### Scenario: Bit pattern entropy f64
- **WHEN** bitpattern_entropy_f64 is called on an array
- **THEN** output SHALL be the Shannon entropy of unique bit patterns

### Requirement: Statistical Functions
The system SHALL provide binomial confidence and entropy functions.

#### Scenario: Binomial confidence interval
- **WHEN** binom_confidence is called with n trials and confidence level c
- **THEN** output SHALL be the probability p for p=0.5 with that confidence

#### Scenario: Binomial free entropy
- **WHEN** binom_free_entropy is called with n, confidence, and base
- **THEN** output SHALL be the free entropy 1 - H(p)

#### Scenario: Set insignificant to zero
- **WHEN** set_zero_insignificant is called on entropy array
- **THEN** values below free entropy threshold SHALL be set to zero

### Requirement: Bitround Encoding
The system SHALL provide a bitround encoder that reduces the bit width of floating-point arrays.

#### Scenario: Encode f32 array with 16 bits
- **WHEN** an f32 array is encoded with nbits=16
- **THEN** the output SHALL use exactly 16 bits per value
- **AND** decoding SHALL recover values within numerical tolerance

#### Scenario: Encode f64 array with 32 bits
- **WHEN** an f64 array is encoded with nbits=32
- **THEN** the output SHALL use exactly 32 bits per value
- **AND** decoding SHALL recover values within numerical tolerance

### Requirement: Bitround Decoding
The system SHALL provide a bitround decoder that restores original precision.

#### Scenario: Roundtrip f32 preserves values
- **WHEN** an f32 array is encoded then decoded
- **THEN** the decoded values SHALL match the original within 2^-(nbits-1) relative error
- **AND** the output array SHALL have the same shape as the input

#### Scenario: Roundtrip f64 preserves values
- **WHEN** an f64 array is encoded then decoded
- **THEN** the decoded values SHALL match the original within 2^-(nbits-1) relative error
- **AND** the output array SHALL have the same shape as the input

### Requirement: Configurable Bit Width
The encoder SHALL accept a configurable `nbits` parameter controlling output precision.

#### Scenario: Reject invalid nbits for f32
- **WHEN** nbits is greater than 24 for f32 input
- **THEN** the encoder SHALL return an error

#### Scenario: Reject invalid nbits for f64
- **WHEN** nbits is greater than 53 for f64 input
- **THEN** the encoder SHALL return an error

#### Scenario: Accept valid nbits range
- **WHEN** nbits is between 1 and 24 (f32) or 1 and 53 (f64)
- **THEN** the encoder SHALL accept the value and produce output

### Requirement: Cross-Implementation Verification
The Rust implementation SHALL produce identical output to reference implementations.

#### Scenario: Match Julia BitInformation.jl output
- **WHEN** the same input array and nbits are used
- **THEN** the Rust encoder SHALL produce bit-identical output to Julia BitInformation.jl

#### Scenario: Match Python numcodecs output
- **WHEN** the same input array and nbits are used
- **THEN** the Rust encoder SHALL produce bit-identical output to Python numcodecs bitround

### Requirement: Performance Comparison
The system SHALL provide benchmarks comparing Rust performance to reference implementations.

#### Scenario: Benchmark IEEE rounding operations
- **WHEN** running the benchmark suite
- **THEN** results SHALL report Rust vs Julia throughput for rounding operations

#### Scenario: Benchmark information functions
- **WHEN** running the benchmark suite
- **THEN** results SHALL report Rust vs Julia throughput for bitcount, mutual information, etc.

#### Scenario: Benchmark bit transformations
- **WHEN** running the benchmark suite
- **THEN** results SHALL report Rust vs Julia throughput for xor_delta, bittranspose, etc.
