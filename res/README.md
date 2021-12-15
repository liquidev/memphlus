This directory houses resources that are later compiled down into the game itself.

To compile these resources to game assets, use `scripts/compile-assets.sh`.

# Some notes

- The map format the game expects is a strict subset of Tiled's features. These limitations aren't
  currently documented, so the easiest way of creating a new map is to just copy the existing one
  and avoid changing things that seem important (such as Map Properties).
