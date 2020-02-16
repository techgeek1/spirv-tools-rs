//! # Spirv Tools RS - A thin, safe, wrapper over the Spirv-Tools library by Kronos
//! Spirv-Tools is a set of utilities for working with SPIR-V.
//! Spriv Tools RS provides a means of safely working with these tools from rust code
//! and provides unsafe api bindings should the wrapper be insufficient.
//! 
//! # Structure
//! The crate root contains the safe wrapper over the bindings
//! `raw` contains the raw bindings

mod error;
mod opt;

pub mod raw;

pub use error::*;
pub use opt::*;

use std::ffi::CString;
use std::ptr;
use std::slice;
use std::str;

use self::raw::*;

pub type SpvResult<T> = Result<T, SpvError>;

#[derive(Clone, Copy)]
pub enum TargetEnv {
    /// SPIR-V 1.0 latest revision, no other restrictions
    Universal1_0,
    /// Vulkan 1.0 latest revision
    Vulkan1_0,
    /// SPIR-V 1.1 latest revision, no other restrictions
    Universal1_1,
    /// OpenCL Full Profile 2.1 latest revision
    OpenCL2_1,
    /// OpenCL Full Profile 2.2 latest revision
    OpenCL2_2,
    /// OpenGL 4.0 plus GL_ARB_gl_spirv, latest revisions
    OpenGL4_0,
    /// OpenGL 4.1 plus GL_ARB_gl_spirv, latest revisions
    OpenGL4_1,
    /// OpenGL 4.2 plus GL_ARB_gl_spirv, latest revisions
    OpenGL4_2,
    /// OpenGL 4.3 plus GL_ARB_gl_spirv, latest revisions
    OpenGL4_3,
    
    // There is no variant for OpenGL 4.4.

    /// OpenGL 4.5 plus GL_ARB_gl_spirv, latest revisions
    OpenGl4_5,
    /// SPIR-V 1.2, latest revision, no other restrictions
    Universal1_2,
    /// OpenCL Full Profile 1.2 plus cl_khr_il_program latest revision
    OpenCL1_2,
    /// OpenCL Embedded Profile 1.2 plus cl_khr_il_program, latest revision
    OpenCLEmbedded1_2,
    /// OpenCL Full Profile 2.0 plus cl_khr_il_program, latest revision
    OpenCL2_0,
    // OpenCL Embedded Profile 2.0 plus cl_khr_il_program, latest revision
    OpenCLEmbedded2_0,
    /// OpenCL Embedded Profile 2.1 latest revision
    OpenCLEmbedded2_1,
    /// OpenCL Embedded Profile 2.2 latest revision  
    OpenCLEmbedded2_2,
    /// SPIR-V 1.3 latest revision, no other restrictions
    Universal1_3,
    /// Vulkan 1.1 latest revision
    Vulkan1_1,
    /// Work in progress WebGPU 1.0
    WebGPU0,
}

impl TargetEnv {
    /// Convert the rust version of `TargetEnv` into the C version
    fn to_raw(self) -> spv_target_env {
        match self {
            TargetEnv::Universal1_0          => spv_target_env::UNIVERSAL_1_0,
            TargetEnv::Vulkan1_0             => spv_target_env::VULKAN_1_0,
            TargetEnv::Universal1_1          => spv_target_env::UNIVERSAL_1_1,
            TargetEnv::OpenCL2_1             => spv_target_env::OPENCL_2_1,
            TargetEnv::OpenCL2_2             => spv_target_env::OPENCL_2_2,
            TargetEnv::OpenGL4_0             => spv_target_env::OPENGL_4_0,
            TargetEnv::OpenGL4_1             => spv_target_env::OPENGL_4_1,
            TargetEnv::OpenGL4_2             => spv_target_env::OPENGL_4_2,
            TargetEnv::OpenGL4_3             => spv_target_env::OPENGL_4_3,
            TargetEnv::OpenGl4_5             => spv_target_env::OPENGL_4_5,
            TargetEnv::Universal1_2          => spv_target_env::UNIVERSAL_1_2,
            TargetEnv::OpenCL1_2             => spv_target_env::OPENCL_1_2,
            TargetEnv::OpenCLEmbedded1_2     => spv_target_env::OPENCL_EMBEDDED_1_2,
            TargetEnv::OpenCL2_0             => spv_target_env::OPENCL_2_0,
            TargetEnv::OpenCLEmbedded2_0     => spv_target_env::OPENCL_EMBEDDED_2_0,
            TargetEnv::OpenCLEmbedded2_1     => spv_target_env::OPENCL_EMBEDDED_2_1,
            TargetEnv::OpenCLEmbedded2_2     => spv_target_env::OPENCL_EMBEDDED_2_2,
            TargetEnv::Universal1_3          => spv_target_env::UNIVERSAL_1_3,
            TargetEnv::Vulkan1_1             => spv_target_env::VULKAN_1_1,
            TargetEnv::WebGPU0               => spv_target_env::WEBGPU_0
        }
    }
}

