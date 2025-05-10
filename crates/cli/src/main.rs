use anyhow::{anyhow, bail, Error, Result};
use clap::Parser;
use std::{path::Path, process};
use wasmi::{Func, FuncType, Val};
use wasmi_cli::{
    args::Args,
    context::Context,
    display::{DisplayExportedFuncs, DisplayFuncType, DisplaySequence, DisplayValue},
    get_invoked_func,
    print_execution_start,
    print_pretty_results,
    print_remaining_fuel,
    typecheck_args,
    utils,
};

fn main() -> Result<()> {
    let args = Args::parse();
    let wasm_file = args.wasm_file();
    let wasi_ctx = args.wasi_context()?;
    let mut ctx = Context::new(wasm_file, wasi_ctx, args.fuel(), args.compilation_mode())?;
    let (func_name, func) = get_invoked_func(&args, &ctx)?;
    let ty = func.ty(ctx.store());
    let func_args = utils::decode_func_args(&ty, args.func_args())?;
    let mut func_results = utils::prepare_func_results(&ty);
    typecheck_args(&func_name, &ty, &func_args)?;

    if args.verbose() {
        print_execution_start(args.wasm_file(), &func_name, &func_args);
    }
    if args.invoked().is_some() && ty.params().len() != args.func_args().len() {
        bail!(
            "invalid amount of arguments given to function {}. expected {} but received {}",
            DisplayFuncType::new(&func_name, &ty),
            ty.params().len(),
            args.func_args().len()
        )
    }

    match func.call(ctx.store_mut(), &func_args, &mut func_results) {
        Ok(()) => {
            print_remaining_fuel(&args, &ctx);
            print_pretty_results(&func_results);
            Ok(())
        }
        Err(error) => {
            if let Some(exit_code) = error.i32_exit_status() {
                // We received an exit code from the WASI program,
                // therefore we exit with the same exit code after
                // pretty printing the results.
                print_remaining_fuel(&args, &ctx);
                print_pretty_results(&func_results);
                process::exit(exit_code)
            }
            bail!("failed during execution of {func_name}: {error}")
        }
    }
}
