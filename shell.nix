let
  sources = import ./nix/sources.nix;
  overlay = _: pkgs: { niv = import sources.niv {}; };
  nixpkgs = import sources.nixpkgs { overlays = [overlay]; config = {}; };
in
nixpkgs.mkShell {
  buildInputs = with nixpkgs; [
    rustc cargo binutils gcc gnumake openssl pkgconfig
  ];
}
