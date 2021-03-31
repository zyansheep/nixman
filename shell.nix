
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
	buildInputs = with pkgs; [
		# Build Tools
		cargo
		cargo-edit
		pkg-config
		ncurses
	];
	/* shellHook = ''
    	export PGDATA=./db/content
	''; */
}
