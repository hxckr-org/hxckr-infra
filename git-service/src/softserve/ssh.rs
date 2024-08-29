use super::config::{
    expand_tilde, get_softserve_host, get_softserve_key_path, get_softserve_port,
    get_softserve_user,
};
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn execute_command(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let tcp = TcpStream::connect((get_softserve_host().as_str(), get_softserve_port()))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    let expanded_path = expand_tilde(&get_softserve_key_path());
    sess.userauth_pubkey_file(get_softserve_user().as_str(), None, &expanded_path, None)?;

    if !sess.authenticated() {
        return Err("Authentication failed".into());
    }

    let mut channel = sess.channel_session()?;
    channel.exec(command)?;

    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    channel.wait_close()?;
    let exit_status = channel.exit_status()?;

    if exit_status == 0 {
        Ok(output)
    } else {
        Err(format!(
            "Command failed. Exit status: {}. Output: {}",
            exit_status, output
        )
        .into())
    }
}
