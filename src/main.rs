use setup::paths::make_fundamental_directories;

mod setup {
    pub mod paths; 
}

mod data_preparation {
    pub mod sourcing;
    pub mod authors;
}


fn main() {
    make_fundamental_directories();
}
