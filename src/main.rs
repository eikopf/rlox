mod bytecode;
mod value;
mod vm;

use argh::FromArgs;

#[derive(FromArgs)]
/// A rust-based bytecode VM for executing Lox code,
/// as described in the book Crafting Interpreters by
/// Robert Nystrom
struct Rlox {
    #[argh(subcommand)]
    subcommand: RloxSubCommand,
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand)]
enum RloxSubCommand {
    Run(RunSubCommand),
    Disassemble(DisassembleSubCommand),
    Repl(ReplSubCommand),
}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "run")]
/// Executes a .lox file.
struct RunSubCommand {}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "disassemble")]
/// Disassembles a .lox file into bytecode.
struct DisassembleSubCommand {}

#[derive(FromArgs, PartialEq)]
#[argh(subcommand, name = "repl")]
/// Loads into a Lox REPL.
struct ReplSubCommand {}

fn main() {
    // cli entrypoint
    argh::from_env::<Rlox>();
}
