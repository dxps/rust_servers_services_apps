pub mod course;
pub mod tutor;

// Re-exporting with shorter (just this module) names.
pub use self::course::delete_course_db;
pub use self::course::get_course_details_db;
pub use self::course::get_courses_for_tutor_db;
pub use self::course::post_new_course_db;
pub use self::course::update_course_details_db;
