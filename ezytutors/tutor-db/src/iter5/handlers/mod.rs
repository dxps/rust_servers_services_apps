pub mod course;
pub mod general;
pub mod tutor;

// Re-exporting with shorter (just this module) names.
pub use self::course::get_course_details;
pub use self::course::get_courses_for_tutor;
pub use self::course::post_new_course;
pub use self::general::health_check_handler;