/// Options for dissassembling a spirv binary
#[derive(Clone, Copy)]
pub struct DisassembleOptions {
    raw: u32
}

impl DisassembleOptions {
    pub fn none() -> Self {
        Self { raw: 0 }
    }

    pub fn print(self) -> Self {
        Self { raw: self.raw | (1 << 1) }
    }

    pub fn color(self) -> Self {
        Self { raw: self.raw | (1 << 2) }
    }

    pub fn indent(self) -> Self {
        Self { raw: self.raw | (1 << 3) }
    }

    pub fn show_byte_offset(self) -> Self {
        Self { raw: self.raw | (1 << 4) }
    }

    pub fn no_header(self) -> Self {
        Self { raw: self.raw | (1 << 5) }
    }

    pub fn friendly_names(self) -> Self {
        Self { raw: self.raw | (1 << 6) }
    }

    #[inline]
    fn into_raw(self) -> u32 {
        self.raw
    }
}

pub enum ValidatorLimit {
    MaxStructMembers(u32),
    MaxStructDept(u32),
    MaxLocalVariables(u32),
    MaxGlobalVariables(u32),
    MaxSwitchBranches(u32),
    MaxFunctionArgs(u32),
    MaxControlFlowNestingDepth(u32),
    MaxAccessChainIndexes(u32),
    MaxIdBound(u32)
}

/// A set of options for controlling validation
pub struct ValidatorOptions {
    raw: spv_validator_options
}

impl ValidatorOptions {
    /// Create a new validator options
    pub fn new() -> Self {
        Self { 
            raw: unsafe { spvValidatorOptionsCreate() }
        }
    }

    /// Records the maximum Universal Limit that is considered valid in the given
    /// Validator options object
    pub fn limit(self, limit: ValidatorLimit) -> Self {
        let (limit_type, value) = match limit {
            ValidatorLimit::MaxStructMembers(x)             => (spv_validator_limit::max_struct_members             , x),
            ValidatorLimit::MaxStructDept(x)                => (spv_validator_limit::max_struct_depth               , x),
            ValidatorLimit::MaxLocalVariables(x)            => (spv_validator_limit::max_local_variables            , x),
            ValidatorLimit::MaxGlobalVariables(x)           => (spv_validator_limit::max_global_variables           , x),
            ValidatorLimit::MaxSwitchBranches(x)            => (spv_validator_limit::max_switch_branches            , x),
            ValidatorLimit::MaxFunctionArgs(x)              => (spv_validator_limit::max_function_args              , x),
            ValidatorLimit::MaxControlFlowNestingDepth(x)   => (spv_validator_limit::max_control_flow_nesting_depth , x),
            ValidatorLimit::MaxAccessChainIndexes(x)        => (spv_validator_limit::max_access_chain_indexes       , x),
            ValidatorLimit::MaxIdBound(x)                   => (spv_validator_limit::max_id_bound                   , x)
        };

        unsafe { spvValidatorOptionsSetUniversalLimit(self.raw, limit_type, value) };
        self
    }

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
    pub fn relax_store_struct(self, relax_store: bool) -> Self {
        unsafe { spvValidatorOptionsSetRelaxStoreStruct(self.raw, relax_store); }
        self
    }

