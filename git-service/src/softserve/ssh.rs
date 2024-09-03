use super::config::{
    expand_tilde, get_softserve_host, get_softserve_key_path, get_softserve_port,
    get_softserve_user,
};
use super::errors::GitServiceError;
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

pub fn execute_command(command: &str) -> Result<String, GitServiceError> {
    let tcp = TcpStream::connect((get_softserve_host().as_str(), get_softserve_port()))
        .map_err(|e| GitServiceError::ConnectionError(e.to_string()))?;
    let mut sess = Session::new().map_err(|e| GitServiceError::SessionError(e.to_string()))?;
    sess.set_tcp_stream(tcp);
    sess.handshake()
        .map_err(|e| GitServiceError::HandshakeError(e.to_string()))?;

    let expanded_path = expand_tilde(&get_softserve_key_path());
    sess.userauth_pubkey_file(get_softserve_user().as_str(), None, &expanded_path, None)
        .map_err(|e| GitServiceError::AuthenticationError(e.to_string()))?;

    if !sess.authenticated() {
        return Err(GitServiceError::AuthenticationError(
            "Authentication failed".to_string(),
        ));
    }

    let mut channel = sess
        .channel_session()
        .map_err(|e| GitServiceError::ChannelError(e.to_string()))?;
    channel
        .exec(command)
        .map_err(|e| GitServiceError::CommandExecutionError(e.to_string()))?;

    let mut output = String::new();
    channel
        .read_to_string(&mut output)
        .map_err(|e| GitServiceError::OutputReadError(e.to_string()))?;
    channel
        .wait_close()
        .map_err(|e| GitServiceError::ChannelCloseError(e.to_string()))?;
    let exit_status = channel
        .exit_status()
        .map_err(|e| GitServiceError::ExitStatusError(e.to_string()))?;

    if exit_status == 0 {
        Ok(output)
    } else {
        Err(GitServiceError::CommandFailedError(exit_status, output))
    }
}
