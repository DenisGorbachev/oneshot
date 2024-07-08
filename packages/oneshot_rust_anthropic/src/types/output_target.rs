use clap::ValueEnum;

#[derive(ValueEnum, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum OutputTarget {
    None,
    #[default]
    Package,
    Home,
}
