extern crate spirv_tools_rs;

use spirv_tools_rs::*;

const ASM_SRC: &'static str = r#"
    ; Magic:     0x07230203 (SPIR-V)
    ; Version:   0x00010000 (Version: 1.0.0)
    ; Generator: 0x00080001 (Khronos Glslang Reference Front End; 1)
    ; Bound:     63
    ; Schema:    0

        OpCapability Shader
    %1 = OpExtInstImport "GLSL.std.450"
        OpMemoryModel Logical GLSL450
        OpEntryPoint Fragment %4 "main" %31 %33 %42 %57
        OpExecutionMode %4 OriginLowerLeft

    ; Debug information
        OpSource GLSL 450
        OpName %4 "main"
        OpName %9 "scale"
        OpName %17 "S"
        OpMemberName %17 0 "b"
        OpMemberName %17 1 "v"
        OpMemberName %17 2 "i"
        OpName %18 "blockName"
        OpMemberName %18 0 "s"
        OpMemberName %18 1 "cond"
        OpName %20 ""
        OpName %31 "color"
        OpName %33 "color1"
        OpName %42 "color2"
        OpName %48 "i"
        OpName %57 "multiplier"

    ; Annotations (non-debug)
        OpDecorate %15 ArrayStride 16
        OpMemberDecorate %17 0 Offset 0
        OpMemberDecorate %17 1 Offset 16
        OpMemberDecorate %17 2 Offset 96
        OpMemberDecorate %18 0 Offset 0
        OpMemberDecorate %18 1 Offset 112
        OpDecorate %18 Block
        OpDecorate %20 DescriptorSet 0
        OpDecorate %42 NoPerspective

    ; All types, variables, and constants
        %2 = OpTypeVoid
        %3 = OpTypeFunction %2                      ; void ()
        %6 = OpTypeFloat 32                         ; 32-bit float
        %7 = OpTypeVector %6 4                      ; vec4
        %8 = OpTypePointer Function %7              ; function-local vec4*
        %10 = OpConstant %6 1
        %11 = OpConstant %6 2
        %12 = OpConstantComposite %7 %10 %10 %11 %10 ; vec4(1.0, 1.0, 2.0, 1.0)
        %13 = OpTypeInt 32 0                         ; 32-bit int, sign-less
        %14 = OpConstant %13 5
        %15 = OpTypeArray %7 %14
        %16 = OpTypeInt 32 1
        %17 = OpTypeStruct %13 %15 %16
        %18 = OpTypeStruct %17 %13
        %19 = OpTypePointer Uniform %18
        %20 = OpVariable %19 Uniform
        %21 = OpConstant %16 1
        %22 = OpTypePointer Uniform %13
        %25 = OpTypeBool
        %26 = OpConstant %13 0
        %30 = OpTypePointer Output %7
        %31 = OpVariable %30 Output
        %32 = OpTypePointer Input %7
        %33 = OpVariable %32 Input
        %35 = OpConstant %16 0
        %36 = OpConstant %16 2
        %37 = OpTypePointer Uniform %7
        %42 = OpVariable %32 Input
        %47 = OpTypePointer Function %16
        %55 = OpConstant %16 4
        %57 = OpVariable %32 Input

    ; All functions
        %4 = OpFunction %2 None %3                  ; main()
        %5 = OpLabel
        %9 = OpVariable %8 Function
        %48 = OpVariable %47 Function
            OpStore %9 %12
        %23 = OpAccessChain %22 %20 %21              ; location of cond
        %24 = OpLoad %13 %23                         ; load 32-bit int from cond
        %27 = OpINotEqual %25 %24 %26                ; convert to bool
            OpSelectionMerge %29 None              ; structured if
            OpBranchConditional %27 %28 %41        ; if cond
        %28 = OpLabel                                ; then
        %34 = OpLoad %7 %33
        %38 = OpAccessChain %37 %20 %35 %21 %36      ; s.v[2]
        %39 = OpLoad %7 %38
        %40 = OpFAdd %7 %34 %39
            OpStore %31 %40
            OpBranch %29
        %41 = OpLabel                                ; else
        %43 = OpLoad %7 %42
        %44 = OpExtInst %7 %1 Sqrt %43               ; extended instruction sqrt
        %45 = OpLoad %7 %9
        %46 = OpFMul %7 %44 %45
            OpStore %31 %46
            OpBranch %29
        %29 = OpLabel                                ; endif
            OpStore %48 %35
            OpBranch %49
        %49 = OpLabel
            OpLoopMerge %51 %52 None               ; structured loop
            OpBranch %53
        %53 = OpLabel
        %54 = OpLoad %16 %48
        %56 = OpSLessThan %25 %54 %55                ; i < 4 ?
            OpBranchConditional %56 %50 %51        ; body or break
        %50 = OpLabel                                ; body
        %58 = OpLoad %7 %57
        %59 = OpLoad %7 %31
        %60 = OpFMul %7 %59 %58
            OpStore %31 %60
            OpBranch %52
        %52 = OpLabel                                ; continue target
        %61 = OpLoad %16 %48
        %62 = OpIAdd %16 %61 %21                     ; ++i
            OpStore %48 %62
            OpBranch %49                           ; loop back
        %51 = OpLabel                                ; loop merge point
            OpReturn
            OpFunctionEnd
"#;

#[test]
fn assemble() {
    let ctx = SpirvContext::new(TargetEnv::OpenGl4_5);
    let assembled = ctx.assemble(ASM_SRC);

    assert!(assembled.is_ok(), "Assembly failed with '{:?}'", assembled);
    assert!(assembled.unwrap().len() > 0);
}

#[test]
fn disassemble() {
    let ctx = SpirvContext::new(TargetEnv::OpenGl4_5);
    let assembled = ctx.assemble(ASM_SRC)
        .unwrap();
    let disassembled = ctx.disassemble(&assembled);

    assert!(disassembled.is_ok(), "Disassembly failed with '{:?}'", disassembled);
    assert!(disassembled.unwrap().len() > 0);

}

#[test]
fn validate() {
    let ctx = SpirvContext::new(TargetEnv::OpenGl4_5);

    let assembled = ctx.assemble(ASM_SRC)
        .unwrap();
    let validated = ctx.validate(&assembled);

    assert!(validated.is_ok(), "Validation failed with '{:?}'", validated);
    assert!(assembled.len() > 0);
}

#[test]
fn optimize() {
    let ctx = SpirvContext::new(TargetEnv::OpenGl4_5);
    let opt = SpirvOptimizer::new(TargetEnv::OpenGl4_5)
        .register_performance_passes();
    
    let assembled = ctx.assemble(ASM_SRC)
        .unwrap();
    let optimized = opt.run(&assembled)
        .unwrap();
    let validated = ctx.validate(&optimized);

    assert!(validated.is_ok(), "Optimization failed with '{:?}'", validated);
    assert!(optimized.len() > 0);
}