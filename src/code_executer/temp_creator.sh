
                        cargo new $PWD/temp_exe/codu_tmp_exeCTU45_72_5 --bin &&
                        chown -R javadyakuza:javadyakuza $PWD/temp_exe/codu_tmp_exeCTU45_72_5 &&
                        touch $PWD/temp_exe/codu_tmp_exeCTU45_72_5/src/executable.rs &&
                        echo "cargo build --manifest-path $PWD/temp_exe/codu_tmp_exeCTU45_72_5/Cargo.toml" > $PWD/temp_exe/codu_tmp_exeCTU45_72_5/bin_builder.sh &&
                        chmod +x $PWD/temp_exe/codu_tmp_exeCTU45_72_5/bin_builder.sh
                        