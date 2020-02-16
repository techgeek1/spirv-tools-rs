//! Raw bingings for Spirv-Tools API

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_char, size_t};

macro_rules! spv_bit {
    ($index: literal) => {
        1 << $index
    };
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct spv_result_t(i32);

impl spv_result_t {
    pub const SUCCESS: Self = Self(0);
    pub const UNSUPPORTED: Self = Self(1);
    pub const END_OF_STREAM: Self = Self(2);
    pub const WARNING: Self = Self(3);
    pub const FAILED_MATCH: Self = Self(4);
    pub const REQUESTED_TERMINATION: Self = Self(5);  // Success, but signals early termination);
    pub const ERROR_INTERNAL: Self = Self(-1);
    pub const ERROR_OUT_OF_MEMORY: Self = Self(-2);
    pub const ERROR_INVALID_POINTER: Self = Self(-3);
    pub const ERROR_INVALID_BINARY: Self = Self(-4);
    pub const ERROR_INVALID_TEXT: Self = Self(-5);
    pub const ERROR_INVALID_TABLE: Self = Self(-6);
    pub const ERROR_INVALID_VALUE: Self = Self(-7);
    pub const ERROR_INVALID_DIAGNOSTIC: Self = Self(-8);
    pub const ERROR_INVALID_LOOKUP: Self = Self(-9);
    pub const ERROR_INVALID_ID: Self = Self(-10);
    pub const ERROR_INVALID_CFG: Self = Self(-11);
    pub const ERROR_INVALID_LAYOUT: Self = Self(-12);
    pub const ERROR_INVALID_CAPABILITY: Self = Self(-13);
    pub const ERROR_INVALID_DATA: Self = Self(-14);  // Indicates data rules validation failure);
    pub const ERROR_MISSING_EXTENSION: Self = Self(-15);
    pub const ERROR_WRONG_VERSION: Self = Self(-16);  // Indicates wrong SPIR-V versio);
}

/// Severity levels of messages communicated to the consumer.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct spv_message_level_t(u32);

impl spv_message_level_t {
    /// Unrecoverable error due to environment.
    /// Will exit the program immediately. E.g.,
    /// out of memory.
    pub const FATAL: Self = Self(0);
    /// Unrecoverable error due to SPIRV-Tools
    /// internals.
    /// Will exit the program immediately. E.g.,
    /// unimplemented feature. 
    pub const INTERNAL_ERROR: Self = Self(1);
    /// Normal error due to user input.
    pub const ERROR: Self = Self(2);
    /// Warning information.
    pub const WARNING: Self = Self(3);
    /// General information.
    pub const INFO: Self = Self(4);
    /// Debug information.
    pub const DEBUG: Self = Self(5);
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct spv_endianness_t(u32);

impl spv_endianness_t {
    pub const LITTLE: Self = Self(1);
    pub const BIG: Self = Self(2);
}

/// The kinds of operands that an instruction may have.
///
/// Some operand types are "concrete".  The binary parser uses a concrete
/// operand type to describe an operand of a parsed instruction.
///
/// The assembler uses all operand types.  In addition to determining what
/// kind of value an operand may be, non-concrete operand types capture the
/// fact that an operand might be optional (may be absent, or present exactly
/// once), or might occur zero or more times.
///
/// Sometimes we also need to be able to express the fact that an operand
/// is a member of an optional tuple of values.  In that case the first member
/// would be optional, and the subsequent members would be required.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct spv_operand_type_t(u32);

impl spv_operand_type_t {
    /// A sentinel value.
    pub const NONE: Self = Self(0);

    /// Set 1:  Operands that are IDs.
    pub const ID: Self = Self(1);
    pub const TYPE_ID: Self = Self(2);
    pub const RESULT_ID: Self = Self(3);
    /// SPIR-V Sec 3.25
    pub const MEMORY_SEMANTICS_ID: Self = Self(4);
    /// SPIR-V Sec 3.27
    pub const SCOPE_ID: Self = Self(5);

    /// Set 2:  Operands that are literal numbers.
    pub const LITERAL_INTEGER: Self = Self(6);  // Always unsigned 32-bits.
    /// The Instruction argument to OpExtInst. It's an unsigned 32-bit literal
    /// number indicating which instruction to use from an extended instruction
    /// set.
    pub const EXTENSION_INSTRUCTION_NUMBER: Self = Self(7);
    /// The Opcode argument to OpSpecConstantOp. It determines the operation
    /// to be performed on constant operands to compute a specialization constant
    /// result.
    pub const SPEC_CONSTANT_OP_NUMBER: Self = Self(8);
    /// A literal number whose format and size are determined by a previous operand
    /// in the same instruction.  It's a signed integer, an unsigned integer, or a
    /// floating point number.  It also has a specified bit width.  The width
    /// may be larger than 32, which would require such a typed literal value to
    /// occupy multiple SPIR-V words.
    pub const TYPED_LITERAL_NUMBER: Self = Self(9);

    /// Set 3:  The literal string operand type.
    pub const LITERAL_STRING: Self = Self(10);

    /// Set 4:  Operands that are a single word enumerated value.
    /// SPIR-V Sec 3.2
    pub const SOURCE_LANGUAGE: Self = Self(11);
    /// SPIR-V Sec 3.3
    pub const EXECUTION_MODEL: Self = Self(12);
    /// SPIR-V Sec 3.4
    pub const ADDRESSING_MODEL: Self = Self(13);
    /// SPIR-V Sec 3.5
    pub const MEMORY_MODEL: Self = Self(14);
    /// SPIR-V Sec 3.6
    pub const EXECUTION_MODE: Self = Self(15);
    /// SPIR-V Sec 3.7
    pub const STORAGE_CLASS: Self = Self(16);
    /// SPIR-V Sec 3.8
    pub const DIMENSIONALITY: Self = Self(17);
    /// SPIR-V Sec 3.9
    pub const SAMPLER_ADDRESSING_MODE: Self = Self(18);
    /// SPIR-V Sec 3.10
    pub const SAMPLER_FILTER_MODE: Self = Self(19);
    /// SPIR-V Sec 3.11
    pub const SAMPLER_IMAGE_FORMAT: Self = Self(20);
    /// SPIR-V Sec 3.12
    pub const IMAGE_CHANNEL_ORDER: Self = Self(21);
    /// SPIR-V Sec 3.13
    pub const IMAGE_CHANNEL_DATA_TYPE: Self = Self(22);
    /// SPIR-V Sec 3.16
    pub const FP_ROUNDING_MODE: Self = Self(23);
    /// SPIR-V Sec 3.17
    pub const LINKAGE_TYPE: Self = Self(24);
    /// SPIR-V Sec 3.18
    pub const ACCESS_QUALIFIER: Self = Self(25);
    /// SPIR-V Sec 3.19
    pub const FUNCTION_PARAMETER_ATTRIBUTE: Self = Self(26);
    /// SPIR-V Sec 3.20
    pub const DECORATION: Self = Self(27);
    /// SPIR-V Sec 3.21
    pub const BUILT_IN: Self = Self(28);
    /// SPIR-V Sec 3.28
    pub const GROUP_OPERATION: Self = Self(29);
    /// SPIR-V Sec 3.29
    pub const KERNEL_ENQ_FLAGS: Self = Self(30);
    /// SPIR-V Sec 3.30
    pub const KERNEL_PROFILING_INFO: Self = Self(31);
    /// SPIR-V Sec 3.31
    pub const CAPABILITY: Self = Self(32);

    /// Set 5:  Operands that are a single word bitmask.
    /// Sometimes a set bit indicates the instruction requires still more operands.
    /// SPIR-V Sec 3.14
    pub const IMAGE: Self = Self(33);
    /// SPIR-V Sec 3.15
    pub const FP_FAST_MATH_MODE: Self = Self(34);
    /// SPIR-V Sec 3.22
    pub const SELECTION_CONTROL: Self = Self(35);
    /// SPIR-V Sec 3.23
    pub const LOOP_CONTROL: Self = Self(36);
    /// SPIR-V Sec 3.24
    pub const FUNCTION_CONTROL: Self = Self(37);
    /// SPIR-V Sec 3.26
    pub const MEMORY_ACCESS: Self = Self(38);

    // The remaining operand types are only used internally by the assembler.
    // There are two categories:
    //    Optional : expands to 0 or 1 operand, like ? in regular expressions.
    //    Variable : expands to 0, 1 or many operands or pairs of operands.
    //               This is similar to * in regular expressions.

    /// An optional operand represents zero or one logical operands.
    /// In an instruction definition, this may only appear at the end of the
    /// operand types.
    pub const OPTIONAL_ID: Self = Self(39); // Manually expanded from original due to hygenic macros
    pub const FIRST_OPTIONAL_TYPE: Self = Self::OPTIONAL_ID;
  
    /// An optional image operand type.
    pub const OPTIONAL_IMAGE: Self = Self(40);
    /// An optional memory access type.
    pub const OPTIONAL_MEMORY_ACCESS: Self = Self(41);
    /// An optional literal integer.
    pub const OPTIONAL_LITERAL_INTEGER: Self = Self(42);
    /// An optional literal number, which may be either integer or floating point.
    pub const OPTIONAL_LITERAL_NUMBER: Self = Self(43);
    /// Like TYPED_LITERAL_NUMBER, but optional, and integral.
    pub const OPTIONAL_TYPED_LITERAL_INTEGER: Self = Self(44);
    /// An optional literal string.
    pub const OPTIONAL_LITERAL_STRING: Self = Self(45);
    /// An optional access qualifier
    pub const OPTIONAL_ACCESS_QUALIFIER: Self = Self(46);
    /// An optional context-independent value, or CIV.  CIVs are tokens that we can
    /// assemble regardless of where they occur -- literals, IDs, immediate
    /// integers, etc.
    pub const OPTIONAL_CIV: Self = Self(46);

    /// A variable operand represents zero or more logical operands.
    /// In an instruction definition, this may only appear at the end of the
    /// operand types.
    pub const VARIABLE_ID: Self = Self(47); // Manually expanded from original due to hygenic macros
    pub const FIRST_VARIABLE_TYPE: Self = Self::VARIABLE_ID;
    pub const VARIABLE_LITERAL_INTEGER: Self = Self(48);
    /// A sequence of zero or more pairs of (typed literal integer, Id).
    /// Expands to zero or more:
    ///  (TYPED_LITERAL_INTEGER, ID)
    /// where the literal number must always be an integer of some sort.
    pub const VARIABLE_LITERAL_INTEGER_ID: Self = Self(49);
    /// A sequence of zero or more pairs of (Id, Literal integer)
    pub const VARIABLE_ID_LITERAL_INTEGER: Self = Self(50); // Manually expanded from original due to hygenic macros
    pub const LAST_VARIABLE_TYPE: Self = Self::VARIABLE_ID_LITERAL_INTEGER;
    pub const LAST_OPTIONAL_TYPE: Self = Self::VARIABLE_ID_LITERAL_INTEGER;

    /// The following are concrete enum types.
    pub const DEBUG_INFO_FLAGS: Self = Self(51);  // DebugInfo Sec 3.2.  A mask.
    /// DebugInfo Sec 3.3
    pub const DEBUG_BASE_TYPE_ATTRIBUTE_ENCODING: Self = Self(52);
    /// DebugInfo Sec 3.4
    pub const DEBUG_COMPOSITE_TYPE: Self = Self(53);
    /// DebugInfo Sec 3.5
    pub const DEBUG_TYPE_QUALIFIER: Self = Self(54);
    /// DebugInfo Sec 3.6
    pub const DEBUG_OPERATION: Self = Self(55);

    /// This is a sentinel value, and does not represent an operand type.
    /// It should come last.
    pub const NUM_OPERAND_TYPES: Self = Self(56);
}

#[repr(C)]
pub struct spv_ext_inst_type_t(u32);

impl spv_ext_inst_type_t {
    pub const NONE: Self = Self(0);
    pub const GLSL_STD_450: Self = Self(1);
    pub const OPENCL_STD: Self = Self(2);
    pub const SPV_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER: Self = Self(3);
    pub const SPV_AMD_SHADER_TRINARY_MINMAX: Self = Self(4);
    pub const SPV_AMD_GCN_SHADER: Self = Self(5);
    pub const SPV_AMD_SHADER_BALLOT: Self = Self(6);
    pub const DEBUGINFO: Self = Self(7);
}

/// This determines at a high level the kind of a binary-encoded literal
/// number, but not the bit width.
/// In principle, these could probably be folded into new entries in
/// spv_operand_type_t.  But then we'd have some special case differences
/// between the assembler and disassembler
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct spv_number_kind_t(u32);

impl spv_number_kind_t {
    /// The default for value initialization.
    pub const NONE: Self = Self(0);
    pub const UNSIGNED_INT: Self = Self(1);
    pub const SIGNED_INT: Self = Self(2);
    pub const FLOATING: Self = Self(3);
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct spv_text_to_binary_options_t(u32);

impl spv_text_to_binary_options_t {
    pub const NONE: Self = Self(spv_bit!(0));
    /// Numeric IDs in the binary will have the same values as in the source.
    /// Non-numeric IDs are allocated by filling in the gaps, starting with 1
    /// and going up.
    pub const PRESERVE_NUMERIC_IDS: Self = Self(spv_bit!(1));
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct spv_binary_to_text_options_t(u32);

impl spv_binary_to_text_options_t {
    pub const NONE: Self = Self(spv_bit!(0));
    pub const PRINT: Self = Self(spv_bit!(1));
    pub const COLOR: Self = Self(spv_bit!(2));
    pub const INDENT: Self = Self(spv_bit!(3));
    pub const SHOW_BYTE_OFFSET: Self = Self(spv_bit!(4));
    /// Do not output the module header as leading comments in the assembly.
    pub const NO_HEADER: Self = Self(spv_bit!(5));
    /// Use friendly names where possible.  The heuristic may expand over
    /// time, but will use common names for scalar types, and debug names from
    /// OpName instructions.
    pub const FRIENDLY_NAMES: Self = Self(spv_bit!(6));
}

// The default id bound is to the minimum value for the id limit
// in the spir-v specification under the section "Universal Limits".
pub const kDefaultMaxIdBound: u32 = 0x3FFFFF;


/// Information about an operand parsed from a binary SPIR-V module.
/// Note that the values are not included.  You still need access to the binary
/// to extract the values.
#[repr(C)]
pub struct spv_parsed_operand_t {
    /// Location of the operand, in words from the start of the instruction.
    pub offset: u16,
    /// Number of words occupied by this operand.
    pub num_words: u16,
    /// The "concrete" operand type.  See the definition of spv_operand_type_t
    /// for details.
    pub ty: spv_operand_type_t,
    /// If type is a literal number type, then number_kind says whether it's
    /// a signed integer, an unsigned integer, or a floating point number.
    pub number_kind: spv_number_kind_t,
    /// The number of bits for a literal number type.
    pub number_bit_width: u32
}

/// An instruction parsed from a binary SPIR-V module.
#[repr(C)]
pub struct spv_parsed_instruction_t {
    /// An array of words for this instruction, in native endianness.
    pub words: *const u32,
    /// The number of words in this instruction.
    pub num_words: u16,
    pub opcode: u16,
    /// The extended instruction type, if opcode is OpExtInst.  Otherwise
    /// this is the "none" value.
    pub ext_inst_type: spv_ext_inst_type_t,
    /// The type id, or 0 if this instruction doesn't have one.
    pub type_id: u32,
    /// The result id, or 0 if this instruction doesn't have one.
    pub result_id: u32,
    /// The array of parsed operands.
    pub operands: *const spv_parsed_operand_t,
    pub num_operands: u16
}

#[repr(C)]
pub struct spv_const_binary_t {
    pub code: *const u32,
    pub word_count: size_t
}

#[repr(C)]
pub struct spv_binary_t {
    pub code: *mut u32,
    pub word_count: size_t
}

#[repr(C)]
pub struct spv_text_t {
    pub string: *const c_char,
    pub length: size_t
}

#[repr(C)]
pub struct spv_position_t {
    pub line: size_t,
    pub column: size_t,
    pub index: size_t
}

#[repr(C)]
pub struct spv_diagnostic_t {
    pub position: spv_position_t,
    pub error: *mut c_char,
    pub is_text_source: bool
}

// Opaque struct containing the context used to operate on a SPIR-V module.
// Its object is used by various translation API functions.
#[repr(C)] pub struct spv_context_t { _priv: [u8; 0] }
#[repr(C)] pub struct spv_optimizer_t { _priv: [u8; 0] }
#[repr(C)] pub struct spv_validator_options_t { _priv: [u8; 0] }
#[repr(C)] pub struct spv_optimizer_options_t { _priv: [u8; 0] }
#[repr(C)] pub struct spv_reducer_options_t { _priv: [u8; 0] }

pub type spv_const_binary               = *mut spv_const_binary_t;
pub type spv_binary                     = *mut spv_binary_t;
pub type spv_text                       = *mut spv_text_t;
pub type spv_position                   = *mut spv_position_t;
pub type spv_diagnostic                 = *mut spv_diagnostic_t;
pub type spv_optimizer                  = *mut spv_optimizer_t;
pub type spv_const_optimizer            = *const spv_optimizer_t;
pub type spv_const_context              = *const spv_context_t;
pub type spv_context                    = *mut spv_context_t;
pub type spv_validator_options          = *mut spv_validator_options_t;
pub type spv_const_validator_options    = *const spv_validator_options_t;
pub type spv_optimizer_options          = *mut spv_optimizer_options_t;
pub type spv_const_optimizer_options    = *const spv_optimizer_options_t;
pub type spv_reducer_options            = *mut spv_reducer_options_t;
pub type spv_const_reducer_options      = *const spv_reducer_options_t;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct spv_target_env(u32);

impl spv_target_env {
    /// SPIR-V 1.0 latest revision, no other restrictions
    pub const UNIVERSAL_1_0: Self = Self(0);
    /// Vulkan 1.0 latest revision
    pub const VULKAN_1_0: Self = Self(1);
    /// SPIR-V 1.1 latest revision, no other restrictions
    pub const UNIVERSAL_1_1: Self = Self(2);
    /// OpenCL Full Profile 2.1 latest revision
    pub const OPENCL_2_1: Self = Self(3);
    /// OpenCL Full Profile 2.2 latest revision
    pub const OPENCL_2_2: Self = Self(4);
    /// OpenGL 4.0 plus GL_ARB_gl_spirv, latest revisions
    pub const OPENGL_4_0: Self = Self(5);
    /// OpenGL 4.1 plus GL_ARB_gl_spirv, latest revisions
    pub const OPENGL_4_1: Self = Self(6);
    /// OpenGL 4.2 plus GL_ARB_gl_spirv, latest revisions
    pub const OPENGL_4_2: Self = Self(7);
    /// OpenGL 4.3 plus GL_ARB_gl_spirv, latest revisions
    pub const OPENGL_4_3: Self = Self(8);
    
    // There is no variant for OpenGL 4.4.

    /// OpenGL 4.5 plus GL_ARB_gl_spirv, latest revisions
    pub const OPENGL_4_5: Self = Self(9);
    /// SPIR-V 1.2, latest revision, no other restrictions
    pub const UNIVERSAL_1_2: Self = Self(10);
    /// OpenCL Full Profile 1.2 plus cl_khr_il_program latest revision
    pub const OPENCL_1_2: Self = Self(11);
    /// OpenCL Embedded Profile 1.2 plus cl_khr_il_program, latest revision
    pub const OPENCL_EMBEDDED_1_2: Self = Self(12);
    /// OpenCL Full Profile 2.0 plus cl_khr_il_program, latest revision
    pub const OPENCL_2_0: Self = Self(13);
    /// OpenCL Embedded Profile 2.0 plus cl_khr_il_program, latest revision
    pub const OPENCL_EMBEDDED_2_0: Self = Self(14);
    /// OpenCL Embedded Profile 2.1 latest revision
    pub const OPENCL_EMBEDDED_2_1: Self = Self(15);
    /// OpenCL Embedded Profile 2.2 latest revision  
    pub const OPENCL_EMBEDDED_2_2: Self = Self(16);
    /// SPIR-V 1.3 latest revision, no other restrictions
    pub const UNIVERSAL_1_3: Self = Self(17);
    /// Vulkan 1.1 latest revision
    pub const VULKAN_1_1: Self = Self(18);
    /// Work in progress WebGPU 1.0
    pub const WEBGPU_0: Self = Self(19);
}

// SPIR-V Validator can be parameterized with the following Universal Limits.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct spv_validator_limit(u32);

impl spv_validator_limit {
    pub const max_struct_members: Self = Self(0);
    pub const max_struct_depth: Self = Self(1);
    pub const max_local_variables: Self = Self(2);
    pub const max_global_variables: Self = Self(3);
    pub const max_switch_branches: Self = Self(4);
    pub const max_function_args: Self = Self(5);
    pub const max_control_flow_nesting_depth: Self = Self(6);
    pub const max_access_chain_indexes: Self = Self(7);
    pub const max_id_bound: Self = Self(8);
}

#[link(name = "SPIRV-Tools", kind = "static")]
extern {
    /// Returns the SPIRV-Tools software version as a null-terminated string.
    /// The contents of the underlying storage is valid for the remainder of
    /// the process.
    pub fn spvSoftwareVersionString() -> *const c_char;
    
    /// Returns a null-terminated string containing the name of the project,
    /// the software version string, and commit details.
    /// The contents of the underlying storage is valid for the remainder of
    /// the process.
    pub fn spvSoftwareVersionDetailsString() -> *const c_char;

    /// Returns a string describing the given SPIR-V target environment.
    pub fn spvTargetEnvDescription(env: spv_target_env) -> *const c_char;

    /// Parses s into *env and returns true if successful.  If unparsable, returns
    /// false and sets *env to UNIVERSAL_1_0.
    pub fn spvParseTargetEnv(s: *const c_char, env: *mut spv_target_env) -> bool;

    /// Creates a context object.  Returns null if env is invalid.
    pub fn spvContextCreate(env: spv_target_env) -> spv_context;

    /// Destroys the given context object.
    pub fn spvContextDestroy(context: spv_context);

    /// Creates a Validator options object with default options. Returns a valid
    /// options object. The object remains valid until it is passed into
    /// spvValidatorOptionsDestroy.
    pub fn spvValidatorOptionsCreate() -> spv_validator_options;

    /// Destroys the given Validator options object.
    pub fn spvValidatorOptionsDestroy(options: spv_validator_options);

    /// Records the maximum Universal Limit that is considered valid in the given
    /// Validator options object. <options> argument must be a valid options object.
    pub fn spvValidatorOptionsSetUniversalLimit(
        options: spv_validator_options, 
        limit_type: spv_validator_limit,
        limit: u32
    );

    /// Record whether or not the validator should relax the rules on types for
    /// stores to structs.  When relaxed, it will allow a type mismatch as long as
    /// the types are structs with the same layout.  Two structs have the same layout
    /// if
    ///
    /// 1) the members of the structs are either the same type or are structs with
    /// same layout, and
    ///
    /// 2) the decorations that affect the memory layout are identical for both
    /// types.  Other decorations are not relevant.
    pub fn spvValidatorOptionsSetRelaxStoreStruct(
        options: spv_validator_options, 
        val: bool
    );

    /// Records whether or not the validator should relax the rules on pointer usage
    /// in logical addressing mode.
    ///
    /// When relaxed, it will allow the following usage cases of pointers:
    /// 1) OpVariable allocating an object whose type is a pointer type
    /// 2) OpReturnValue returning a pointer value
    pub fn spvValidatorOptionsSetRelaxLogicalPointer(
        options: spv_validator_options,
        val: bool
    );

    /// Records whether the validator should use "relaxed" block layout rules.
    /// Relaxed layout rules are described by Vulkan extension
    /// VK_KHR_relaxed_block_layout, and they affect uniform blocks, storage blocks,
    /// and push constants.
    ///
    /// This is enabled by default when targeting Vulkan 1.1 or later.
    /// Relaxed layout is more permissive than the default rules in Vulkan 1.0.
    pub fn spvValidatorOptionsSetRelaxBlockLayout(
        options: spv_validator_options,
        val: bool
    );

    /// Records whether the validator should use "scalar" block layout rules.
    /// Scalar layout rules are more permissive than relaxed block layout.
    ///
    /// See Vulkan extnesion VK_EXT_scalar_block_layout.  The scalar alignment is
    /// defined as follows:
    /// - scalar alignment of a scalar is the scalar size
    /// - scalar alignment of a vector is the scalar alignment of its component
    /// - scalar alignment of a matrix is the scalar alignment of its component
    /// - scalar alignment of an array is the scalar alignment of its element
    /// - scalar alignment of a struct is the max scalar alignment among its
    ///   members
    ///
    /// For a struct in Uniform, StorageClass, or PushConstant:
    /// - a member Offset must be a multiple of the member's scalar alignment
    /// - ArrayStride or MatrixStride must be a multiple of the array or matrix
    ///   scalar alignment
    pub fn spvValidatorOptionsSetScalarBlockLayout(
        options: spv_validator_options,
        val: bool
    );

    /// Records whether or not the validator should skip validating standard
    /// uniform/storage block layout.
    pub fn spvValidatorOptionsSetSkipBlockLayout(
        options: spv_validator_options,
        val: bool
    );

    /// Creates an optimizer options object with default options. Returns a valid
    /// options object. The object remains valid until it is passed into
    /// |spvOptimizerOptionsDestroy|.
    pub fn spvOptimizerOptionsCreate() -> spv_optimizer_options;

    /// Destroys the given optimizer options object.
    pub fn spvOptimizerOptionsDestroy(options: spv_optimizer_options);

    /// Records whether or not the optimizer should run the validator before
    /// optimizing.  If |val| is true, the validator will be run.
    pub fn spvOptimizerOptionsSetRunValidator(
        options: spv_optimizer_options, 
        val: bool
    );

    /// Records the validator options that should be passed to the validator if it is
    /// run.
    pub fn spvOptimizerOptionsSetValidatorOptions(
        options: spv_optimizer_options, 
        val: spv_validator_options
    );

    /// Records the maximum possible value for the id bound.
    pub fn spvOptimizerOptionsSetMaxIdBound(
        options: spv_optimizer_options, 
        val: u32
    );

    /// Records whether all bindings within the module should be preserved.
    pub fn spvOptimizerOptionsSetPreserveBindings(
        options: spv_optimizer_options,
        val: bool
    );

    /// Records whether all specialization constants within the module
    /// should be preserved.
    pub fn spvOptimizerOptionsSetPreserveSpecConstants(
        options: spv_optimizer_options,
        val: bool
    );

    /// Creates a reducer options object with default options. Returns a valid
    /// options object. The object remains valid until it is passed into
    /// |spvReducerOptionsDestroy|.
    pub fn spvReducerOptionsCreate() -> spv_reducer_options;

    /// Destroys the given reducer options object.
    pub fn spvReducerOptionsDestroy(options: spv_reducer_options);

    /// Records the maximum number of reduction steps that should run before the
    /// reducer gives up.
    pub fn spvReducerOptionsSetStepLimit(
        options: spv_reducer_options, 
        step_limit: u32
    );

    /// Sets seed for random number generation.
    pub fn spvReducerOptionsSetSeed(
        options: spv_reducer_options,
        seed: u32
    );

    /// Encodes the given SPIR-V assembly text to its binary representation. The
    /// length parameter specifies the number of bytes for text. Encoded binary will
    /// be stored into *binary. Any error will be written into *diagnostic if
    /// diagnostic is non-null, otherwise the context's message consumer will be
    /// used. The generated binary is independent of the context and may outlive it.
    pub fn spvTextToBinary(
        context: spv_const_context,
        text: *const c_char,
        length: size_t,
        binary: *mut spv_binary,
        diagnostic: *mut spv_diagnostic
    ) -> spv_result_t;

    /// Encodes the given SPIR-V assembly text to its binary representation. Same as
    /// spvTextToBinary but with options. The options parameter is a bit field of
    /// spv_text_to_binary_options_t.
    pub fn spvTextToBinaryWithOptions(
        context: spv_const_context,
        text: *const c_char,
        length: size_t,
        options: u32,
        binary: *mut spv_binary,
        diagnostic: *mut spv_diagnostic
    ) -> spv_result_t;

    /// Frees an allocated text stream. This is a no-op if the text parameter
    /// is a null pointer.
    pub fn spvTextDestroy(text: spv_text);

    /// Decodes the given SPIR-V binary representation to its assembly text. The
    /// word_count parameter specifies the number of words for binary. The options
    /// parameter is a bit field of spv_binary_to_text_options_t. Decoded text will
    /// be stored into *text. Any error will be written into *diagnostic if
    /// diagnostic is non-null, otherwise the context's message consumer will be
    /// used.
    pub fn spvBinaryToText(
        context: spv_const_context,
        binary: *const u32,
        word_count: size_t,
        options: u32,
        text: *mut spv_text,
        diagnostic: *mut spv_diagnostic
    ) -> spv_result_t;

    /// Frees a binary stream from memory. This is a no-op if binary is a null
    /// pointer.
    pub fn spvBinaryDestroy(binary: spv_binary);

    /// Validates a SPIR-V binary for correctness. Any errors will be written into
    /// *diagnostic if diagnostic is non-null, otherwise the context's message
    /// consumer will be used.
    pub fn spvValidate(
        context: spv_const_context,
        binary: spv_const_binary,
        diagnostic: *mut spv_diagnostic
    ) -> spv_result_t;

    /// Validates a SPIR-V binary for correctness. Uses the provided Validator
    /// options. Any errors will be written into *diagnostic if diagnostic is
    /// non-null, otherwise the context's message consumer will be used.
    pub fn spvValidateWithOptions(
        context: spv_const_context,
        options: spv_const_validator_options,
        binary: spv_const_binary,
        diagnostic: *mut spv_diagnostic
    ) -> spv_result_t;

    /// Validates a raw SPIR-V binary for correctness. Any errors will be written
    /// into *diagnostic if diagnostic is non-null, otherwise the context's message
    /// consumer will be used.
    pub fn spvValidateBinary(
        context: spv_const_context,
        words: *const u32,
        num_words: size_t,
        diagnostic: *mut spv_diagnostic
    ) -> spv_result_t;

    /// Creates a diagnostic object. The position parameter specifies the location in
    /// the text/binary stream. The message parameter, copied into the diagnostic
    /// object, contains the error message to display.
    pub fn spvDiagnosticCreate(
        position: spv_position,
        message: *const c_char
    ) -> spv_diagnostic;

    /// Destroys a diagnostic object.  This is a no-op if diagnostic is a null
    /// pointer.
    pub fn spvDiagnosticDestroy(diagnostic: spv_diagnostic);

    /// Prints the diagnostic to stderr.
    pub fn spvDiagnosticPrint(diagnostic: spv_diagnostic) -> spv_result_t;
}

#[link(name = "SPIRV-Tools-opt", kind = "static")]
extern {
    /// Create an optimizer instance for the target env
    pub fn spvOptimizerCreate(env: spv_target_env) -> spv_optimizer;

    /// Destroy an optimizer instance
    pub fn spvOptimizerDestroy(optimizer: spv_optimizer);

    /// Registers passes that attempt to improve performance of generated code.
    /// This sequence of passes is subject to constant review and will change
    /// from time to time.
    pub fn spvOptimizerRegisterPerformancePasses(optimizer: spv_optimizer);
    
    /// Registers passes that attempt to improve the size of generated code.
    /// This sequence of passes is subject to constant review and will change
    /// from time to time.
    pub fn spvOptimizerRegisterSizePasses(optimizer: spv_optimizer);

    /// Registers passes that have been prescribed for converting from Vulkan to
    /// WebGPU. This sequence of passes is subject to constant review and will
    /// change from time to time.
    pub fn spvOptimizerRegisterVulkanToWebGPUPasses(optimizer: spv_optimizer);

    /// Registers passes that have been prescribed for converting from WebGPU to
    /// Vulkan. This sequence of passes is subject to constant review and will
    /// change from time to time.
    pub fn spvOptimizerRegisterWebGPUToVulkanPasses(optimizer: spv_optimizer);

    /// Registers passes that attempt to legalize the generated code.
    ///
    /// Note: this recipe is specially designed for legalizing SPIR-V. It should be
    /// used by compilers after translating HLSL source code literally. It should
    /// *not* be used by general workloads for performance or size improvement.
    ///
    /// This sequence of passes is subject to constant review and will change
    /// from time to time.
    pub fn spvOptimizerRegisterLegalizationPasses(optimizer: spv_optimizer);

    /// Register passes specified in the list of |flags|.  Each flag must be a
    /// string of a form accepted by Optimizer::FlagHasValidForm().
    ///
    /// If the list of flags contains an invalid entry, it returns false and an
    /// error message is emitted to the MessageConsumer object (use
    /// Optimizer::SetMessageConsumer to define a message consumer, if needed).
    ///
    /// If all the passes are registered successfully, it returns true.
    pub fn spvOptimizerRegisterPassesFromFlags(
        optimizer: spv_optimizer,
        flags: *const *const c_char,
        num_flags: size_t
    ) -> bool;

    /// Registers the optimization pass associated with |flag|.  This only accepts
    /// |flag| values of the form "--pass_name[=pass_args]".  If no such pass
    /// exists, it returns false.  Otherwise, the pass is registered and it returns
    /// true.
    ///
    /// The following flags have special meaning:
    ///
    /// -O: Registers all performance optimization passes
    ///     (Optimizer::RegisterPerformancePasses)
    ///
    /// -Os: Registers all size optimization passes
    ///      (Optimizer::RegisterSizePasses).
    ///
    /// --legalize-hlsl: Registers all passes that legalize SPIR-V generated by an
    ///                  HLSL front-end.
    pub fn spvOptimizerRegisterPassFromFlag(
        optimizer: spv_optimizer,
        flag: *const c_char
    ) -> bool;

    /// Validates that |flag| has a valid format.  Strings accepted:
    ///
    /// --pass_name[=pass_args]
    /// -O
    /// -Os
    ///
    /// If |flag| takes one of the forms above, it returns true.  Otherwise, it
    /// returns false.
    pub fn spvOptimizerFlagHasValidForm(flag: *const c_char) -> bool;

    /// Allows changing, after creation time, the target environment to be
    /// optimized for.  Should be called before calling Run().
    pub fn spvOptimizerSetTargetEnv(optimizer: spv_optimizer, env: spv_target_env);

    /// Optimizes the given SPIR-V module |original_binary| and writes the
    /// optimized binary into |optimized_binary|.
    /// Returns true on successful optimization, whether or not the module is
    /// modified. Returns false if |original_binary| fails to validate or if errors
    /// occur when processing |original_binary| using any of the registered passes.
    /// In that case, no further passes are executed and the contents in
    /// |optimized_binary| may be invalid.
    ///
    /// It's allowed to alias |original_binary| to the start of |optimized_binary|.
    pub fn spvOptimizerRun(
        optimizer: spv_const_optimizer,
        original_binary: *const u32,
        original_bianry_size: size_t,
        optimized_binary: *mut spv_binary
    ) -> bool;

    /// Same as above, except it takes an options object.  See the documentation
    /// for |OptimizerOptions| to see which options can be set.
    pub fn spvOptimizerRunWithOptions(
        optimizer: spv_const_optimizer,
        original_binary: *const u32,
        original_binary_size: size_t,
        optimized_binary: *mut spv_binary,
        opt_options: spv_optimizer_options
    ) -> bool;

}