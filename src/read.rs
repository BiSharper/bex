use std::{io, ops};

/// A Trait for managing and analyzing a sequence (array/slice) of data one item at a time
/// # Type Parameters
/// * `T` - Any type that is Sized (has a constant size in memory) and can be compared for equality
pub trait Analyser<T: Sized + PartialEq + Copy> {
    /// Get the entire sequence being analyzed
    ///
    /// # Returns
    /// Array slice of the sequence being analyzed
    fn contents(&self) -> &[T];

    /// Get the current position of cursor within the sequence
    ///
    /// # Returns
    /// Cursor position as usize
    fn pos(&self) -> &usize;

    /// Consumes the analyser, returning the sequence being analyzed
    ///
    /// # Returns
    /// The sequence being analyzed as an owned vector
    fn drain(self) -> Vec<T>;

    /// Sets the cursor to a given position
    ///
    /// # Arguments
    /// * `position` - The index in sequence, where cursor will be placed
    ///
    /// # Returns
    /// `std::io::Result<()>` - Ok if operation successful, otherwise an Err with the `std::io::Error`
    fn set_pos(&mut self, position: usize) -> io::Result<()>;

    /// Move the cursor one position back
    ///
    /// # Returns
    /// `std::io::Result<()>` - Ok if operation successful, otherwise an Err with the `std::io::Error`
    fn step_back(&mut self) -> io::Result<()>  { self.set_pos(self.pos() - 1) }

    /// Move the cursor one position forward
    ///
    /// # Returns
    /// `std::io::Result<()>` - Ok if operation successful, otherwise an Err with the `std::io::Error`
    fn step_forward(&mut self) -> io::Result<()> { self.set_pos(self.pos() + 1) }

    /// Get the length of sequence
    ///
    /// # Returns
    /// Length of the sequence as usize
    fn len(&self) -> usize { self.contents().len() }

    /// Check if end of sequence is reached by the cursor
    ///
    /// # Returns
    /// Boolean that's true if end of sequence has been reached
    fn is_end(&self) -> bool { self.pos() >= &self.len() }

    /// Resets the cursor to first position (at index 0)
    ///
    /// # Returns
    /// `std::io::Result<()>` - Ok if operation successful, otherwise an Err with the `std::io::Error`
    fn reset(&mut self) -> io::Result<()> { self.set_pos(0) }

    /// Compares the current position's element with the target, moves cursor forward if they match
    ///
    /// # Arguments
    /// `target` - Target element to compare with the current element in sequence
    ///
    /// # Returns
    /// `std::io::Result<bool>` - Ok if operation successful, containing true if element matched target and optionally moved forward, otherwise an Err with the `std::io::Error`
    fn take(&mut self, target: &T) -> io::Result<bool> {
        let result = self.peek()? == target;
        if result { self.step_forward()?; }
        Ok(result)
    }

    /// Takes multiple elements from the sequence if they match provided targets.
    ///
    /// # Arguments
    /// * `target` -  An ordered sequence of target elements to compare and consume from the sequence
    ///
    /// # Returns
    /// `std::io::Result<bool>` - Ok(true) if all elements in the sequence match the targets and move the cursor forward, otherwise Ok(false).
    fn take_multi(&mut self, target: &[&T]) -> io::Result<bool>  {
        for &element in target {
            match self.take(element) {
                Ok(val) => {
                    if !val { return Ok(false); }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(true)
    }

    /// Looks at the current element in the sequence without moving the cursor.
    ///
    /// # Returns
    /// `std::io::Result<&T>` - Ok with a reference to the current element in the sequence, otherwise an Err with the `std::io::Error` if the cursor is beyond the sequence bounds ('end of file' condition).
    fn peek(&self) -> io::Result<&T> {
        self.contents()
            .get(*self.pos())
            .ok_or(
                io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "End of file was reached unexpectedly."
                )
            )
    }

    /// Gets the current element and then moves the cursor forward by one position.
    ///
    /// # Returns
    /// `std::io::Result<T>` - Ok with a copy of the current element in the sequence, otherwise an Err with the `std::io::Error` if the cursor is beyond the sequence bounds ('end of file' condition).
    fn get(&mut self) -> io::Result<T> {
        let current = *self.peek()?;
        self.step_forward()?;
        return Ok(current)
    }

}

impl<T: Sized + PartialEq + Copy> ops::Index<ops::Range<usize>> for dyn Analyser<T> {
    type Output = [T];

    fn index(&self, range: ops::Range<usize>) -> &[T] {
        &self.contents()[range]
    }
}