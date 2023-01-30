use serde::Serialize;
use std::fmt::Debug;
use std::io::Write;
use std::error;

/// Supported output formats
#[derive(clap::ValueEnum, Clone)]
pub enum Format {
    Json,
    Yaml,
    Text
}

/// Builds a printer
///
/// # Example
///
/// ```
/// use dinstaller_cli::printers::{PrinterBuilder, Format};
/// use std::io;
///
/// let numbers = [1, 2, 3];
/// let printer = PrinterBuilder::new(numbers, io::stdout())
///   .with_format(Format::Json)
///   .build();
/// printer.print();
/// ```
pub struct PrinterBuilder<T, W> {
    format: Format,
    content: T,
    writer: W
}

// 'static restricts the T to types that don't contain any references
// see https://users.rust-lang.org/t/confused-about-type-lifetimes-and-e0310/30702/3
impl<T: Serialize + Debug + 'static, W: Write + 'static> PrinterBuilder<T, W> {
    pub fn new(content: T, writer: W) -> Self {
        PrinterBuilder {
            format: Format::Text,
            writer,
            content,
        }
    }

    /// Sets the printer format
    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    /// Builds the printer with the given configuration
    pub fn build(self) -> Box<dyn Printer<T, W>> {
        match self.format {
            Format::Json => Box::new(JsonPrinter {
                content: self.content, writer: self.writer
            }),
            Format::Text => Box::new(TextPrinter {
                content: self.content , writer: self.writer
            }),
            Format::Yaml => Box::new(YamlPrinter {
                content: self.content , writer: self.writer
            })
        }
    }
}

pub trait Printer<T, W> {
    fn print(self: Box<Self>) -> Result<(), Box<dyn error::Error>>;
}

pub struct JsonPrinter<T, W> {
    content: T,
    writer: W,
}

impl<T: Serialize + Debug, W: Write> Printer<T, W> for JsonPrinter<T, W> {
    fn print(self: Box<Self>) -> Result<(), Box<dyn error::Error>> {
        Ok(serde_json::to_writer(self.writer, &self.content)?)
    }
}

pub struct TextPrinter<T, W> {
    content: T,
    writer: W
}

impl<T: Serialize + Debug, W: Write> Printer<T, W> for TextPrinter<T, W> {
    fn print(mut self: Box<Self>) -> Result<(), Box<dyn error::Error>> {
        Ok(write!(self.writer, "{:?}", &self.content)?)
    }
}

pub struct YamlPrinter<T, W> {
    content: T,
    writer: W
}

impl<T: Serialize + Debug, W: Write> Printer<T, W> for YamlPrinter<T, W> {
    fn print(self: Box<Self>) -> Result<(), Box<dyn error::Error>> {
        Ok(serde_yaml::to_writer(self.writer, &self.content)?)
    }
}

/// Prints the content using the given format
///
/// # Example
///
///```rust
/// use dinstaller_lib::users;
/// use dinstaller_cli::printers::{print, Format};
/// use std::io;
///
/// let user = users::User { login: "jane doe".to_string() };
/// print(user, io::stdout(), Some(Format::Json))
/// ```
pub fn print<T, W>(content: T, writer: W, format: Option<Format>)
where T: serde::Serialize + Debug + 'static, W: Write + 'static {

    let format = format.unwrap_or(Format::Text);
    let printer = PrinterBuilder::new(content, writer)
        .with_format(format)
        .build();
    printer.print().unwrap();

}

