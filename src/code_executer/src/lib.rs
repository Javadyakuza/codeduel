pub mod models;
use chrono::Utc;
pub use models::CargoProjectParams;
use std::env;
use std::io::SeekFrom;
use tokio::io::AsyncSeekExt;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

const STATIC_TOML: &str = "[package]
name = \"code_executer\"
version = \"0.1.0\"
edition = \"2021\"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html


[dependencies]
chrono = \"0.4.33\"
rocket = \"0.5.0\"
serde = { version = \"1.0.196\", features = [\"derive\"] }
tokio = { version = \"1.18.2\", features = [\"full\"] }

[dependencies.rocket_contrib]
version = \"0.4.5\"
features = [\"handlebars_templates\", \"tera_templates\"]                                                              





";

pub async fn parse_init_execute(
    params: CargoProjectParams,
) -> Result<bool, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let formatted_timestamp = now.format("%Y-%m-%dT%H:%M:%S%Z");
    // preparing the project name
    let rpn = formatted_timestamp
        .to_string()
        .chars()
        .rev()
        .take(10)
        .map(|mut x: char| {
            if x == ':' {
                x = '_';
                x
            } else {
                x
            }
        })
        .collect::<String>();

    // getting the current location of the program
    let pwd = env::current_dir().unwrap(); // panic impossible

    // creating the file system options
    let mut options = tokio::fs::File::options();

    // writing the temp creator bash file commands
    match options.write(true).read(true).open("temp_creator.sh").await {
        Ok(mut file) => {
            match file
                .write_all(
                    format!(
                        "
                        cargo new $PWD/temp_exe/codu_tmp_exe{} --bin &&
                        chown -R javadyakuza:javadyakuza $PWD/temp_exe/codu_tmp_exe{} &&
                        touch $PWD/temp_exe/codu_tmp_exe{}/src/executable.rs &&
                        echo \"cargo build --manifest-path $PWD/temp_exe/codu_tmp_exe{}/Cargo.toml\" > $PWD/temp_exe/codu_tmp_exe{}/bin_builder.sh &&
                        chmod +x $PWD/temp_exe/codu_tmp_exe{}/bin_builder.sh
                        ",
                        rpn, rpn, rpn, rpn, rpn, rpn,
                    )
                    .as_bytes(),
                )
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to update the creator bash script \n {:?}", e),
                    )))
                }
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to open the creator bash script \n {:?}", e),
            )))
        }
    }

    // executing the bash script to generate the project
    let _ = match Command::new("sh")
        .arg(format!("{}/temp_creator.sh", pwd.to_str().unwrap()))
        .output()
        .await
    {
        Ok(o) => {
            println!("creator {:?}", o.stdout);
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to execute the creator bash script \n {:?}", e),
            )))
        }
    };

    // writing the purger commands on the project
    let _ = match options.write(true).read(true).open("temp_remover.sh").await {
        Ok(mut file) => {
            match file
                .write_all(
                    format!(
                        "
                        rm -rf temp_exe/codu_tmp_exe{} &&
                        rm -rf target/debug/codu_tmp_exe{} &&
                        rm -rf target/debug/codu_tmp_exe{}.d
                        ",
                        rpn, rpn, rpn,
                    )
                    .as_bytes(),
                )
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to update the remover bash script \n {:?}", e),
                    )))
                }
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to open the bash script \n {:?}", e),
            )))
        }
    };
    // writing the executable string
    match options
        .write(true)
        .read(true)
        .open(format!(
            "{}/temp_exe/codu_tmp_exe{}/src/executable.rs",
            pwd.to_str().unwrap(),
            rpn
        ))
        .await
    {
        Ok(mut file) => {
            if let Err(e) = file.write_all(params.executable.as_bytes()).await {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to update the executable file \n {:?}", e),
                )));
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to open the executable file \n {:?}", e),
            )))
        }
    }

    // writing the executer string
    match options
        .write(true)
        .read(true)
        .open(format!(
            "{}/temp_exe/codu_tmp_exe{}/src/main.rs",
            pwd.to_str().unwrap(),
            rpn
        ))
        .await
    {
        Ok(mut file) => {
            if let Err(e) = file.write_all(params.executer.as_bytes()).await {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to update the executer file \n {:?}", e),
                )));
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to open the executer file \n {:?}", e),
            )))
        }
    }

    let _ = match Command::new("sh")
        // .current_dir(format!(
        //     "{}/temp_exe/codu_tmp_exe{}/",
        //     pwd.to_str().unwrap(),
        //     rpn
        // ))
        .arg(format!(
            "{}/temp_exe/codu_tmp_exe{}/bin_builder.sh",
            pwd.to_str().unwrap(),
            rpn
        ))
        .output()
        .await
    {
        Ok(o) => {
            // println!(
            //     "raw {:?} \n stdout {:?} \n to_string {} \n",
            //     o,
            //     o.stdout,
            //     String::from_utf8_lossy(&o.stdout).to_string()
            // );
            let tmp_output = String::from_utf8_lossy(&o.stderr).to_string();
            // println!("bin runner {:?}", tmp_output);
            if tmp_output.contains("error") {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{}", tmp_output),
                )));
            } else {
                o
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to compile the program \n {:?}", e),
            )))
        }
    };

    // writing the compiled program bin executer
    let _ = match options.write(true).read(true).open("temp_runner.sh").await {
        Ok(mut file) => {
            match file
                .write_all(
                    format!("{}/target/debug/codu_tmp_exe{}", pwd.to_str().unwrap(), rpn)
                        .as_bytes(),
                )
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to update the runner bash script \n {:?}", e),
                    )))
                }
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to open the runner bash script \n {:?}", e),
            )))
        }
    };

    let _ = match Command::new("sh")
        .arg(format!("{}/temp_runner.sh", pwd.to_str().unwrap()))
        .output()
        .await
    {
        Ok(o) => {
            let tmp_output = String::from_utf8_lossy(&o.stdout).to_string();
            if tmp_output.contains("false") {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "running test cases failed",
                )));
            } else {
                o
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to compile the program \n {:?}", e),
            )))
        }
    };

    Ok(true)
}

pub async fn update_toml() -> Result<bool, Box<dyn std::error::Error>> {
    // creating the file system options and getting the current path
    let mut options = tokio::fs::File::options();
    let pwd = env::current_dir().unwrap(); // panic impossible

    // replacing the cargo.toml file with the static content
    match options
        .read(true)
        .write(true)
        .open(format!("{}/Cargo.toml", pwd.to_str().unwrap()))
        .await
    {
        Ok(mut f) => {
            f.seek(SeekFrom::Start(0)).await.unwrap(); // panic impossible

            match f.write_all(STATIC_TOML.as_bytes()).await {
                Ok(_) => {
                    // running the bash script and purging the project
                    let _ = match Command::new("sh")
                        .arg(format!("{}/temp_remover.sh", pwd.to_str().unwrap()))
                        .output()
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("couldn't purge the project \n {:?}", e),
                            )))
                        }
                    };
                    Ok(true)
                }
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("couldn't update the main toml file \n {:?}", e),
                    )))
                }
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("couldn't delete the workspace \n {:?}", e),
            )))
        }
    }
}
