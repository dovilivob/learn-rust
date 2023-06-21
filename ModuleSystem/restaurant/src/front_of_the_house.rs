// By default, all items (functions, methods, structs, enums, modules, and constants) are private.
pub mod hosting;

// this child module can access the parent module's private items, and it's private itself
mod serving;
