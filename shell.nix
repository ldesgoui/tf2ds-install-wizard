{ pkgs ? import <nixpkgs> {}
}:

with pkgs;

mkShell {
  buildInputs = [
    latest.rustChannels.stable.rust
    ncurses
  ];

  shellHook = ''
    dev() { cargo watch -x build -s "steam-run target/debug/tf2ds-install-wizard"; }
  '';
}
