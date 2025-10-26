mod m2k;
mod m4k;

use anyhow::Result;

pub trait UseAsMapper {
    fn new(program_path: &str) -> Result<Self>
    where
        Self: Sized;

    fn read(&mut self, addr: usize) -> Result<u8>;
    fn write(&mut self, addr: usize) -> Result<()>;
}

macro_rules! define_mappers {
    (
        $(
            $variant:ident => $module:ident :: $struct_name:ident
        ),* $(,)?
    ) => {
        #[derive(clap::ValueEnum, Copy, Clone, Debug, serde::Serialize)]
        #[serde(rename_all = "lowercase")]
        pub enum MapperKind {
            $( $variant ),*
        }

        impl MapperKind {
            pub fn to_mapper(self, program_path: &str) -> anyhow::Result<Box<dyn UseAsMapper>> {
                match self {
                    $(
                        MapperKind::$variant => Ok(Box::new($module::$struct_name::new(program_path)?)),
                    )*
                }
            }
        }
    };
}

define_mappers! {
    M2K => m2k::Mapper2K,
    M4K => m4k::Mapper4K,
}
