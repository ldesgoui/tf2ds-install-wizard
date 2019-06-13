{ pkgs ? import <nixpkgs> {}
}:

with pkgs;

mkShell {
  buildInputs = [
    ncurses
  ];

  shellHook = ''
    W() { cargo watch -x build -s "steam-run target/debug/srcdsds"; }
  '';
}
