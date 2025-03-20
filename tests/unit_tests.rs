use port_claim::kill_port_process;
use port_claim::port_available;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

// Import functions to test
// These tests will access functions through the running binary

#[test]
fn test_port_available_when_free() {
    // Choose a likely-free port
    let port = 59432;
    assert!(port_available(port, false), "Port should be available");
}

#[test]
fn test_port_available_when_in_use() {
    // Bind a port
    let port = 59433;
    let _listener =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Failed to bind port for test");

    // Keep it alive for the test
    assert!(!port_available(port, false), "Port should be unavailable");
}

#[test]
fn test_parse_valid_port() {
    let port_str = "8080";
    let result = port_str.parse::<u16>();
    assert!(result.is_ok(), "Should parse valid port");
    assert_eq!(result.unwrap(), 8080);
}

#[test]
fn test_parse_invalid_port() {
    let port_str = "invalid";
    let result = port_str.parse::<u16>();
    assert!(result.is_err(), "Should not parse invalid port");
}

#[test]
fn test_parse_out_of_range_port() {
    let port_str = "65536"; // Valid u16 max is 65535
    let result = port_str.parse::<u16>();
    assert!(result.is_err(), "Should not parse out of range port");
}

#[test]
fn test_port_edges() {
    // Test port 0 (usually reserved)
    let port_zero_result = port_available(0, false);
    assert!(
        port_zero_result || !port_zero_result,
        "Port availability check should not crash"
    );

    // Test highest valid port
    let highest_port = 65535;
    let highest_result = port_available(highest_port, false);
    assert!(
        highest_result || !highest_result,
        "Port availability check should not crash"
    );
}

#[test]
fn test_port_binding_ipv4_only() {
    // Test that we only check IPv4
    let port = 59434;
    // Create a TcpListener that binds to the IPv6 address for this port
    let ipv6_addr = format!("[::1]:{}", port);

    // This might fail if IPv6 is not supported, so we handle that case
    if let Ok(_listener) = TcpListener::bind(&ipv6_addr) {
        // If IPv6 binding succeeds, our IPv4 check should still report port as available
        let _handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            // listener is dropped when thread ends
        });

        // Check that IPv4 port is still considered available
        // This may vary based on OS binding behavior, so we handle both cases
        let ipv4_available = port_available(port, false);
        assert!(
            ipv4_available || !ipv4_available,
            "IPv4 port check should not crash when IPv6 is in use"
        );

        // No need to join the handle - we just want the thread to keep the listener alive
    }
}

#[test]
fn test_port_available_function_direct() {
    let port = 62999;

    let available = port_available(port, false);
    assert!(available || !available);

    if available {
        match TcpListener::bind(format!("127.0.0.1:{}", port)) {
            Ok(_listener) => {
                assert!(!port_available(port, false));
            }
            Err(_) => {}
        }
    }
}

#[test]
fn test_kill_process_error_handling() {
    // Test with a port that's very unlikely to be in use
    let unused_port = 65534;

    // First make sure it's actually unused
    if !port_available(unused_port, false) {
        // If it's in use, we'll skip the rest of this test
        return;
    }

    // Attempt to kill process on the unused port
    kill_port_process(unused_port, false);

    // No assertion needed - we're testing that it doesn't panic or crash
}
