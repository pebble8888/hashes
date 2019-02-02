use digest::generic_array::typenum::U64;
use consts::BLAKE2B_IV;

/**
 * $1: $state public な構造体
 * $2: $fix_state 内部に $state を含む固定の public な構造体
 * $3: $word    u64
 * $4: $vec     u64x4
 * $5: &bytes   U64
 * $6: $R1      32 
 * $7: $R2      24
 * $8: $R3      16
 * $9: $R4      63
 * $10: $IV     BLAKE2B_IV
 * $11: $vardoc 変更可能な構造体
 * $12: $doc 固定の出力
 */
blake2_impl!(VarBlake2b, Blake2b,
    u64, u64x4, U64,
    32, 24, 16, 63, BLAKE2B_IV,
    "Blake2b instance with a variable output.",
    "Blake2b instance with a fixed output.",
);
