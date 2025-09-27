push:
	cargo clean
	git add .
	git commit -m "$m"
	git push
