
                        cargo new $PWD/temp_exe/codu_tmp_exeCTU05_10_6 --bin &&
                        chown -R javadyakuza:javadyakuza $PWD/temp_exe/codu_tmp_exeCTU05_10_6 &&
                        touch $PWD/temp_exe/codu_tmp_exeCTU05_10_6/src/executable.rs &&
                        echo "cargo build --manifest-path $PWD/temp_exe/codu_tmp_exeCTU05_10_6/Cargo.toml" > $PWD/temp_exe/codu_tmp_exeCTU05_10_6/bin_builder.sh &&
                        chmod +x $PWD/temp_exe/codu_tmp_exeCTU05_10_6/bin_builder.sh
                        