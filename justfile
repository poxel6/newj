default:
	just --list --unsorted

str:
	cargo run -- -n app -d org.example && eza --tree -Astype -F --icons=never app && rm -rf ./app

txt:
	cargo run -- -n app -d org.example && bat --style=plain --pager=never $(find . | grep App) && rm -rf ./app

help:
	cargo run -- --help
	

