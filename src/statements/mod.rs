pub use self::declare_stmt::*;
pub use self::if_stmt::*;
pub use self::return_stmt::*;
pub use self::statement::*;
pub use self::go_stmt::*;

pub mod go_stmt;
pub mod if_stmt;
pub mod declare_stmt;
pub mod statement;
pub mod return_stmt;
pub mod for_stmt;

