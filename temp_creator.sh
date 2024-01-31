
                        cargo new $PWD/code_executer/temp_exe/codu_tmp_exeCTU03_85_1 --bin &&
                        chown -R javadyakuza:javadyakuza $PWD/code_executer/temp_exe/codu_tmp_exeCTU03_85_1 &&
                        touch $PWD/code_executer/temp_exe/codu_tmp_exeCTU03_85_1/src/executable.rs &&
                        echo "cargo build --manifest-path $PWD/code_executer/temp_exe/codu_tmp_exeCTU03_85_1/Cargo.toml" > $PWD/temp_exe/codu_tmp_exeCTU03_85_1/bin_builder.sh &&
                        chmod +x $PWD/code_executer/temp_exe/codu_tmp_exeCTU03_85_1/bin_builder.sh
                        