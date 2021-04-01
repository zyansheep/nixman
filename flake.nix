{
  	inputs = {
		utils.url = "github:numtide/flake-utils";
		naersk.url = "github:nmattia/naersk";
		fenix.url = "github:nix-community/fenix";
  	};

  	outputs = { self, nixpkgs, utils, naersk, fenix }:
	utils.lib.eachDefaultSystem (system: let
		pkgs = nixpkgs.legacyPackages."${system}";
		# Specify Rust Toolchain
		# Use Stable (Default)
		# naersk-lib = naersk.lib."${system}";
		# Use Nightly (provided by fenix)
		naersk-lib = naersk.lib."${system}".override {
			inherit (fenix.packages.${system}.minimal) cargo rustc;
		};
	in rec {
		# `nix build`
		packages.nixman = naersk-lib.buildPackage {
			pname = "nixman";
			root = ./.;
			nativeBuildInputs = with pkgs; [
				pkg-config
			];
			buildInputs = with pkgs; [
				ncurses
				openssl
			];
		};
		defaultPackage = packages.nixman;

		# `nix run`
		apps.nixman = utils.lib.mkApp {
			drv = packages.nixman;
		};
		defaultApp = apps.nixman;

		# `nix develop`
		devShell = pkgs.mkShell {
			nativeBuildInputs = packages.nixman.nativeBuildInputs;
			buildInputs = packages.nixman.buildInputs;
		};
	});
}