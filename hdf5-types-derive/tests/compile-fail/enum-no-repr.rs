extern crate hdf5_types;
#[macro_use]
extern crate hdf5_types_derive;

#[derive(H5Type)]
//~^ ERROR proc-macro derive
//~^^ HELP H5Type can only be derived for enums with explicit representation
enum Foo {
    X = 1,
    Y = 2,
}
