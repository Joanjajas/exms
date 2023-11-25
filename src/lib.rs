//! **Crate for getting fast and easy readable statistics and comparisons about
//! school, college or any exam**
//!
//! IMPORTANT: This crate is under development, so continuous and big changes
//! are expected.
//!
//! You can create a `Exam` object from a file. For the moment the only file
//! formats supported are `JSON` and `TOML` files.
//!
//! # Examples
//!
//! ```no_run
//! use std::error::Error;
//! use std::path::Path;
//!
//! use exms::exam::Exam;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let file_path = Path::new("students.toml");
//!     let exam = Exam::from_file(&file_path)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # JSON
//!
//! The JSON file should have a single object that contains a key of "students"
//! and a value that is another object. This "students" object should contain
//! key/value pairs for each student, where the key is the student's name and
//! the value is the student's grade. The student's name should be a string, and
//! the grade should be a number.
//!
//! ```json
//! {
//!     "students": {
//!         "Abad Martinez, Jose": 4.89,
//!         "Acevedo Fuenzalida, Ignacio Joaquin": 5.79,
//!         "Alba Gisbert, Diego": 7.11,
//!         "Alcántara Campillo, Irene": 4.41,
//!     }
//! }
//! ```
//!
//! # TOML
//!
//! The TOML file should have a single table that contains a key of "students"
//! and entries for each student. Each entry should contain the student's name
//! as the key and the student's grade as the value. The student's name should
//! be a string, and the grade should be a number.
//!
//! ```toml
//! [students]
//! "Abad Martinez, Jose" = 4.89
//! "Acevedo Fuenzalida, Ignacio Joaquin" = 5.79
//! "Alba Gisbert, Diego" = 7.11
//! "Alcántara Campillo, Irene" = 4.41
//! ```
//!
//! # Parsing other format files
//!
//! Alternatively you can use your own parsing logic for any file you want to
//! parse, parsing the file content into a `Vec<Student>` and creating a new
//! `Exam` using the
//! [Exam::from_student_vec()](exam::Exam::from_student_vec) function.
//!
//! # Examples
//!
//! ```compile_fail
//! use std::error::Error;
//! use std::path::Path;
//!
//! use exms::exam::Exam;
//! use exms::exam::Student;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let file_path = Path::new("my_file");
//!
//!     // Here you should use your parsing function
//!     let students: Vec<Student> = my_parsing_logic::parse(&file_path);
//!     let exam = Exam::new(students, None);
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod exam;
pub mod input;
