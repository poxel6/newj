default:
	just --list --unsorted

str:
	cargo run -- -n app -d me.poxel && eza --tree -Astype -F --icons=never app && rm -rf ./app

txt:
	cargo run -- -n app -d me.poxel && bat --style=plain --pager=never $(find . | grep App) && rm -rf ./app

help:
	cargo run -- --help
	