    /// Records whether or not the validator should relax the rules on pointer usage
    /// in logical addressing mode.
    ///
    /// When relaxed, it will allow the following usage cases of pointers:
    /// 1) OpVariable allocating an object whose type is a pointer type
    /// 2) OpReturnValue returning a pointer value
    pub fn relax_logical_pointer(self, relax_ptr: bool) -> Self {
        unsafe { spvValidatorOptionsSetRelaxLogicalPointer(self.raw, relax_ptr); }
        self
    }

    /// Records whether the validator should use "relaxed" block layout rules.
    /// Relaxed layout rules are described by Vulkan extension
    /// VK_KHR_relaxed_block_layout, and they affect uniform blocks, storage blocks,
    /// and push constants.
    ///
    /// This is enabled by default when targeting Vulkan 1.1 or later.
    /// Relaxed layout is more permissive than the default rules in Vulkan 1.0.
    pub fn relax_block_layout(self, relax_layout: bool) -> Self {
        unsafe { spvValidatorOptionsSetRelaxBlockLayout(self.raw, relax_layout); }
        self
    }

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
    pub fn scalar_block_layout(self, scalar_layout: bool) -> Self {
        unsafe { spvValidatorOptionsSetScalarBlockLayout(self.raw, scalar_layout); }
        self
    }

    /// Records whether or not the validator should skip validating standard
    /// uniform/storage block layout.
    pub fn skip_block_layout(self, skip_layout: bool) -> Self {
        unsafe { spvValidatorOptionsSetSkipBlockLayout(self.raw, skip_layout); }
        self
    }
}

impl Drop for ValidatorOptions {
    fn drop(&mut self) {
        unsafe { spvValidatorOptionsDestroy(self.raw); }
    }
}

impl Default for ValidatorOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// A context for invoking spirv-tools
pub struct Context {
    env: TargetEnv,
    include_diagnostics: bool
}

impl Context {
    /// Create a new context for the target environment
    pub fn new(env: TargetEnv) -> Self {
        Self {
            env: env,
            include_diagnostics: false
        }
    }

    /// Include diagnostic information in any error codes
    pub fn with_diagnostics(mut self) -> Self {
        self.include_diagnostics = true;
        self
    }

