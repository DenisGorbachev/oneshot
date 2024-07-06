use clap::ValueEnum;

#[derive(ValueEnum, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum OutputTarget {
    None,
    Package,
    Home,
}
