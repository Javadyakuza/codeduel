
                        cargo new $PWD/temp_exe/codu_tmp_exeCTU83_81_2 --bin &&
                        chown -R javadyakuza:javadyakuza $PWD/temp_exe/codu_tmp_exeCTU83_81_2 &&
                        touch $PWD/temp_exe/codu_tmp_exeCTU83_81_2/src/executable.rs &&
                        echo "cargo build --manifest-path $PWD/temp_exe/codu_tmp_exeCTU83_81_2/Cargo.toml" > $PWD/temp_exe/codu_tmp_exeCTU83_81_2/bin_builder.sh &&
                        chmod +x $PWD/temp_exe/codu_tmp_exeCTU83_81_2/bin_builder.sh
                        