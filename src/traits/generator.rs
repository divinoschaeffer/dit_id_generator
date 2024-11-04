/// The `Generator` trait provides an interface for generating a unique ID.
///
/// This trait defines a single method, `generate_id`, intended to produce a unique identifier
/// each time it is called. Implementations can decide how the ID is generated, which allows for flexibility
/// in its use.
///
/// # Example
///
/// ```rust, ignore
///
/// struct IdGenerator;
///
/// impl Generator for IdGenerator {
///     fn generate_id(&mut self) -> String {
///         "unique_id_123".to_string() // Replace with actual ID generation logic
///     }
/// }
///
/// fn main() {
///     let mut generator = IdGenerator;
///     let id = generator.generate_id();
///     println!("Generated ID: {}", id);
/// }
/// ```
///
/// # Notes
///
/// - `generate_id` is called on a mutable reference to allow implementations to modify internal state if necessary.
pub trait Generator {
    /// Generates a unique ID.
    ///
    /// # Returns
    ///
    /// A `String` representing the generated unique ID.
    fn generate_id(&mut self) -> String;
}
