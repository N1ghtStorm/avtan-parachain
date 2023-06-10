# Avtan Rooster Parachain Ko Ko Ko 

                        ~-.
                        ,,,;            ~-.~-.~-
                    (.../           ~-.~-.~-.~-.~-.
                < } O~`, ,        ~-.~-.~-.~-.~-.~-.
                    (/    T ,     ~-.~-.~-.~-.~-.~-.~-.
                        ;    T     ~-.~-.~-.~-.~-.~-.~-.
                      ;   {_.~-.~-.~-.~-.~-.~-.~
                    ;:  .-~`    ~-.~-.~-.~-.~-.
                    ;.: :'    ._   ~-.~-.~-.~-.~-
                    ;::`-.    '-._  ~-.~-.~-.~-
                    ;::. `-.    '-,~-.~-.~-.
                        ';::::.`''-.-'
                        ';::;;:,:'
                            '||T
                            / |
                          __   _

# Commands:

./target/release/avtan-node build-spec --disable-default-bootnode > rococo-local-parachain-plain.json
./target/release/avtan-node build-spec --chain rococo-local-parachain-plain.json --raw --disable-default-bootnode > rococo-local-parachain-2666-raw.json

./target/release/avtan-node  export-genesis-wasm --chain rococo-local-parachain-2666-raw.json > para-2666-wasm
./target/release/avtan-node  export-genesis-state --chain rococo-local-parachain-2666-raw.json > para-2666-genesis