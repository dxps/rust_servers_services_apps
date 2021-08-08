pub mod course;
pub mod tutor;

// Re-exporting with shorter (just this module) names.
pub use self::course::Course;
pub use self::course::CreateCourse;
pub use self::course::UpdateCourse;
