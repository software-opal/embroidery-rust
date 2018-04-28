// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{
    errors {
        InvalidFormatError(msg: String)
    }
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }
}

// (position: uint32, message: String) {
//     description("invalid file format: {}", message)
//     display("invalid file format(at byte {}): '{}'", position, message)
// }
