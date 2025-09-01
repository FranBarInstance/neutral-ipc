
use serde_json::json;
use std::error::Error;
use std::result::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use neutralts::Template;

// ============================================
// Neutral IPC record version 0 (draft version)
// ============================================
//
// HEADER:
//
// \x00              # reserved
// \x00              # control (action/status) (10 = parse template)
// \x00              # content-format 1 (10 = JSON, 20 = file path, 30 = plaintext, 40 = binary)
// \x00\x00\x00\x00  # content-length 1 big endian byte order
// \x00              # content-format 2 (10 = JSON, 20 = file path, 30 = plaintext, 40 = binary)
// \x00\x00\x00\x00  # content-length 2 big endian byte order (can be zero)
//
// All text utf8

const HEADER_SIZE: usize = 12;
const CTRL_PARSE_TEMPLATE: u8 = 10;
const CTRL_STATUS_OK: u8 = 0;
const _CTRL_STATUS_KO: u8 = 1;
const CONTENT_JSON: u8 = 10;
const CONTENT_PATH: u8 = 20;
const CONTENT_TEXT: u8 = 30;
const _CONTENT_BIN: u8 = 40;

// IPC config
const CONFIG_FILE: &str = "/etc/neutral-ipc-cfg.json";

struct Config {
    host: String,
    port: String,
}

impl Config {
    pub fn new() -> Self {
        match fs::read_to_string(CONFIG_FILE) {
            Ok(config_content) => {
                match serde_json::from_str::<serde_json::Value>(&config_content) {
                    Ok(config) => Config {
                        host: config["host"].as_str().unwrap_or("127.0.0.1").to_string(),
                        port: config["port"].as_str().unwrap_or("4273").to_string(),
                    },
                    Err(_) => {
                        eprintln!("Config is not a valid JSON, default is used.");
                        Config::default()
                    }
                }
            },
            Err(_) => {
                eprintln!("Impossible to read config, default is used.");
                Config::default()
            }
        }
    }

    fn default() -> Self {
        Config {
            host: "127.0.0.1".to_string(),
            port: "4273".to_string(),
        }
    }
}

/// Header structure representing the protocol header.
///
/// The header contains information about the request or response, including reserved fields,
/// control/status indicators, content formats, and content lengths.
#[derive(Debug)]
pub struct Header {
    /// Reserved field that must be set to 0x00. This field is reserved for future use.
    pub reserved: u8,

    /// Control field indicating the action for requests or status for responses.
    /// - For requests:
    ///   - `10`: Parse template
    ///   - Other values can be defined as needed.
    /// - For responses:
    ///   - `0`: Success
    ///   - `1`: General error
    ///   - Other values can indicate specific error states.
    pub control: u8,

    /// Content format for the first content block. Possible values include:
    /// - `10`: JSON
    /// - `20`: File path
    /// - `30`: Plaintext
    /// - `40`: Binary
    pub content_format_1: u8,

    /// Length of the first content block in bytes, represented in big-endian byte order.
    pub content_length_1: u32,

    /// Content format for the second content block. Possible values are the same as for `content_format_1`.
    pub content_format_2: u8,

    /// Length of the second content block in bytes, represented in big-endian byte order.
    /// This field can be zero if there is no second content block.
    pub content_length_2: u32,
}

impl Header {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < HEADER_SIZE {
            return None;
        }
        Some(Header {
            reserved: bytes[0],
            control: bytes[1],
            content_format_1: bytes[2],
            content_length_1: u32::from_be_bytes([bytes[3], bytes[4], bytes[5], bytes[6]]),
            content_format_2: bytes[7],
            content_length_2: u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
        })
    }

    fn to_bytes(&self) -> [u8; HEADER_SIZE] {
        let mut buffer = [0; HEADER_SIZE];
        buffer[0] = self.reserved;
        buffer[1] = self.control;
        buffer[2] = self.content_format_1;
        buffer[3..7].copy_from_slice(&self.content_length_1.to_be_bytes());
        buffer[7] = self.content_format_2;
        buffer[8..12].copy_from_slice(&self.content_length_2.to_be_bytes());
        buffer
    }
}

struct ParseTemplateResult {
    json: String,
    text: String,
    status: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new();
    let bindto = format!("{}:{}", config.host.as_str(), config.port);
    let listener = TcpListener::bind(bindto).await?;
    println!("Neutral IPC on {}:{}",config.host, config.port);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream).await {
                        eprintln!("Failed to handle client: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut header_bytes = [0; HEADER_SIZE];
    stream.read_exact(&mut header_bytes).await?;

    if let Some(header) = Header::from_bytes(&header_bytes) {
        match header.control {
            CTRL_PARSE_TEMPLATE => {
                if header.content_format_1 != CONTENT_JSON {
                    return Err("Invalid content_format_1. Expected JSON.".into());
                }

                if header.content_format_2 != CONTENT_TEXT && header.content_format_2 != CONTENT_PATH {
                    return Err("Invalid content_format_2. Expected TEXT or PATH.".into());
                }

                let mut content_1_buffer = vec![0; header.content_length_1 as usize];
                stream.read_exact(&mut content_1_buffer).await?;

                let mut content_2_buffer = vec![0; header.content_length_2 as usize];
                stream.read_exact(&mut content_2_buffer).await?;

                let json_content = String::from_utf8(content_1_buffer)
                    .map_err(|e| format!("Failed to parse json content: {}", e))?;

                let text_content = String::from_utf8(content_2_buffer)
                    .map_err(|e| format!("Failed to parse text content: {}", e))?;

                let result = parse_template(&json_content, &text_content, header.content_format_2);
                let response_header = Header {
                    reserved: 0,
                    control: result.status,
                    content_format_1: CONTENT_JSON,
                    content_length_1: result.json.len() as u32,
                    content_format_2: CONTENT_TEXT,
                    content_length_2: result.text.len() as u32,
                };

                stream.write_all(&response_header.to_bytes()).await?;
                stream.write_all(result.json.as_bytes()).await?;
                stream.write_all(result.text.as_bytes()).await?;
            }
            _ => {
                return Err("Unsupported control code".into());
            }
        }
    } else {
        return Err("Invalid header format".into());
    }

    Ok(())
}

fn parse_template(schema: &str, tpl: &str, tpl_type: u8) -> ParseTemplateResult {
    let mut template = Template::new().unwrap();
    template.merge_schema_str(schema).unwrap();

    if tpl_type == CONTENT_PATH {
        template.set_src_path(tpl).unwrap();
    } else {
        template.set_src_str(tpl);
    }

    let contents = template.render();
    let result = json!({
        "has_error": template.has_error(),
        "status_code": template.get_status_code(),
        "status_text": template.get_status_text(),
        "status_param": template.get_status_param()
    });

    ParseTemplateResult {
        json: result.to_string(),
        text: contents,
        status: CTRL_STATUS_OK,
    }
}
