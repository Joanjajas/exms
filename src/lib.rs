//! Crate for getting fast and easy readable statistics and comparisons about
//! exam results
//!
//! You can create a [Exam](exms::exam::Exam) object from a file. For the
//! moment the only file formats supported are [JSON](#json) and [TOML](#toml)
//! files.
//!
//! # Examples
//!
//! ```no_run
//! use std::error::Error;
//! use std::path::Path;
//!
//! use exms::error::ParseError;
//! use exms::exam::Exam;
//!
//! fn main() -> Result<(), ParseError> {
//!     let file_path = Path::new("students.toml");
//!     let exam = Exam::from_file(&file_path)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Parsing from a file
//!
//! Each file that you want to parse should follow this format:
//! It must contain a single field named `students` that contains key/value
//! pairs for each student, where the key is the student's name and the value is
//! the student's grade. The student's name should be a string, and the grade
//! a number.
//!
//! The file can also contain an optional field named `details`. The currently
//! possible options for this field are:
//!
//! - `max_grade` (number): The maximum possible grade of the exam. If no value
//!   is provided, the maximum grade will default to 10.
//! - `name` (string): The name of the exam. If no value is provided, the file
//!   name will be used as the name.
//!
//!
//! Here are some examples of valid files:
//!
//! JSON:
//!
//! ```json
//! {
//!     "details": {
//!         "name": "Exam 1",
//!         "max_grade": 10
//!     },
//!
//!     "students": {
//!         "Abad Martinez, Jose": 4.89,
//!         "Acevedo Fuenzalida, Ignacio Joaquin": 5.79,
//!         "Alba Gisbert, Diego": 7.11,
//!         "Alcántara Campillo, Irene": 4.41,
//!     }
//! }
//! ```
//!
//! TOML:
//!
//! ```toml
//! [details] # This field is optional
//! name = "Exam 1"
//! max_grade = 10
//!
//! [students]
//! "Abad Martinez, Jose" = 4.89
//! "Acevedo Fuenzalida, Ignacio Joaquin" = 5.79
//! "Alba Gisbert, Diego" = 7.11
//! "Alcántara Campillo, Irene" = 4.41
//! ```
//!
//! # Parsing other file formats
//!
//! Alternatively you can use your own parsing logic for any file you want to
//! parse, parsing the file content into a `Vec<Student>` and creating a new
//! `Exam` using the
//! [Exam::new()](exam::Exam::new) function.
//!
//! ```compile_fail
//! use std::error::Error;
//! use std::path::Path;
//!
//! use exms::exam::{Exam, Student};
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let file_path = Path::new("my_file");
//!
//!     // Here you should parse your file into a Vec<Student>
//!     let students: Vec<Student> = parse_file(&file_path)?;
//!     let exam = Exam::new(students);
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod exam;
