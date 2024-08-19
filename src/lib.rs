

#[cfg(test)]
mod test_sftp {
    use std::io::{Read, Write};

    #[tokio::test]
    async fn test_sftp_connect() {
        use std::net::TcpStream;
        use ssh2::Session;
        use std::path::Path;
        use std::fs::File;
        use indicatif::{ProgressBar, ProgressStyle};
        use std::env;

        let host: String = "127.0.0.1".to_string();
        let port: u16 = 2222;
        let username: String  = "test".to_string();
        let password: String = "test123".to_string();
        let tcp = TcpStream::connect(format!("{}:{}",host,port)).unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        session.userauth_password(username.as_str(), password.as_str()).unwrap();
        session.authenticated();
        let sftp = session.sftp().unwrap();        
        for (path,file_stat) in sftp.readdir(Path::new("/upload")).unwrap() {


            let file_path = path;
            if file_stat.is_file() {
                let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
                let file_size = file_stat.size.unwrap();
                let dest_path = format!("{}/storage/download/{}",env::current_dir().unwrap().to_str().unwrap().to_string(),file_name);
                let mut sftp_file = sftp.open(Path::new(&file_path)).unwrap();
                let mut file = File::options().append(false).write(true).read(true).create(true).open(dest_path).unwrap();
                let bar = ProgressBar::new(file_size);
                bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                    .unwrap()
                    .progress_chars("#>-"));
                println!("Downloading {}",file_path.as_path().to_str().unwrap().to_string());
                loop {
                    let mut buf: [u8; 1024] = [0;1024];
                    let total: usize = sftp_file.read(&mut buf).unwrap();
                    if total == 0 {
                        break;
                    }
                    file.write_all(&mut String::from_utf8_lossy(&buf).trim_end_matches(char::from(0)).as_bytes()).unwrap();
                    bar.inc(total as u64);
                }
                bar.finish();
            }
        }

    }
}