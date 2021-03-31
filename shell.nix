
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
	buildInputs = with pkgs; [
		# Build Tools
		cargo
		cargo-edit
		pkg-config
		ncurses
		openssl
	];
	/* shellHook = ''
    	export PGDATA=./db/content
	''; */
}
