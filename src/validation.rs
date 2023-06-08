///
/// Validation path element
///
enum Element {
    ///
    /// Field name
    ///
    Field(&'static str),

    ///
    /// Id
    ///
    Id(String),
}

///
/// A validation path
///
struct Path {
    ///
    /// Path elements
    ///
    elements: Vec<Element>,
}

impl Path {
    ///
    /// Creates a new path
    ///
    fn new() -> Path {
	Path {
	    elements: Vec::new(),
	}
    }

    ///
    /// Pushes a field
    ///
    fn push_field(&mut self, name: &'static str) {
	self.elements.push(Element::Field(name));
    }

    ///
    /// Pops an element off the path
    ///
    fn pop(&mut self) {
	self.elements.pop();
    }

    ///
    /// Creates a string representation of the path
    ///
    fn to_string(&self) -> String {
	let mut buffer = String::new();
	self.elements.iter().for_each(|e| {
	    buffer.push('/');
	    match e {
		Element::Field(f) => buffer.push_str(f),
		Element::Id(id) => buffer.push_str(id),
	    }
	});
	buffer
    }
}

///
/// A generic validation error
///
#[derive(Debug, PartialEq)]
pub struct Error {
    ///
    /// The message
    ///
    message: String,

    ///
    /// The path
    ///
    path: String,
}

impl Error {
    ///
    /// Creates a new error
    ///
    fn new(path: &Path, message: &str) -> Error {
	Error {
	    message: String::from(message),
	    path: path.to_string(),
	}
    }
}

///
/// A generic validator
///
pub struct Validator {
    ///
    /// The current path
    ///
    path: Path,
}

impl Validator {

    ///
    /// Creates a new validator
    ///
    pub fn new() -> Validator {
	Validator {
	    path: Path::new(),
	}
    }

    ///
    /// Validates a field
    ///
    pub fn validate_field<T>(&mut self, name: &'static str, message: &str, value: T, predicate: fn(&T) -> bool) -> Result<T, Error> {
	self.path.push_field(name);
	if !predicate(&value) {
	    Err(Error::new(&self.path, message))
	} else {
	    self.path.pop();
	    Ok(value)
	}
    }
    
    ///
    /// Validates a field from
    ///
    pub fn validate_field_into<O, I: ValidateInto<O>>(&mut self, name: &'static str, value: I) -> Result<O, Error> {
	self.path.push_field(name);
	let output = value.validate_into(self)?;
	self.path.pop();
	Ok(output)
    }
    
    ///
    /// Validates an input value
    ///
    pub fn validate_into<O, I : ValidateInto<O>>(&mut self, input: I) -> Result<O, Error> {
	input.validate_into(self)
    }
}

pub trait ValidateInto<T> {

    ///
    /// Converts a type into a validated type
    ///
    fn validate_into(self, validator: &mut Validator) -> Result<T, Error>;
    
}
