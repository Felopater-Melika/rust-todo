{

  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShellNoCC {
          buildInputs = [ pre-commit clang glibc_multi ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          shellHook = ''
              tmux new-session -d -s todolist
              tmux new-window -t todolist:1 'Nvim'
              tmux send-keys -t todolist:1 'nvim' C-m

              tmux new-window -t todolist:2 -n 'Shell'

              tmux new-window -t todolist -n 'Cargo Watch'
              tmux send-keys -t todolist:3 'cargo watch -x "run"' C-m

              tmux new-window -t todolist -n 'Database'
              tmux send-keys -t todolist:4 'docker-compose up' C-m

              tmux attach -t todolist
          '';
        };
      }
    );
}