publish:
	cargo update
	cargo publish
	version=`grep version Cargo.toml | cut -d "\"" -f 2`
	major=`echo ${version} | cut -d "." -f 1`
	minor=`echo ${version} | cut -d "." -f 2`
	patch=`echo ${version} | cut -d "." -f 3`
	docker build -t "ghcr.io/tarneaux/shlonk:latest" .
	docker tag "ghcr.io/tarneaux/shlonk:latest" "ghcr.io/tarneaux/shlonk:${major}"
	docker tag "ghcr.io/tarneaux/shlonk:latest" "ghcr.io/tarneaux/shlonk:${major}.${minor}"
	docker tag "ghcr.io/tarneaux/shlonk:latest" "ghcr.io/tarneaux/shlonk:${major}.${minor}.${patch}"
	docker push "ghcr.io/tarneaux/shlonk:latest"
	docker push "ghcr.io/tarneaux/shlonk:${major}"
	docker push "ghcr.io/tarneaux/shlonk:${major}.${minor}"
	docker push "ghcr.io/tarneaux/shlonk:${major}.${minor}.${patch}"
