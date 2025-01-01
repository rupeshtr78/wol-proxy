# Remote Wake-on-LAN (WoL) Proxy Server

Application to remotely wake up your home server using the Wake-on-LAN (WoL) protocol. The application acts as a proxy that listens for HTTP requests and sends the wol magic packet to wake up your server. Wrote this to remote wake my plex music server while I am on the road. (Need portforwarding on your router)

Mobile --> Router --> Wol Proxy --> Server

## Features

- **Wake-on-LAN Magic Packet Generation**: The application generates and sends a magic packet to the specified MAC address of your server.
- **HTTP API**: Provides an HTTP endpoint to trigger the WoL request.
- **Cookie Verification**: Ensures that only authorized requests can trigger the WoL functionality.
- **Configurable Bind and Broadcast Addresses**: Allows you to specify the bind and broadcast addresses for sending the magic packet.
- **Test Suite**: Includes unit tests ( Need more work here ) .

## Prerequisites

- Rust (latest stable version)
- A proxy server i am using pie 4.
- MAC address for your music server

## Installation

1. **Clone the Repository**:
   ```sh
   git clone https://github.com/your-username/remote-wol-proxy.git
   cd remote-wol-proxy
   ```

2. **Build the Application**:
   ```sh
   cargo build --release
   For cross build check arm7-build.sh
   ```

3. **Configure the Application**:
   - Ensure that the `Cargo.toml` file is correctly configured.
   - Modify the `scripts/cookie.sh` script to set the appropriate cookie value for verification.

4. **Run the Application**:
   ```sh
   cargo run --release
   ```

## Usage

### Sending a Wake-on-LAN Request

You can send a Wake-on-LAN request by making an HTTP POST request to the `/wol` endpoint with the following JSON payload:

```json
{
    "mac_address": "00:11:22:33:44:55",
    "bind_addr": "192.168.1.100",  // optional
    "broadcast_addr": "192.168.1.255" // optional
}
```

### Example cURL Command

```sh
curl -X POST http://localhost:8080/wol \
    -H "Content-Type: application/json" \
    -d '{"mac_address": "00:11:22:33:44:55"}'
    -b "cookie=your-cookie-value"
```

### Cookie Verification

The application verifies the presence and validity of a cookie in the HTTP request to ensure that only authorized requests can trigger the WoL functionality. The cookie value is expected to be set in the `scripts/cookie.sh` script. ( Needs more work here. Kind of a hack at the moment ).

## Scripts

- **arm7-build.sh**: Script to build the application for ARMv7 architecture.
- **cookie.sh**: Script to set the cookie value for verification.
- **create-service.sh**: Script to create a systemd service for the application.
- **curl.sh**: Example cURL command to send a WoL request.
- **wol-proxy.service**: Systemd service file for the application.

## Testing

 You can run the tests using the following command: ( Need more work here )

```sh
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you encounter any problems or have suggestions for improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgments

- Thanks to the Rust community for providing excellent resources and libraries.
- Inspired by the need to remotely wake up my music server without direct access to the local network.

## Contact

For any questions or feedback, please reach out to [rupeshtr@gmail.com](mailto:your-email@example.com).

---

Enjoy waking up your music server remotely with ease! 🎶
