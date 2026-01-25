// this files  is more about how to handle files in rust
//
//what is file format?
//File formats live in a large design space with trade-offs in performance, human-readability,
//and portability. Some formats are highly portable and self-describing.
//Others restrict themselves to being accessible within a single environment and are unable
//to be read by third-party tools, yet they are high performance.
//

//common file formats
//1. Plain Text Files: These are the simplest file formats, consisting of human-readable text. Examples include .txt, .csv, and .log files.
//2. JSON (JavaScript Object Notation): A lightweight data interchange format that is easy for humans to read and write, and easy for machines to parse and generate. Commonly used in web applications.
//3. XML (eXtensible Markup Language): A markup language that defines a set of rules for encoding documents in a format that is both human-readable and machine-readable.
//4. YAML (YAML Ain't Markup Language): A human-readable data serialization format that is often used for configuration files and data exchange between languages with different data structures.
//5. Binary Files: These files contain data in a format that is not human-readable. Examples include .exe, .bin, and image files like .png and .jpg.
//6. PDF (Portable Document Format): A file format developed by Adobe that presents documents independently of software, hardware, or operating systems.
//7. Markdown (.md): A lightweight markup language with plain text formatting syntax, often used for documentation and readme files.
//8. INI Files: Simple configuration files that use a basic structure of sections, properties, and values.
//9. SQLite Database Files: A self-contained, serverless, zero-configuration, transactional SQL database engine.
//10. Proprietary Formats: Many applications use their own proprietary file formats, which may not be easily accessible or readable by other software.
////In Rust, you can handle files using the standard library's `std::fs` module, which provides functions for reading from and writing to files.

use serde::Serialize;

#[derive(Serialize)]
struct City {
    name: String,
    population: u32,
}
