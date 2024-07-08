use directories::ProjectDirs;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PROJECT_DIRS: Option<ProjectDirs> = ProjectDirs::from("org", "Oneshot", "Oneshot");
}