    /// Assemble a spirv binary from it's textual form
    pub fn assemble<'src>(&self, source: &'src str) -> Result<Vec<u32>, AssembleError<'src>> {
        unsafe {
            // Setup to call the C library
            let context = spvContextCreate(self.env.to_raw());
            let src = CString::new(source)
                .map_err(|_| AssembleError::InvalidSourceString(source))?;
            
            // Assemble the source code
            let (err_code, bin, diag) = {
                let str_ptr = src.as_ptr();
                let str_len = src.as_bytes().len();
                
                let mut out_bin: spv_binary = ptr::null_mut();
                let mut out_diag = ptr::null_mut();
    
                let result = if self.include_diagnostics {
                    spvTextToBinary(
                        context, 
                        str_ptr,
                        str_len,
                        &mut out_bin as *mut spv_binary,
                        &mut out_diag as *mut spv_diagnostic
                    )
                }
                else {
                    spvTextToBinary(
                        context, 
                        str_ptr,
                        str_len,
                        &mut out_bin as *mut spv_binary,
                        ptr::null_mut()
                    )
                };
    
                (result, out_bin, out_diag)
            };
    
            // Copy over the resulting code from C memory to Rust memory
            let result = match err_code {
                spv_result_t::SUCCESS => {
                    let slice = slice::from_raw_parts(
                        (*bin).code, 
                        (*bin).word_count as usize
                    );
    
                    let mut binary = Vec::with_capacity(slice.len());
                    binary.set_len(slice.len());
                    binary.copy_from_slice(slice);
    
                    Ok(binary)
                },
                _                     => {
                    let (err, diag) = SpvError::from_raw(err_code, diag);
                    Err(AssembleError::SpirvTools(err, diag))
                }
            };
    
            // Cleanup
            if !diag.is_null() {
                spvDiagnosticDestroy(diag);
            }
    
            if !bin.is_null() {
                spvBinaryDestroy(bin);
            }
    
            spvContextDestroy(context);
    
            result
        }
    }

    /// Disassemble a spirv binary into it's textual form using default options
    #[inline]
    pub fn disassemble(&self, binary: &[u32]) -> Result<String, DisassembleError> {
        self.disassemble_with_options(binary, DisassembleOptions::none())
    }

    /// Disassemble a spirv binary into it's textual form with the specified options
    pub fn disassemble_with_options(&self, binary: &[u32], options: DisassembleOptions) -> Result<String, DisassembleError> {
        unsafe {
            // Setup and disassemble the binary
            let context = spvContextCreate(self.env.to_raw());
    
            let (err_code, text, diag) = {
                let mut out_text: spv_text = ptr::null_mut();
                let mut out_diag = ptr::null_mut();
    
                let result = if self.include_diagnostics {
                    spvBinaryToText(
                        context,
                        binary.as_ptr(),
                        binary.len(),
                        options.into_raw(),
                        &mut out_text as *mut spv_text,
                        &mut out_diag as *mut spv_diagnostic
                    )
                }
                else {
                    spvBinaryToText(
                        context,
                        binary.as_ptr(),
                        binary.len(),
                        options.into_raw(),
                        &mut out_text as *mut spv_text,
                        ptr::null_mut()
                    )
                };
    
                (result, out_text, out_diag)
            };
    
            let result = match err_code {
                spv_result_t::SUCCESS => {
                    let bytes = slice::from_raw_parts(
                        (*text).string as *const u8, 
                        (*text).length
                    );
    
                    let text = str::from_utf8(bytes)
                        .and_then(|x| Ok(x.to_owned()))
                        .expect("Spirv Tools returned an invalid encoding!");
    
                    Ok(text)
                },
                _                     => {
                    let (err, diag) = SpvError::from_raw(err_code, diag);
                    Err(DisassembleError::SpirvTools(err, diag))
                }
            };
    
            if !diag.is_null() {
                spvDiagnosticDestroy(diag);
            }
    
            if !text.is_null() {
                spvTextDestroy(text);
            }
    
            spvContextDestroy(context);
    
            result
        }
    }

    /// Validate a spirv binary with the default options
    #[inline]
    pub fn validate(&self, binary: &[u32]) -> Result<(), ValidateError> {
        self.validate_with_options(binary, ValidatorOptions::default())
    }

    /// Validate a spirv binary with a set of options
    pub fn validate_with_options(&self, binary: &[u32], options: ValidatorOptions) -> Result<(), ValidateError> {
        unsafe {
            let context = spvContextCreate(self.env.to_raw());
            let mut binary = spv_const_binary_t {
                code: binary.as_ptr(),
                word_count: binary.len()
            };
    
            let (err_code, diag) = {
                let mut out_diag = ptr::null_mut();
                
                let result = if self.include_diagnostics {
                    spvValidateWithOptions(
                        context,
                        options.raw,
                        &mut binary as spv_const_binary,
                        &mut out_diag as *mut spv_diagnostic
                    )
                }
                else {
                    spvValidateWithOptions(
                        context,
                        options.raw,
                        &mut binary as spv_const_binary,
                        ptr::null_mut()
                    )
                };
    
                (result, out_diag)
            };
            
            let result = match err_code {
                spv_result_t::SUCCESS => Ok(()),
                _                     => {
                    let (err, diag) = SpvError::from_raw(err_code, diag);
                    Err(ValidateError::SpirvTools(err, diag))
                }
            };
    
            if !diag.is_null() {
                spvDiagnosticDestroy(diag);
            }
    
            spvContextDestroy(context);
    
            result
        }
    }
}
