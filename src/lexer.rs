use std::io;
use std::io::{Read, Seek, SeekFrom};

pub struct Bexer<R: Read + Seek> {
    reader: R,
    peeked: Option<Option<char>>
}

impl<R: Read + Seek> Bexer<R> {
    pub fn create(reader: R) -> Self { Self { reader, peeked: None } }

    pub fn step_back(&mut self) -> io::Result<u64>  {
        self.peeked = None;
        self.reader.seek(SeekFrom::Current(-1))
    }

    pub fn step_forward(&mut self) -> io::Result<u64>  {
        self.peeked = None;
        self.reader.seek(SeekFrom::Current(1))
    }

    pub fn reset(&mut self) -> io::Result<u64>  {
        self.peeked = None;
        self.reader.seek(SeekFrom::Start(0))
    }

    pub fn take(&mut self, target: char) -> io::Result<bool> { match self.peek()? {
        None => Ok(false),
        Some(found) => match found {
            _ if found == target => Ok(true),
            _ => Ok(false)
        }
    }}

    pub fn take_while_char(&mut self, char: char) -> io::Result<usize> {
        self.take_while(&char, |char, found |
            found.is_some_and(|c | c == *char)
        )
    }

    pub fn take_until_char(&mut self, char: char) -> io::Result<usize> {
        self.take_while(&char, |char, found |
            found.is_some_and(|c | c != *char)
        )
    }

    pub fn take_until_chars(&mut self, chars: &[char]) -> io::Result<usize> {
        self.take_while(chars, |chars, found |
            found.is_some_and(|c | !chars.contains(&c))
        )
    }

    pub fn take_while_chars(&mut self, chars: &[char]) -> io::Result<usize> {
        self.take_while(chars, |chars, found |
            found.is_some_and(|c | chars.contains(&c))
        )
    }

    pub fn take_while<T: ?Sized>(&mut self, predicate_data: &T, predicate: fn(&T, Option<char>) -> bool) -> io::Result<usize> {
        let mut count: usize = 0;

        while predicate(predicate_data, self.peek()?) {
            self.step_forward()?;
            count += 1;
        }

        Ok(count)
    }

    pub fn take_multi(&mut self, string: &str) -> io::Result<bool> {
        let mut length: usize = 0;
        for i in string.chars() { match self.take(i)? {
            true => length += 1,
            false => break
        }}
        Ok(length == string.len())
    }

    pub fn peek(&mut self) -> io::Result<Option<char>> {
        match self.peeked {
            None => {
                let char = decode_char(&mut self.reader, true)?;
                self.peeked = Some(char);
                Ok(char)
            }
            Some(peeked) => Ok(peeked)
        }
    }

    pub fn consume(&mut self) -> io::Result<Option<char>> {
        match self.peeked {
            None => {
                let char = decode_char(&mut self.reader, true)?;
                self.peeked = Some(char);
                Ok(self.peeked.unwrap())
            }
            Some(peeked) => match peeked {
                None => Ok(None),
                Some(peeked) => {
                    self.reader.seek(SeekFrom::Current(peeked.len_utf8() as i64))?;
                    Ok(Some(peeked))
                }
            }

        }
    }
}

fn decode_char<R: Read + Seek>(reader: &mut R, quietly: bool) -> io::Result<Option<char>> {
    let mut bytes = [0; 4];
    let mut len = 0;

    loop {
        match reader.read(&mut bytes[len..len + 1]) {
            Ok(0) => break, // End of file
            Ok(n) => len += n,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }

        if let Some(c) = std::str::from_utf8(&bytes[..len]).ok().and_then(|s| s.chars().next()) {
            if quietly { reader.seek(SeekFrom::Current(-(len as i64)))?; }
            return Ok(Some(c));
        }

        if len == 4 {
            if quietly { reader.seek(SeekFrom::Current(-(len as i64)))?; }
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence", ));
        }
    }

    if quietly { reader.seek(SeekFrom::Current(-(len as i64)))?; }
    return Ok(None)
}

