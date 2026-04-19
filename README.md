# 🦀 Rust Motion Detector (ESP32-S3-Nano)
## Compatible with ESP32-S3R8

An asynchronous motion detection system built with **Rust** and **ESP-IDF**. This project uses a PIR sensor to detect movement, provides visual feedback via the onboard LED, and sends an HTTP GET request to a remote server (e.g., a Raspberry Pi) to trigger an external event.

---

## 🚀 Features
* **Real-time Detection:** High-frequency GPIO polling (100ms).
* **Wi-Fi Integration:** Robust connection logic with automatic retries and state monitoring.
* **HTTP Notifications:** Sends triggers via `esp-idf-svc` to a configurable URL.
* **Visual Debugging:** Distinct LED blink patterns for boot, Wi-Fi pairing, and errors.
* **Cooldown Logic:** Prevents "trigger spam" with a configurable 10-second window.

---

## 🛠 Hardware Setup

| Component | ESP32-S3 Pin | Configuration |
| :--- | :--- | :--- |
| **PIR Sensor** | `GPIO 5` | `Pull::Down` |
| **Status LED** | `GPIO 48` | Onboard RGB (Digital Out) |

---

## ⚙️ Configuration

Environment variables are managed via `.cargo/config.toml`. Ensure these are set before building so the compiler can embed them:

```toml
[env]
WIFI_SSID = "Your_SSID"
WIFI_PASS = "Your_Password"
PI_URL    = "[http://192.168.](http://192.168.)x.x:5000/motion"
MCU       = "esp32s3"
ESP_IDF_VERSION = "v5.5.3"
```

## 🏗 Installation & Build
###  1. Prerequisites

Ensure you have the Xtensa toolchain and linker installed:
```bash
cargo install espup
espup install
. $HOME/export-esp.sh
cargo install ldproxy
```

### 2. Compilation

To build and flash the device in one command:

```bash
cargo run --release
```

## 🚦 LED Status Codes

The device communicates its state via the LED on GPIO 48:

*   2 Fast Blinks: System Booted.

*    1 Slow Blink: Initializing Hardware/NVS.

*   3 Standard Blinks: Wi-Fi started, attempting        connection.

*    Slow Pulsing (1s): Connecting to Access Point...

*    Solid High (2s): Wi-Fi Connected.

*    Solid High (1.5s): Motion Detected & Request Sent.

*    3 Rapid Blinks: HTTP Request Failed (Check server status).

## 📂 Project Structure

-    src/main.rs: Core application logic and Wi-Fi management.

-    .cargo/config.toml: Target settings and environment variables.

-    sdkconfig.defaults: ESP-IDF framework overrides.

-    partitions.csv: Flash memory layout for the ESP32-S3.

## 📝 License

This project is licensed under the MIT License.