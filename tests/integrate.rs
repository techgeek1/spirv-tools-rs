extern crate spirv_tools_rs;

use spirv_tools_rs::*;

const ASM_SRC0: &'static str = r#"
    OpEntryPoint GLCompute %3 "foo"
    %1 = OpTypeVoid
    %2 = OpTypeFunction %1
    %3 = OpFunction %1 None %2
    OpFunctionEnd
"#;

const ASM_SRC1: &'static str = r#"
    OpEntryPoint GLCompute %3 "bar"
    %1 = OpTypeVoid
    %2 = OpTypeFunction %1
    %3 = OpFunction %1 None %2
    OpFunctionEnd
"#;

// TODO: Figure out better tests for this

#[test]
fn assemble() {
    let ctx = SpirvContext::new();
    let result = spirv::assemble(ctx, ASM_SRC0);

    assert!(result.is_ok(), "Assembly failed with '{:?}'", result);

    let result = spirv::assemble(ctx, ASM_SRC1);

    assert!(result.is_ok(), "Assembly failed with '{:?}'", result);
}

#[test]
fn disassemble() {
    let ctx = SpirvContext::new();
    let assembled = ctx.assemble(ASM_SRC0)
        .unwrap();
    let disassembled = ctx.disassemble(&assembled);

    assert!(disassembled.is_ok(), "Disassembly failed with '{:?}'", disassembled);

    let assembled = ctx.assemble(ctx, ASM_SRC1)
        .unwrap();
    let disassembled = ctx.disassemble(ctx, &assembled);

    assert!(disassembled.is_ok(), "Disassembly failed with '{:?}'", disassembled);
}

#[test]
fn validate() {
    let ctx = SpirvContext::new();

    let assembled = ctx.assemble(ctx, ASM_SRC0)
        .unwrap();
    let validated = ctx.validate(ctx, &optimized);

    assert!(validated.is_ok(), "Optimization failed with '{:?}'", disassembled);

    let assembled = ctx.assemble(ctx, ASM_SRC1)
        .unwrap();
    let validated = ctx.validate(ctx, &optimized);

    assert!(validated.is_ok(), "Optimization failed with '{:?}'", disassembled);
}

#[test]
fn optimize() {
    let ctx = SpirvContext::new();
    let opt = SpirvOptimizer::new(ctx.env)
        .register_performance_passes();
    
    let assembled = ctx.assemble(ctx, ASM_SRC0)
        .unwrap();
    let optimized = opt.run(&assembled)
        .unwrap();
    let validated = ctx.validate(ctx, &optimized);

    assert!(validated.is_ok(), "Optimization failed with '{:?}'", disassembled);

    let assembled = ctx.assemble(ctx, ASM_SRC1)
        .unwrap();
    let optimized = opt.run(&assembled)
        .unwrap();
    let validated = ctx.validate(ctx, &optimized);

    assert!(validated.is_ok(), "Optimization failed with '{:?}'", disassembled);
}