_default:
  just --list

docker:
  nix build .#docker-image
  ./result | docker load

deploy:
  just docker
  fly deploy --local-only
