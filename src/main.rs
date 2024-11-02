// Server endpoints can be checked using Nushell:
//
// # Define the check command
// def check-endpoints [] {
//     ["/version" "/healthz" "/metrics"] | each { |ep| 
//         try { 
//             http get $"http://127.0.0.1:3000($ep)"
//             {endpoint: $ep, status: "OK"}
//         } catch {
//             {endpoint: $ep, status: "Error"}
//         }
//     }
// }
//
// # Check all endpoints
// check-endpoints | table
//
// # Check failed endpoints
// $endpoint_status | where status == "Error" | table
//
// # Check working endpoints
// $endpoint_status | where status == "OK" | table
//
// Example output:
// ╭───┬──────────┬────────╮
// │ # │ endpoint │ status │
// ├───┼──────────┼────────┤
// │ 0 │ /version │ OK     │
// │ 1 │ /healthz │ Error  │
// │ 2 │ /metrics │ Error  │
// ╰───┴──────────┴────────╯


use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
use nu_table::{NuTable, NuTableConfig, TableTheme};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    
    // Initial server status
    let mut table = NuTable::new(2, 2);
    table.insert((0, 0).into(), "Status".to_string());
    table.insert((0, 1).into(), "Server Started".to_string());
    table.insert((1, 0).into(), "Address".to_string());
    table.insert((1, 1).into(), "http://127.0.0.1:3000".to_string());
    
    let config = NuTableConfig {
        theme: TableTheme::rounded(),
        ..NuTableConfig::default()
    };
    
    if let Some(output) = table.draw(config.clone(), 80) {
        println!("{}", output);
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer_addr = stream.peer_addr()?;
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                // Connection info table
                let mut status_table = NuTable::new(4, 2);
                status_table.insert((0, 0).into(), "Connection".to_string());
                status_table.insert((0, 1).into(), format!("{}:{}", peer_addr.ip(), peer_addr.port()));
                status_table.insert((1, 0).into(), "Time".to_string());
                status_table.insert((1, 1).into(), timestamp.to_string());
                
                match handle_connection(stream) {
                    Ok((bytes, status, path)) => {
                        status_table.insert((2, 0).into(), "Request".to_string());
                        status_table.insert((2, 1).into(), path);
                        status_table.insert((3, 0).into(), "Response".to_string());
                        status_table.insert((3, 1).into(), format!("✓ {} ({} bytes)", status, bytes));
                    }
                    Err(e) => {
                        status_table.insert((2, 0).into(), "Status".to_string());
                        status_table.insert((2, 1).into(), format!("✗ Failed: {}", e));
                    }
                }
                
                if let Some(output) = status_table.draw(config.clone(), 80) {
                    println!("{}", output);
                }
            }
            Err(e) => {
                let mut error_table = NuTable::new(1, 2);
                error_table.insert((0, 0).into(), "Error".to_string());
                error_table.insert((0, 1).into(), e.to_string());
                
                if let Some(output) = error_table.draw(config.clone(), 80) {
                    println!("{}", output);
                }
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<(usize, String, String)> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let first_line = request.lines().next().unwrap_or("");
    
    let (status_line, content, status_text, path) = match first_line {
        s if s.starts_with("GET /version ") => {
            let json = format!(r#"{{
                "version": "{}",
                "commit": "unknown",
                "branch": "main",
                "built_at": "{}",
                "rust_version": "{}",
                "platform": "{}",
                "arch": "{}"
            }}"#,
                env!("CARGO_PKG_VERSION"),
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                rustc_version_runtime::version().to_string(), 
                std::env::consts::OS,
                std::env::consts::ARCH
            );
            
            if request.contains("Accept: application/json") {
                (
                    "HTTP/1.1 200 OK",
                    json,
                    "200 OK",
                    "/version"
                )
            } else {
                (
                    "HTTP/1.1 200 OK",
                    format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>Version Information</title>
    <style>
        body {{ 
            font-family: 'Courier New', monospace;
            background: #1a1a1a;
            color: #e0e0e0;
            padding: 40px;
            line-height: 1.6;
        }}
        .terminal {{
            background: #252525;
            border-radius: 6px;
            padding: 20px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
            border: 1px solid #333;
        }}
        .info-title {{
            color: #6ba2ff;
            font-size: 24px;
            margin: 0 0 20px 0;
        }}
        .data-grid {{
            display: grid;
            grid-template-columns: auto 1fr;
            gap: 10px;
            margin: 20px 0;
        }}
        .label {{
            color: #a0a0a0;
            padding-right: 20px;
        }}
        .value {{
            color: #6ba2ff;
        }}
        pre {{
            background: #1a1a1a;
            padding: 15px;
            border-radius: 4px;
            border: 1px solid #404040;
            overflow-x: auto;
        }}
    </style>
</head>
<body>
    <div class="terminal">
        <h1 class="info-title">Server Version Information</h1>
        <div class="data-grid">
            <div class="label">Version:</div>
            <div class="value">0.1.0</div>
            <div class="label">Platform:</div>
            <div class="value">{}</div>
            <div class="label">Architecture:</div>
            <div class="value">{}</div>
            <div class="label">Build Time:</div>
            <div class="value">{}</div>
        </div>
        <h2 class="info-title">Raw JSON Response</h2>
        <pre>{}</pre>
    </div>
</body>
</html>"#,
                    std::env::consts::OS,
                    std::env::consts::ARCH,
                    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    json
                ),
                "200 OK",
                "/version"
                )
            }
        },
        s if s.starts_with("GET ") => {
            let path = s.split_whitespace().nth(1).unwrap_or("/unknown");
            (
                "HTTP/1.1 404 NOT FOUND",
                format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>404 - Not Found</title>
    <style>
        @import url('https://fonts.googleapis.com/css2?family=Fira+Code:wght@400;600&display=swap');
        body {{ 
            font-family: 'Fira Code', monospace;
            background: #1c1c1c;
            color: #d4d4d4;
            padding: 2rem;
            margin: 0;
            line-height: 1.5;
        }}
        .terminal {{
            background: #252525;
            border: 1px solid #333;
            border-radius: 8px;
            padding: 2rem;
            max-width: 800px;
            margin: 2rem auto;
            box-shadow: 0 10px 30px rgba(0,0,0,0.3);
        }}
        .error-code {{
            color: #ff6b6b;
            font-size: 1.5rem;
            margin-bottom: 1.5rem;
            font-weight: 600;
        }}
        .path-box {{
            background: #1c1c1c;
            border: 1px solid #333;
            border-radius: 4px;
            padding: 1rem;
            margin: 1rem 0;
            font-family: 'Fira Code', monospace;
            color: #4d9375;
        }}
        .divider {{
            border-top: 1px solid #333;
            margin: 2rem 0;
        }}
        .endpoints-table {{
            width: 100%;
            border-collapse: collapse;
            margin: 1rem 0;
        }}
        .endpoints-table th {{
            text-align: left;
            padding: 0.5rem;
            color: #808080;
            border-bottom: 1px solid #333;
        }}
        .endpoints-table td {{
            padding: 0.5rem;
            border-bottom: 1px solid #2a2a2a;
        }}
        .endpoint-path {{
            color: #4d9375;
            font-weight: 600;
        }}
        .endpoint-method {{
            color: #569cd6;
        }}
        .endpoint-desc {{
            color: #808080;
        }}
        .status {{
            color: #ff6b6b;
            margin-bottom: 1rem;
        }}
    </style>
</head>
<body>
    <div class="terminal">
        <div class="error-code">Error: Path Not Found</div>
        <div class="status">Status: 404 Not Found</div>
        <p>The requested path does not exist:</p>
        <div class="path-box">{path}</div>
        
        <div class="divider"></div>
        
        <p>Available Endpoints:</p>
        <table class="endpoints-table">
            <thead>
                <tr>
                    <th>Method</th>
                    <th>Path</th>
                    <th>Description</th>
                    <th>Response Type</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td class="endpoint-method">GET</td>
                    <td class="endpoint-path">/version</td>
                    <td class="endpoint-desc">Server version information</td>
                    <td class="endpoint-desc">application/json</td>
                </tr>
                <tr>
                    <td class="endpoint-method">GET</td>
                    <td class="endpoint-path">/healthz</td>
                    <td class="endpoint-desc">Health check endpoint</td>
                    <td class="endpoint-desc">application/json</td>
                </tr>
                <tr>
                    <td class="endpoint-method">GET</td>
                    <td class="endpoint-path">/metrics</td>
                    <td class="endpoint-desc">Prometheus metrics</td>
                    <td class="endpoint-desc">text/plain</td>
                </tr>
            </tbody>
        </table>

        <div class="divider"></div>
        <p class="endpoint-desc">Tip: Use curl -v for detailed request/response information</p>
    </div>
</body>
</html>"#),
                "404 Not Found",
                path
            )
        },
        &_ => (
            "HTTP/1.1 400 BAD REQUEST",
            format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>400 - Bad Request</title>
    <style>
        body {{ 
            font-family: 'Courier New', monospace;
            background: #1a1a1a;
            color: #e0e0e0;
            padding: 40px;
            line-height: 1.6;
        }}
        .terminal {{
            background: #252525;
            border-radius: 6px;
            padding: 20px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
            border: 1px solid #333;
        }}
        .error-title {{
            color: #ff6b6b;
            font-size: 24px;
            margin: 0 0 20px 0;
            display: flex;
            align-items: center;
            gap: 10px;
        }}
        .error-title::before {{
            content: "✗";
            color: #ff6b6b;
        }}
    </style>
</head>
<body>
    <div class="terminal">
        <h1 class="error-title">400 - Bad Request</h1>
        <p>The request was malformed or invalid.</p>
    </div>
</body>
</html>"#),
            "400 Bad Request",
            "/unknown"
        ),
    };

    let response = format!(
        "{}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );

    stream.write_all(response.as_bytes())?;
    Ok((bytes_read, status_text.to_string(), path.to_string()))
}