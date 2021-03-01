//! Command line parser setup for elf2tab.

use std::path::PathBuf;

use structopt::StructOpt;

fn usage() -> &'static str {
    "elf2tab [FLAGS] [OPTIONS] ELF...
Converts Tock userspace programs from .elf files to Tock Application Bundles."
}

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Convert Tock userland apps from .elf files to Tock Application Bundles (TABs or .tab files).",
    usage = usage(),
    global_setting(structopt::clap::AppSettings::ColoredHelp)
)]
pub struct Opt {
    #[structopt(short = "v", long = "verbose", help = "Be verbose")]
    pub verbose: bool,

    #[structopt(long = "deterministic", help = "Produce a deterministic TAB file")]
    pub deterministic: bool,

    #[structopt(
        long = "minimum-ram-size",
        name = "min-ram-size",
        help = "in bytes",
        conflicts_with = "stack-size",
        conflicts_with = "heap-size",
        conflicts_with = "kernel-heap-size"
    )]
    pub minimum_stack_size: Option<u32>,

    #[structopt(
        long = "output-file",
        short = "o",
        name = "filename",
        default_value = "TockApp.tab",
        parse(from_os_str),
        help = "output file name"
    )]
    pub output: PathBuf,

    #[structopt(
        long = "package-name",
        short = "n",
        name = "pkg-name",
        help = "package name"
    )]
    pub package_name: Option<String>,

    #[structopt(
        long = "stack",
        name = "stack-size",
        default_value = "2048",
        help = "in bytes"
    )]
    pub stack_size: u32,

    #[structopt(
        long = "app-heap",
        name = "heap-size",
        default_value = "1024",
        help = "in bytes"
    )]
    pub app_heap_size: u32,

    #[structopt(
        long = "kernel-heap",
        name = "kernel-heap-size",
        default_value = "1024",
        help = "in bytes"
    )]
    pub kernel_heap_size: u32,

    #[structopt(
        name = "elf",
        help = "application file(s) to package",
        parse(from_os_str),
        required = true
    )]
    pub input: Vec<PathBuf>,

    #[structopt(
        long = "protected-region-size",
        name = "protected-region-size",
        help = "Size of the protected region (including headers)"
    )]
    pub protected_region_size: Option<u32>,

    #[structopt(
        long = "supported-boards",
        name = "supported-boards",
        help = "comma separated list of boards this app is compatible with"
    )]
    pub supported_boards: Option<String>,
}

mod test {

    #[cfg(test)]
    use super::Opt;
    #[cfg(test)]
    use structopt::StructOpt;

    #[test]
    // elf2tab [FLAGS] [--package-name=<pkg-name>] [--output-file=[<filename>]] <elf>...
    fn simple_invocations_succeed() {
        {
            let args = vec!["elf2tab", "app.elf"];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_ok());
        }
        {
            let args = vec!["elf2tab", "--package-name", "my-pkg", "app.elf"];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_ok());
        }
        {
            let args = vec!["elf2tab", "--output-file", "out.tab", "app.elf"];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_ok());
        }
        {
            let args = vec!["elf2tab", "--package-name", "my-pkg", "app.elf"];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_ok());
        }
        {
            let args = vec![
                "elf2tab",
                "--output-file",
                "out.tab",
                "--package-name",
                "pkg-name",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            println!("{:?}", result);
            assert!(result.is_ok());
        }
    }

    #[test]
    // elf2tab [FLAGS] [--package-name=<pkg-name>] [--output-file=[<filename>]] <elf>...
    fn simple_invocations_fail() {
        {
            let args = vec!["elf2tab", "app.elf", "--package-name"];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_err());
        }
    }

    #[test]
    // elf2tab [FLAGS] [--package-name=<pkg-name>] [--output-file=[<filename>]] [--minimum-stack-size=<min-stack-size>] <elf>...
    fn advanced_invocations_succeed() {
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--minimum-ram-size",
                "10",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_ok());
        }
    }

    #[test]
    // elf2tab [FLAGS] [--package-name=<pkg-name>] [--output-file=[<filename>]] [--minimum-stack-size=<min-stack-size>] <elf>...
    fn advanced_invocations_fail() {
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--minimum-ram-size",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_err());
        }
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--minimum-ram-size",
                "10",
                "--app-heap",
                "10",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_err());
        }
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--minimum-ram-size",
                "10",
                "--stack",
                "10",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_err());
        }
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--minimum-ram-size",
                "10",
                "--kernel-heap",
                "10",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_err());
        }
    }

    #[test]
    // elf2tab [FLAGS] [--package-name=<pkg-name>] [--output-file=[<filename>]] [--app-heap[=<heap-size>]]
    //                [--kernel-heap[=<kernel-heap-size>]] [--stack[=<stack-size>]] <elf>..."
    fn expert_invocations_succeed() {
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--kernel-heap",
                "10",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_ok());
        }
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--app-heap",
                "10",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_ok());
        }
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--stack",
                "10",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_ok());
        }
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--stack",
                "10",
                "--app-heap",
                "10",
                "--kernel-heap",
                "10",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_ok());
        }
    }

    #[test]
    // elf2tab [FLAGS] [--package-name=<pkg-name>] [--output-file=[<filename>]] [--app-heap[=<heap-size>]]
    //                [--kernel-heap[=<kernel-heap-size>]] [--stack[=<stack-size>]] <elf>..."
    fn expert_invocations_fail() {
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--kernel-heap",
                "10",
                "--minimum-ram-size",
                "10",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_err());
        }
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--app-heap",
                "10",
                "--minimum-ram-size",
                "10",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_err());
        }
        {
            let args = vec![
                "elf2tab",
                "--package-name",
                "my-pkg",
                "--stack",
                "10",
                "--minimum-ram-size",
                "10",
                "app.elf",
            ];
            let result = Opt::from_iter_safe(args.iter());
            assert!(result.is_err());
        }
    }
}
