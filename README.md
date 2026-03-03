# cmus_cover_art
Tool for displaying album art for cmus audio player.

# Credits
Heavily inspired by https://github.com/nogizhopaboroda/cmus-cover-art

# Preview
![alt text](https://raw.githubusercontent.com/Vizkid04/cmus_cover_art/master/Screenshot_cmus2.png)

![alt text](https://raw.githubusercontent.com/Vizkid04/cmus_cover_art/master/Screenshot_cmus.png)

# Dependencies
- Kitty terminal emulator. May be avoided by modifying the code at src/display.rs which calls "kitten icat" and instead using an alternative terminal image rendering utility like imgcat.
- Rust (cargo)
- Magick

# Instructions
- `cd /home/$USER/.config/cmus`
- `git clone https://github.com/Vizkid04/cmus_cover_art.git`
- `cd cmus-cover-art`
- `cargo build --release`
- `chmod +x target/release/observe`
- `chmod +x target/release/display`
- Open cmus and run `:set status_display_program=/home/$USER/.config/cmus/cmus-cover-art/target/release/observe`
- Run `/home/$USER/.config/cmus/cmus-cover-art/target/release/display` to display the album art.

# Recommended
- The visualizer I am using is cava. I achieve the terminal sections (multiplexing) using tmux.
